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
            ErrorKind::FailedCast => {
                write!(f, "failed cast lol")
            }
            ErrorKind::DivideByZero(n) => write!(f, "attempt to divide {} by 0", n),
        }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    FailedCast,
    DivideByZero(f32)
}
