/// Heavy inspirations from ripgrep's error handling: https://github.com/BurntSushi/ripgrep/blob/master/crates/regex/src/error.rs
use crate::token::Token;
use std::{error, fmt};

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
}

impl error::Error for Error {}

impl Error {
    pub fn new(kind: ErrorKind) -> Error {
        Error { kind }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ErrorKind::UnmatchedParen(t) => write!(f, "expected ')' after expression"),
            ErrorKind::ExpectExpression => write!(f, "expected expression"),
        }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    UnmatchedParen(Token),
    ExpectExpression,
}
