use crate::token::{Literal, Token, TokenType};
struct Scanner {
    chars: Vec<char>,
    tokens: Vec<Token>,
    /// First charcter in the lexeme being scanned
    start: usize,
    /// The character considered
    current: usize,
    /// What src line we're on
    line: usize,
}
impl Scanner {
    fn new(src: String) -> Self {
        let chars = src.chars().collect::<Vec<char>>();
        Self {
            chars,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.chars.len()
    }

    fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::Eof, String::new(), None, self.line))
    }

    fn advance(&mut self) -> &char {
        let char = &self.chars[self.current];
        self.current += 1;
        char
    }

    fn add_token_literal(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let text = self.chars[self.start..self.current]
            .iter()
            .cloned()
            .collect::<String>();
        self.tokens
            .push(Token::new(token_type, text, literal, self.line));
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_literal(token_type, None);
    }

    fn scan_token(&mut self) {
        match self.advance() {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            _ => {
                panic!()
            }
        }
    }
}
