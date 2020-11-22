use crate::vfs::Vfs;
use itertools::Itertools;
use leek_parser::STMT;
use std::borrow::Cow;
use std::cell::RefCell;
use std::sync::Arc;
use tokio::macros::support::Future;
use tokio::sync::{Mutex, RwLock};
use tower_lsp::jsonrpc::Result as RPCResult;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

mod doc;
mod vfs;

#[derive(Clone)]
struct Backend {
    client: Client,
    vfs: Arc<RwLock<Vfs>>,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> RPCResult<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::Incremental)),
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

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let mut vfs = self.vfs.write().await;
        for change in params.content_changes {
            vfs.update(&params.text_document.uri, change);
        }

        let doc = vfs.open_uri(&params.text_document.uri).await;
        let ast = leek_parser::parse(doc);
        let diags = match ast {
            Ok(ast) => ast
                .iter()
                .flat_map(|s| get_variables(s).into_iter())
                .map(|(span, v)| Diagnostic {
                    source: Some("leekscript".to_string()),
                    message: format!("Found variable '{}'", v),
                    range: Range {
                        start: doc.lookup_pos(span.start).unwrap(),
                        end: doc.lookup_pos(span.end).unwrap(),
                    },
                    severity: Some(DiagnosticSeverity::Information),
                    ..Default::default()
                })
                .collect(),
            Err(err) => {
                let pos = doc.lookup_pos(err.location.offset).unwrap();
                vec![Diagnostic {
                    source: Some("leekscript".to_string()),
                    message: err.to_string(),
                    range: Range {
                        start: pos,
                        end: pos,
                    },
                    severity: Some(DiagnosticSeverity::Error),
                    ..Default::default()
                }]
            }
        };

        self.client
            .publish_diagnostics(
                params.text_document.uri,
                diags,
                params.text_document.version,
            )
            .await;
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
        vfs: Arc::new(RwLock::new(Vfs::new())),
    });
    Server::new(stdin, stdout)
        .interleave(messages)
        .serve(service)
        .await;
}
