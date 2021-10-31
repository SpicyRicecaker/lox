use std::fmt::Display;

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}

// impl std::error::Error for Error { }
impl Error {
    pub fn new(kind: ErrorKind) -> Self {
        Self { kind }
    }
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ErrorKind::UnterminatedComment(p) => write!(
                f,
                "Unterminated multi-line comment. Start begins at line {}, char {}",
                p.line, p.char
            ),
            ErrorKind::UnterminatedString(p) => write!(
                f,
                "Unterminated string. Quote begins at line {}, char {}",
                p.line, p.char
            ),
            ErrorKind::UnexpectedCharacter(p) => write!(
                f,
                "Unexpected character at line {}, char {}",
                p.line, p.char
            ),
        }
    }
}

#[derive(Debug)]
pub struct Position {
    line: usize,
    char: usize,
}

impl Position {
    pub fn new(line: usize, char: usize) -> Self {
        Position { line, char }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    UnterminatedComment(Position),
    UnterminatedString(Position),
    UnexpectedCharacter(Position),
}
