use std::{error, fmt};

use crate::token::Token;

#[derive(Debug)]
pub struct RuntimeError {
    pub kind: ErrorKind,
}

impl error::Error for RuntimeError {}

impl RuntimeError {
    pub fn new(kind: ErrorKind) -> RuntimeError {
        RuntimeError { kind }
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ErrorKind::UndefinedVariable(t) => write!(f, "undefined variable `{}`", t.lexeme),
        }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    UndefinedVariable(Token),
}
