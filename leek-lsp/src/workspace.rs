use std::collections::HashMap;

use tower_lsp::lsp_types::{Position, TextDocumentContentChangeEvent, Url};

use leek_parser::STMT;

use crate::doc::{TextDocument, Document};
use std::ops::Deref;

#[derive(Debug, Default)]
pub struct Workspace {
    data: HashMap<Url, Document>,
}

impl Deref for Workspace {
    type Target = HashMap<Url, Document>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl Workspace {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, url: Url, data: String) {
        self.data.insert(url, Document::from(data));
    }

    pub fn get_doc(&self, url: &Url) -> Option<&TextDocument> {
        self.get(url).map(|doc| &doc.doc)
    }

    pub fn get_ast(&self, url: &Url) -> Option<&crate::error::Result<Vec<STMT>>> {
        self.get(url).map(|doc| &doc.ast)
    }

    pub fn update_doc(
        &mut self,
        url: &Url,
        change: TextDocumentContentChangeEvent,
    ) {
        if let Some(doc) = self.data.get_mut(url) {
            doc.update(change);
        }
    }
}
