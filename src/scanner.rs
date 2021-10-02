use crate::token::{Literal, Token, TokenType};
use crate::Lox;
pub struct Scanner {
    chars: Vec<char>,
    pub tokens: Vec<Token>,
    /// First charcter in the lexeme being scanned
    start: usize,
    /// The character considered
    current: usize,
    /// What src line we're on
    line: usize,
}
impl Scanner {
    pub fn new(src: String) -> Self {
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

    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(
            TokenType::Eof,
            String::new(),
            Literal::None,
            self.line,
        ))
    }

    fn advance(&mut self) -> &char {
        let char = &self.chars[self.current];
        self.current += 1;
        char
    }

    fn add_token_literal(&mut self, token_type: TokenType, literal: Literal) {
        let text = self.chars[self.start..self.current]
            .iter()
            .cloned()
            .collect::<String>();
        self.tokens
            .push(Token::new(token_type, text, literal, self.line));
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_literal(token_type, Literal::None);
    }

    fn scan_token(&mut self) {
        match self.advance() {
            // fully single characters
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
            // possible doubled chars
            // looks really ugly and we could combine them but I can't think of a way not to use doubled match statements
            '!' => {
                let res = if self.next_is('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(res);
            }
            '=' => {
                let res = if self.next_is('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(res);
            }
            '<' => {
                let res = if self.next_is('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(res);
            }
            '>' => {
                let res = if self.next_is('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(res);
            }
            // special character, could be divide, but also could be a comment
            '/' => {
                // if comment
                if self.next_is('/') {
                    // comment until end of line
                    // why not just use next is you ask? well next is always consumes, i thought conditionals were short circuiting but whatever
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                };
            }
            // any white space
            w if w.is_whitespace() => {}
            // newline
            '\n' => self.line += 1,
            // string literals
            '"' => {
                let start_line = self.line;
                while self.peek() != '"' && !self.is_at_end() {
                    // newlines inside "" don't count
                    if self.peek() == '\n' {
                        self.line += 1;
                    }
                    // remember, advance only changes current, not start
                    self.advance();
                }
                // check if string terminates at end of file w/o closing
                if self.is_at_end() {
                    Lox::error(
                        self.line as u32,
                        &format!(
                            "Unterminated string. Quote begins at line {}, char {}",
                            start_line, self.start
                        ),
                    )
                }
                // advance one more time, since we stop at the quote
                self.advance();
                // trim the quotes, and add the token
                // substring start + 1 end - 1
                let string = self.chars[(self.start + 1)..(self.current - 1)]
                    .iter()
                    .cloned()
                    .collect::<String>();
                self.add_token_literal(TokenType::String, Literal::String(string));
            }
            // digit
            n if n.is_digit(10) => {
                while self.peek().is_digit(10) {
                    self.advance();
                }
                // if fraction continue, also 0. doesn't work, it has to be 0.(digit+)
                if self.peek() == '.' && self.peek_next().is_digit(10) {
                    // consume .
                    self.advance();
                    // get the digits to the right
                    while self.peek().is_digit(10) {
                        self.advance();
                    }
                }

                // get string
                let string = self.chars[(self.start)..(self.current)]
                    .iter()
                    .cloned()
                    .collect::<String>();
                // parse into f64
                let float = string.parse::<f32>().unwrap();
                // insert float into tokens
                self.add_token_literal(TokenType::Number, Literal::Number(float));
            }
            _ => {
                Lox::error(self.line as u32, "unexpected character.");
            }
        }
    }

    fn next_is(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.chars[self.current] != expected {
            return false;
        }
        self.current += 1;
        true
    }

    /// Peek character at current
    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.chars[self.current]
        }
    }

    /// Peek character at one after current
    fn peek_next(&self) -> char {
        let idx = self.current + 1;
        if idx > self.chars.len() {
            '\0'
        } else {
            self.chars[idx]
        }
    }
}
