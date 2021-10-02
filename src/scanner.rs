use super::token::{Token, TokenType};
struct Scanner {
    src: String,
    tokens: Vec<Token>,
    /// First charcter in the lexeme being scanned
    start: u32,
    /// The character considered
    current: u32,
    /// What src line we're on
    line: u32,
}
impl Scanner {
    fn new(src: String) -> Self {
        let tokens = Vec::new();
        Self {
            src,
            tokens,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.src.len() as u32
    }

    fn scan_tokens(&mut self) {
        while !self.is_at_end() {}

        self.tokens
            .push(Token::new(TokenType::Eof, String::new(), None, self.line))
    }

    fn scan_token(&mut self) {}
}
