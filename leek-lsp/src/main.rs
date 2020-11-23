use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;

use itertools::Itertools;
use tokio::fs::File;
use tokio::macros::support::Future;
use tokio::prelude::io::AsyncReadExt;
use tokio::sync::{Mutex, RwLock};
use tower_lsp::jsonrpc::Result as RPCResult;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

use leek_parser::STMT;

use crate::doc::{Document, TextDocument};
use crate::vfs::Vfs;
use crate::workspace::Workspace;

mod doc;
mod error;
mod vfs;
mod workspace;

macro_rules! log {
    ($client:expr, $value:expr) => {
        log!($client, $value, return ())
    };
    ($client:expr, $value:expr, return $return:expr) => {
        match $value {
            Ok(val) => val,
            Err(err) => {
                $client
                    .log_message(MessageType::Error, err.to_string())
                    .await;
                return $return;
            }
        }
    };
}

#[derive(Clone)]
struct Backend {
    client: Client,
    workspace: Arc<RwLock<Workspace>>,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> RPCResult<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::Full,
                )),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                completion_provider: Some(CompletionOptions {
                    trigger_characters: Some(vec![" ".to_string(), "=".to_string()]),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::Info, "server initialized!")
            .await;
    }

    async fn shutdown(&self) -> RPCResult<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let mut ws = self.workspace.write().await;
        let mut file = log!(
            self.client,
            File::open(params.text_document.uri.path()).await
        );
        let mut data = String::new();
        file.read_to_string(&mut data).await;
        ws.add(params.text_document.uri, data);
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let mut ws = self.workspace.write().await;
        for change in params.content_changes {
            ws.update_doc(&params.text_document.uri, change);
        }
        std::mem::drop(ws);

        let ws = self.workspace.read().await;
        let diags = match ws.get_ast(&params.text_document.uri) {
            Some(Err(crate::error::Error::ParseError(pos, err))) => vec![Diagnostic {
                source: Some("leekscript".to_string()),
                message: err.clone(),
                severity: Some(DiagnosticSeverity::Error),
                range: Range {
                    start: *pos,
                    end: *pos,
                },
                ..Default::default()
            }],
            Some(Err(err)) => vec![Diagnostic {
                severity: Some(DiagnosticSeverity::Error),
                source: Some("leekscript".to_string()),
                message: err.to_string(),
                ..Default::default()
            }],
            _ => vec![],
        };

        self.client
            .publish_diagnostics(
                params.text_document.uri,
                diags,
                params.text_document.version,
            )
            .await;
    }

    async fn completion(&self, params: CompletionParams) -> RPCResult<Option<CompletionResponse>> {
        let ws = self.workspace.read().await;
        if let Some(doc) = ws.get(&params.text_document_position.text_document.uri) {
            Ok(Some(CompletionResponse::Array(
                doc.symbols
                    .iter()
                    .map(|s| CompletionItem {
                        kind: Some(map_kind(s.kind)),
                        label: s.name.clone(),
                        ..Default::default()
                    })
                    .collect(),
            )))
        } else {
            self.client
                .log_message(
                    MessageType::Error,
                    (crate::error::Error::NonExistantDocument(
                        params.text_document_position.text_document.uri,
                    ))
                    .to_string(),
                )
                .await;
            Ok(None)
        }
    }

    async fn hover(&self, params: HoverParams) -> RPCResult<Option<Hover>> {
        self.client
            .log_message(
                MessageType::Info,
                format!(
                    "[hover] pos={:?}",
                    params.text_document_position_params.position
                ),
            )
            .await;
        let mut ws = self.workspace.read().await;
        let data_res = ws
            .get(&params.text_document_position_params.text_document.uri)
            .ok_or_else({
                let url = params
                    .text_document_position_params
                    .text_document
                    .uri
                    .clone();
                || crate::error::Error::NonExistantDocument(url)
            });
        let doc: &Document = log!(self.client, data_res, return Ok(None));

        let offset = doc
            .doc
            .lookup_offset(params.text_document_position_params.position);
        let offset = if let Some(offset) = offset {
            offset
        } else {
            return Ok(None);
        };

        for node in doc.ast.iter().flatten() {
            if node.get_range().contains(&offset) {
                let node_at_pos = node.get_innermost(offset);
                let range = node_at_pos
                    .clone()
                    .map(|e| e.get_range(), |s| s.get_range())
                    .and_then(|range| {
                        doc.doc.lookup_pos(range.start).and_then(|s| {
                            doc.doc
                                .lookup_pos(range.end)
                                .map(|e| Range { start: s, end: e })
                        })
                    });
                let res = Hover {
                    contents: HoverContents::Scalar(MarkedString::LanguageString(LanguageString {
                        language: "leekscript".to_string(),
                        value: node_at_pos
                            .map(|e| format!("{:?}", e), |s| format!("{:?}", s))
                            .unwrap_or_else(|| "Not found".to_string()),
                    })),
                    range,
                };
                eprintln!("result: {:#?}", res);
                return Ok(Some(res));
            }
        }

        Ok(None)
    }
}

fn map_kind(kind: SymbolKind) -> CompletionItemKind {
    match kind {
        SymbolKind::File => CompletionItemKind::File,
        SymbolKind::Module => CompletionItemKind::Module,
        SymbolKind::Namespace => CompletionItemKind::Module,
        SymbolKind::Package => CompletionItemKind::Module,
        SymbolKind::Class => CompletionItemKind::Class,
        SymbolKind::Method => CompletionItemKind::Method,
        SymbolKind::Property => CompletionItemKind::Property,
        SymbolKind::Field => CompletionItemKind::Field,
        SymbolKind::Constructor => CompletionItemKind::Constructor,
        SymbolKind::Enum => CompletionItemKind::Enum,
        SymbolKind::Interface => CompletionItemKind::Interface,
        SymbolKind::Function => CompletionItemKind::Function,
        SymbolKind::Variable => CompletionItemKind::Variable,
        SymbolKind::Constant => CompletionItemKind::Constant,
        SymbolKind::String => CompletionItemKind::Value,
        SymbolKind::Number => CompletionItemKind::Value,
        SymbolKind::Boolean => CompletionItemKind::Value,
        SymbolKind::Array => CompletionItemKind::Value,
        SymbolKind::Object => CompletionItemKind::Value,
        SymbolKind::Key => CompletionItemKind::Field,
        SymbolKind::Null => CompletionItemKind::Value,
        SymbolKind::EnumMember => CompletionItemKind::EnumMember,
        SymbolKind::Struct => CompletionItemKind::Struct,
        SymbolKind::Event => CompletionItemKind::Event,
        SymbolKind::Operator => CompletionItemKind::Operator,
        SymbolKind::TypeParameter => CompletionItemKind::TypeParameter,
        SymbolKind::Unknown => CompletionItemKind::Text,
    }
}

fn get_variables(stmt: &STMT) -> Vec<(std::ops::Range<usize>, Cow<str>)> {
    match stmt {
        STMT::Declr(r, n, _) => vec![(r.clone(), Cow::Borrowed(n))],
        STMT::If(_, _, c, nc) => c
            .iter()
            .flat_map(|s| get_variables(s).into_iter())
            .chain(
                nc.iter()
                    .flatten()
                    .flat_map(|s| get_variables(s).into_iter()),
            )
            .collect(),
        STMT::While(_, _, b) => b
            .iter()
            .flat_map(|s| get_variables(s).into_iter())
            .collect(),
        STMT::Defun(_, _, _, b) => b
            .iter()
            .flat_map(|s| get_variables(s).into_iter())
            .collect(),
        _ => vec![],
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, messages) = LspService::new(|client| Backend {
        client,
        workspace: Arc::new(RwLock::new(Workspace::new())),
    });
    Server::new(stdin, stdout)
        .interleave(messages)
        .serve(service)
        .await;
}
