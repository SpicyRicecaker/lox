/// Heavy inspirations from ripgrep's error handling: https://github.com/BurntSushi/ripgrep/blob/master/crates/regex/src/error.rs
use crate::token::Token;
use std::{error, fmt};

#[derive(Debug)]
pub struct ParseError {
    pub kind: ParseErrorKind,
}

impl error::Error for ParseError {}

impl ParseError {
    pub fn new(kind: ParseErrorKind) -> ParseError {
        ParseError { kind }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ParseErrorKind::Error(exp, fnd, str) => {
                write!(f, "expected `Token {:?}`, found {:?}, {}", exp.token_type, fnd, str)
            }
            ParseErrorKind::ExpectLeftOperand(t) => write!(
                f,
                "missing left operand for {:?}({}) in line {}",
                t.token_type, t.lexeme, t.line
            ),
        }
    }
}

#[derive(Debug)]
pub enum ParseErrorKind {
    Error(Token, Token, String),
    ExpectLeftOperand(Token),
}
