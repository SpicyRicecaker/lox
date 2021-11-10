/// Heavy inspirations from ripgrep's error handling: https://github.com/BurntSushi/ripgrep/blob/master/crates/regex/src/error.rs
use crate::token::Token;
use std::{error, fmt};

#[derive(Debug)]
pub struct ParseError {
    pub kind: ErrorKind,
}

impl error::Error for ParseError {}

impl ParseError {
    pub fn new(kind: ErrorKind) -> ParseError {
        ParseError { kind }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ErrorKind::Error(exp, fnd, str) => {
                write!(f, "expected {:?}, found {:?}, during {}", exp, fnd, str)
            }
        }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    Error(Token, Token, String),
}
