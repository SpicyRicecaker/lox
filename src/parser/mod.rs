pub mod error;

use crate::{
    ast::{Expr, Stmt},
    token::{Literal, Token, TokenType},
};
use error::{Error, ErrorKind};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn parse(&mut self) -> Result<Vec<Stmt>> {
        // create vec of statements
        let mut statements: Vec<Stmt> = Vec::new();

        // as long as we're not at end of file
        while !self.is_at_end() {
            // make mo statements
            let declaration = match self.declaration() {
                Ok(d) => d,
                Err(e) => {
                    // synchronize (e.g. ignore all other errors in the statement)
                    self.synchronize();
                    return Err(e);
                }
            };
            statements.push(declaration);
        }
        println!("{:?}", statements);

        Ok(statements)
    }

    fn declaration(&mut self) -> Result<Stmt> {
        if self.matches(&[TokenType::Var]) {
            self.var_declaration()
        } else {
            self.statement()
        }
    }

    fn var_declaration(&mut self) -> Result<Stmt> {
        let name = self
            .consume(
                TokenType::Identifier,
                Error::new(ErrorKind::ExpectVariableName),
            )?
            .clone();

        let mut initializer = Expr::Null;

        if self.matches(&[TokenType::Equal]) {
            initializer = self.expression()?;
        }

        self.consume(TokenType::Semicolon, Error::new(ErrorKind::ExpectSemicolon))?;

        Ok(Stmt::Var { name, initializer })
    }

    fn statement(&mut self) -> Result<Stmt> {
        // check if it's a print statement
        if self.matches(&[TokenType::Print]) {
            self.print_statement()
        } else {
            self.expression_statement()
        }
    }

    /// Generates print expr statement
    fn print_statement(&mut self) -> Result<Stmt> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, Error::new(ErrorKind::ExpectSemicolon))?;
        Ok(Stmt::Print(value))
    }

    /// Generates stock expr statement
    fn expression_statement(&mut self) -> Result<Stmt> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, Error::new(ErrorKind::ExpectSemicolon))?;
        Ok(Stmt::Expr(expr))
    }

    fn recursive_descent(
        &mut self,
        token_type: &[TokenType],
        func: fn(&mut Parser) -> Result<Expr>,
    ) -> Result<Expr> {
        // initially, base expr matches anything of higher precedence
        let mut expr = func(self)?;

        // follows the rule `comparison (("!=" | "=") comparison )*`
        // so long as the next few tokens are bang equal and bang not equal
        while self.matches(token_type) {
            // set the operator to the previous thing (which should be a comparison)
            let operator = self.previous().clone();
            // set the right to another comparison
            let right = func(self)?;
            // then append it
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    /// ranked in order of precedence (lowest precedence to highest precedence)
    /// expression just counts as equality, we theoretically don't need expression, but it reads better
    fn expression(&mut self) -> Result<Expr> {
        self.equality()
    }

    /// `==` and `!=`, we can multiple of these in a sentence so
    fn equality(&mut self) -> Result<Expr> {
        self.recursive_descent(
            &[TokenType::BangEqual, TokenType::EqualEqual],
            Self::comparison,
        )
    }

    /// !=, <=, etc.
    fn comparison(&mut self) -> Result<Expr> {
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
    fn term(&mut self) -> Result<Expr> {
        self.recursive_descent(&[TokenType::Plus, TokenType::Minus], Self::factor)
    }

    /// `*` and `/`, e.g. 1 * 2 / 3
    fn factor(&mut self) -> Result<Expr> {
        self.recursive_descent(&[TokenType::Slash, TokenType::Star], Self::unary)
    }

    /// '!' or '-' found, we can recursively parse itself (i.e. !!true, --number, etc.)
    fn unary(&mut self) -> Result<Expr> {
        if self.matches(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            })
        } else {
            Ok(self.primary()?)
        }
    }
    fn primary(&mut self) -> Result<Expr> {
        // TODO How do we get rid of this duplication zzz.
        if self.matches(&[
            TokenType::False,
            TokenType::True,
            TokenType::Nil,
            TokenType::Number,
            TokenType::String,
            TokenType::LeftParen,
            // better error handling, no left operand
            // + 2
            TokenType::Plus,
            // * 2
            TokenType::Star,
            // / 2
            TokenType::Slash,
            // < 2
            TokenType::Less,
            // <= 2
            TokenType::LessEqual,
            // > 2
            TokenType::Greater,
            // >= 2
            TokenType::GreaterEqual,
            // != 2
            TokenType::BangEqual,
            // == 2
            TokenType::EqualEqual,
            // [identifier]
            TokenType::Identifier,
        ]) {
            let expr = match self.previous().token_type {
                TokenType::False => Expr::Literal(Literal::Boolean(false)),
                TokenType::True => Expr::Literal(Literal::Boolean(true)),
                TokenType::Nil => Expr::Literal(Literal::Nil),
                TokenType::Number | TokenType::String => {
                    Expr::Literal(self.previous().literal.clone())
                }
                // in `var apple = 2;`, the name token would just be the previous, which would then match apple!
                TokenType::Var => Expr::Variable {
                    name: self.previous().clone(),
                },
                TokenType::LeftParen => {
                    let expr = self.expression()?;
                    self.consume(
                        TokenType::RightParen,
                        Error::new(ErrorKind::UnmatchedParen(self.peek().clone())),
                    )?;
                    Expr::Grouping {
                        expression: Box::new(expr),
                    }
                }
                // Call factor to evaluate the rest of the statement as a factor, not as terms
                TokenType::Star | TokenType::Slash => {
                    let prev = self.previous().clone();
                    self.factor()?;
                    return Err(Box::new(self::error::Error::new(
                        self::error::ErrorKind::ExpectLeftOperand(prev),
                    )));
                }
                TokenType::Less
                | TokenType::LessEqual
                | TokenType::Greater
                | TokenType::GreaterEqual => {
                    let prev = self.previous().clone();
                    self.comparison()?;
                    return Err(Box::new(self::error::Error::new(
                        self::error::ErrorKind::ExpectLeftOperand(prev),
                    )));
                }
                TokenType::BangEqual | TokenType::EqualEqual => {
                    let prev = self.previous().clone();
                    self.equality()?;
                    return Err(Box::new(self::error::Error::new(
                        self::error::ErrorKind::ExpectLeftOperand(prev),
                    )));
                }
                TokenType::Plus => {
                    let prev = self.previous().clone();
                    self.term()?;
                    return Err(Box::new(self::error::Error::new(
                        self::error::ErrorKind::ExpectLeftOperand(prev),
                    )));
                }
                _ => {
                    return Err(Box::new(self::error::Error::new(
                        self::error::ErrorKind::ExpectExpression(self.peek().clone()),
                    )));
                }
            };
            Ok(expr)
        } else {
            Err(Box::new(self::error::Error::new(
                self::error::ErrorKind::ExpectExpression(self.peek().clone()),
            )))
        }
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

    /// checks if current cursor is over a certain token type, consumes it if so otherwise errors
    fn consume(&mut self, token_type: TokenType, error: Error) -> Result<&Token> {
        // if current is on token type
        if self.check(token_type) {
            // iterate over it
            Ok(self.advance())
        } else {
            Err(Box::new(error))
        }
    }
    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    /// In error handling, when we come across an error, we want to report it, but not any of
    /// the false positives that will be generated from it as a result.
    /// This function discards all errors until what it thinks is the next statement (e.g. right after a semicolon)
    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            match self.peek().token_type {
                TokenType::Class
                | TokenType::Func
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => {}
            }
        }
        self.advance();
    }
}

#[cfg(test)]
mod test {
    use crate::interpreter::Interpreter;

    #[test]
    fn parse() {
        let mut interpreter = Interpreter::new();
        match crate::run("1+1".to_string(), &mut interpreter) {
            Ok(_) => {}
            Err(e) => eprintln!("{}", e),
        };
    }
}
