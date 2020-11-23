use std::fmt;
use std::ops::Range;

use tower_lsp::lsp_types::Position;
use url::Url;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    ParseError(Position, String),
    IoError(std::io::Error),
    NonExistantDocument(Url),
    UndefinedSymbol(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ParseError(r, s) => write!(f, "{}:{}: {}", r.line + 1, r.character + 1, s),
            Self::IoError(err) => write!(f, "IO error: {}", err),
            Self::NonExistantDocument(url) => write!(f, "No such document at {}", url),
            Self::UndefinedSymbol(sym) => write!(f, "Undefined symbol: {}", sym),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        if let Self::IoError(err) = self {
            Some(err.clone())
        } else {
            None
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}
