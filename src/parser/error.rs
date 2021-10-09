/// Heavy inspirations from ripgrep's error handling: https://github.com/BurntSushi/ripgrep/blob/master/crates/regex/src/error.rs

use crate::token::Token;
use std::{error, fmt};

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
}

impl error::Error for Error { 
    fn description(&self) -> &str {
        match self.kind {
            
        }
    }
}

impl Error {
    pub fn new(kind: ErrorKind) -> Error {
        Error { kind }
    } }

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {

        }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    Default(Token)
}

