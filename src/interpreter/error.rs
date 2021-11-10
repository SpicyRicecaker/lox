use std::{error, fmt};

use super::Object;

#[derive(Debug)]
pub struct InterpreterError {
    pub kind: ErrorKind,
}

impl error::Error for InterpreterError {}

impl InterpreterError {
    pub fn new(kind: ErrorKind) -> InterpreterError {
        InterpreterError { kind }
    }
}

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ErrorKind::FailedCast(t, o) => {
                write!(f, "failed to cast {} to {}", t, o)
            }
            ErrorKind::DivideByZero(n) => write!(f, "attempt to divide {} by 0", n),
            ErrorKind::UnitializedVariable => {
                write!(f, "unitialized variable (too lazy to write name lol")
            }
        }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    FailedCast(Object, Object),
    DivideByZero(f32),
    UnitializedVariable,
}
