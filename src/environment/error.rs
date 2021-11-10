use std::{error, fmt};

use crate::token::Token;

#[derive(Debug)]
pub struct EnvironmentError {
    pub kind: ErrorKind,
}

impl error::Error for EnvironmentError {}

impl EnvironmentError {
    pub fn new(kind: ErrorKind) -> EnvironmentError {
        EnvironmentError { kind }
    }
}

impl fmt::Display for EnvironmentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ErrorKind::UndefinedVariable(t, s) => write!(f, "undefined variable `{:#?}` duing {}", t, s),
        }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    UndefinedVariable(Token, String),
}

pub fn env_error(token: &Token, &str)