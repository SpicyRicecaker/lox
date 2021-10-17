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
            ErrorKind::UnmatchedParen(_) => write!(f, "expected ')' after expression"),
            ErrorKind::ExpectExpression(t) => {
                write!(
                    f,
                    "unexpected expression `{:?} {}` at line {}",
                    t.token_type, t.literal, t.line
                )
            }

            ErrorKind::ExpectLeftOperand(t) => {
                write!(
                    f,
                    "missing left operand for `{:?} {}`",
                    t.token_type, t.lexeme
                )
            }
            ErrorKind::ExpectSemicolon => write!(f, "missing semicolon"),
            ErrorKind::ExpectVariableName => write!(f, "expect variable name"),
        }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    UnmatchedParen(Token),
    ExpectExpression(Token),
    ExpectLeftOperand(Token),
    ExpectVariableName,
    ExpectSemicolon,
}
