use core::panic;

use crate::{
    token::{self, Literal, Token, TokenType},
    tree::ast::{Binary, Expr, Grouping, Unary},
};

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn recursive_descent(
        &mut self,
        token_type: &[TokenType],
        func: fn(&mut Parser) -> Expr,
    ) -> Expr {
        // initially, base expr matches anything of higher precedence
        let mut expr = func(self);

        // follows the rule `comparison (("!=" | "=") comparison )*`
        // so long as the next few tokens are bang equal and bang not equal
        while self.matches(token_type) {
            // set the operator to the previous thing (which should be a comparison)
            let operator = self.previous().clone();
            // set the right to another comparison
            let right = func(self);
            // then append it
            expr = Expr::Binary(Binary::new(Box::new(expr), operator, Box::new(right)));
        }
        expr
    }

    /// ranked in order of precedence (lowest precedence to highest precedence)
    /// expression just counts as equality, we theoretically don't need expression, but it reads better
    fn expression(&mut self) -> Expr {
        self.equality()
    }

    /// `==` and `!=`, we can multiple of these in a sentence so
    fn equality(&mut self) -> Expr {
        self.recursive_descent(
            &[TokenType::BangEqual, TokenType::EqualEqual],
            Self::comparison,
        )
    }

    /// !=, <=, etc.
    fn comparison(&mut self) -> Expr {
        self.recursive_descent(
            &[
                TokenType::Greater,
                TokenType::GreaterEqual,
                TokenType::Less,
                TokenType::Greater,
            ],
            Self::term,
        )
    }

    /// `+` and `-`, e.g. 1 + 2 + 3 + 4
    fn term(&mut self) -> Expr {
        self.recursive_descent(&[TokenType::Plus, TokenType::Minus], Self::factor)
    }

    /// `*` and `/`, e.g. 1 * 2 / 3
    fn factor(&mut self) -> Expr {
        self.recursive_descent(&[TokenType::Slash, TokenType::Star], Self::unary)
    }

    /// '!' or '-' found, we can recursively parse itself (i.e. !!true, --number, etc.)
    fn unary(&mut self) -> Expr {
        if self.matches(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary();
            Expr::Unary(Unary::new(operator, Box::new(right)))
        } else {
            self.primary()
        }
    }
    fn primary(&mut self) -> Expr {
        let expr = match self.peek().token_type {
            TokenType::False => Expr::Literal(Literal::Boolean(false)),
            TokenType::True => Expr::Literal(Literal::Boolean(true)),
            TokenType::Nil => Expr::Literal(Literal::Nil),
            TokenType::Number | TokenType::String => Expr::Literal(self.previous().literal.clone()),
            TokenType::LeftParen => {
                let expr = self.expression();
                self.consume(TokenType::RightParen, "expected ')' after expression.");
                Expr::Grouping(Grouping::new(Box::new(expr)))
            }
            _ => panic!(),
        };
        self.advance();
        expr
    }
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    /// If our token signals the end of file, then return it
    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    /// consumes current token and returns it
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    /// ret. true if current token is of given type
    /// does not consume token, however
    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().token_type == token_type
        }
    }

    fn matches(&mut self, types: &[TokenType]) -> bool {
        if types.iter().any(|c| self.check(*c)) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
}
