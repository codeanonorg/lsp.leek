use crate::doc::TextDocument;
use itertools::Itertools;
use std::collections::HashMap;
use std::path::Path;
use std::str::FromStr;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::stream::StreamExt;
use tower_lsp::lsp_types::TextDocumentContentChangeEvent;
use url::Url;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Vfs {
    opened_documents: HashMap<Url, TextDocument>,
}

impl Vfs {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn open<P: AsRef<Path>>(&mut self, path: P) -> &TextDocument {
        let path = path.as_ref().to_string_lossy();
        let url = Url::from_str(&path).unwrap();
        self.open_uri(&url).await
    }

    pub async fn open_uri(&mut self, url: &Url) -> &TextDocument {
        if self.opened_documents.contains_key(&url) {
            &self.opened_documents[url]
        } else {
            let mut str = String::new();
            let mut file = File::open(url.path()).await.unwrap();
            file.read_to_string(&mut str).await.unwrap();
            let doc = TextDocument::from(str);
            self.opened_documents.insert(url.clone(), doc);
            &self.opened_documents[url]
        }
    }

    pub fn update(&mut self, url: &Url, change: TextDocumentContentChangeEvent) {
        if let Some(doc) = self.opened_documents.get_mut(url) {
            doc.update(change);
        }
    }
}
