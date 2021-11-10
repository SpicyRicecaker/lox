pub mod error;

use crate::{
    ast::{Expr, Stmt},
    environment::error::env_error,
    token::{Literal, Token, TokenType},
};

use self::error::{ParseError, ParseErrorKind};

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
            // dbg!(&statements);
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
        // eprintln!("{:?}", statements);

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
            .consume(TokenType::Identifier, "var declaration beginning")?
            .clone();

        let mut initializer = Expr::Null;

        if self.matches(&[TokenType::Equal]) {
            initializer = self.expression()?;
        }

        self.consume(TokenType::Semicolon, "var declaration ending")?;

        Ok(Stmt::Var { name, initializer })
    }

    fn block(&mut self) -> Result<Stmt> {
        // create a new vec of statements
        let mut statements = Vec::new();

        // keep consuming tokens until we get to a right brace
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            // push it onto the vec
            statements.push(self.declaration()?);
        }

        // Then consume the right bracket
        self.consume(TokenType::RightBrace, "block")?;

        // Return our statements
        Ok(Stmt::Block { statements })
    }

    fn statement(&mut self) -> Result<Stmt> {
        // check if it's a print statement
        // TODO Could, should convert this into match statement, it's looking a lot like a certain Yandere Developer's code right now
        if self.matches(&[TokenType::Print]) {
            self.print_statement()
        // check if its a block
        } else if self.matches(&[TokenType::LeftBrace]) {
            // dbg!("its a block");
            self.block()
        } else if self.matches(&[TokenType::If]) {
            self.if_statement()
        } else if self.matches(&[TokenType::While]) {
            self.while_statement()
        } else if self.matches(&[TokenType::For]) {
            self.for_statement()
        } else {
            // otherwise just treat it as an extension
            self.expression_statement()
        }
    }

    /// Desugars a `for` loop to [Stmt::While]
    fn for_statement(&mut self) -> Result<Stmt> {
        // Take the for `(` beginning parenthesis
        self.consume(TokenType::LeftParen, "beginning of for statement")?;

        // Any part of the space in between semicolons can be omitted
        // for(; sdf; sdf)
        // for(sdf; ; sdf)
        // for(; ; sdf)
        // Are all valid

        // If there is no beginning initializer, i.e. `for(; i < 2; i++)`
        let initializer = if self.matches(&[TokenType::Semicolon]) {
            None
        }
        // If there is a beginning initializer, i.e. `for(var i = 0; i < 2; i++)`
        else if self.matches(&[TokenType::Var]) {
            // we declare a new variable
            Some(self.var_declaration()?)
        }
        // Believe it or not, for loops actually can do `for(i=0; ...)`
        else {
            Some(self.expression_statement()?)
        };

        // If there's no condition
        let condition = if self.check(TokenType::Semicolon) {
            None
        } else {
            Some(self.expression()?)
        };
        // We have to check instead of consume here because if user does not have any semicolons then we should error rather than interpret that as an optional omission
        self.consume(TokenType::Semicolon, "first for statement semicolon")?;

        // for (asdf; asdf; )
        let increment = if !self.check(TokenType::RightParen) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(TokenType::RightParen, "second for statement semicolon")?;

        // body refers to the `{ }` after the `for` statement
        let body = self.statement()?;

        // Now we start to build a ast syntax tree that includes all these elements

        // increment would be
        /*
        {
            user code here...
            b += 1;
        }
        */
        // If there is an increment add it to the body, otherwise leave body as as
        // map is pretty useful for manipulating options
        let body = if let Some(increment) = increment {
            Stmt::Block {
                statements: vec![body, Stmt::Expr(increment)],
            }
        } else {
            body
        };

        // If there is no condition, treat it as a `while (true) {}` loop
        let condition = if let Some(condition) = condition {
            condition
        } else {
            Expr::Literal(Literal::Boolean(false))
        };

        // If there is an initializer, we run it once before the whole loop
        // TODO this syntax looks a lot like the builder syntax
        // It's not wrong but it doesn't feel terribly right either
        let body = if let Some(initializer) = initializer {
            Stmt::Block {
                statements: vec![initializer, body],
            }
        } else {
            body
        };

        dbg!(&body);

        Ok(Stmt::While {
            condition,
            body: Box::new(body),
        })
    }

    /// Generates [Stmt::While] with a condition and a body
    fn while_statement(&mut self) -> Result<Stmt> {
        // Again, duplicate code that I'm too lazy to get rid of rn lol
        // See if_statement
        // First consume `(`
        self.consume(TokenType::LeftParen, "beginning of while statement")?;
        // Then consume the statemtent inside `(..)`
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "end of while statement")?;
        // Finaly, consume the right `)`
        let body = Box::new(self.statement()?);

        Ok(Stmt::While { condition, body })
    }

    /// Generates expr conditional, then statement, else statement
    /// Basically splits up an if statement in to if + `condition` + `left branch` + `right branch`
    /// `right branch` is only there if there is an `if  else` statement
    fn if_statement(&mut self) -> Result<Stmt> {
        // Consume left (
        // TODO it's not good nor idiomatic that we have to generate an error like this
        // Maybe helper method that gens this? Would like to see what the runtime error looks like first
        self.consume(TokenType::LeftParen, "beginning of if statement")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "end of if statement")?;
        // Note that we use self.statement() here instead of self.block(), because unlike rust we want to
        // support single-line conditional into statements like `if (true) run();`
        let then_branch = self.statement()?;
        let else_branch = if self.matches(&[TokenType::Else]) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };

        Ok(Stmt::If {
            condition,
            then_branch: Box::new(then_branch),
            else_branch,
        })
    }

    /// Generates print expr statement
    fn print_statement(&mut self) -> Result<Stmt> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "after print statement")?;
        Ok(Stmt::Print(value))
    }

    /// Generates stock expr statement
    fn expression_statement(&mut self) -> Result<Stmt> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "after expr statement")?;
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
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr> {
        // Offset the call to the or statement, which will eventually come back to equality
        let expr = self.or()?;

        // if we find an equals after the left-hand side,
        // wrap it all up in an assignment expression
        if self.matches(&[TokenType::Equal]) {
            let _equals = self.previous();
            // right-associative recursion is ok
            let value = self.assignment()?;

            if let Expr::Variable { name } = expr {
                Ok(Expr::Assign {
                    name,
                    value: Box::new(value),
                })
            } else {
                Err(Box::new(env_error(self.peek(), "during assignment")))
            }
        } else {
            // otherwise just return the expr
            Ok(expr)
        }
    }

    fn or(&mut self) -> Result<Expr> {
        // Match the lower precedence level, or has lower precendence than and (and is evaluated first)
        let mut expr = self.and()?;

        while self.matches(&[TokenType::Or]) {
            let operator = self.previous().clone();
            let right = self.and()?;
            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr> {
        let mut expr = self.equality()?;

        while self.matches(&[TokenType::And]) {
            let operator = self.previous().clone();
            let right = self.equality()?;
            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
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

    /// The lowest precedence part of the context-free grammar, matches various primitive types like `false`, `(` + `expr` + `)`, etc. 
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
                TokenType::Identifier => Expr::Variable {
                    name: self.previous().clone(),
                },
                TokenType::LeftParen => {
                    let expr = self.expression()?;
                    self.consume(
                        TokenType::RightParen,
                        "during primary (), while matching parentheses",
                    )?;
                    Expr::Grouping {
                        expression: Box::new(expr),
                    }
                }
                // The tokens below shouldn't be in primary, so it's mostly error collection
                // Call factor to evaluate the rest of the statement as a factor, not as terms
                TokenType::Star | TokenType::Slash => {
                    let prev = self.previous().clone();
                    self.factor()?;
                    return Err(Box::new(ParseError::new(
                        ParseErrorKind::ExpectLeftOperand(prev),
                    )));
                }
                TokenType::Less
                | TokenType::LessEqual
                | TokenType::Greater
                | TokenType::GreaterEqual => {
                    let prev = self.previous().clone();
                    self.comparison()?;
                    return Err(Box::new(ParseError::new(
                        ParseErrorKind::ExpectLeftOperand(prev),
                    )));
                }
                TokenType::BangEqual | TokenType::EqualEqual => {
                    let prev = self.previous().clone();
                    self.equality()?;
                    return Err(Box::new(ParseError::new(
                        ParseErrorKind::ExpectLeftOperand(prev),
                    )));
                }
                TokenType::Plus => {
                    let prev = self.previous().clone();
                    self.term()?;
                    return Err(Box::new(ParseError::new(
                        ParseErrorKind::ExpectLeftOperand(prev),
                    )));
                }
                _ => {
                    return Err(Box::new(ParseError::new(
                        ParseErrorKind::Error(self.peek().clone(), self.peek().clone(), "invalid sequence, shouldn't ever occur".into()),
                    )));
                }
            };
            Ok(expr)
        } else {
            Err(Box::new(ParseError::new(
                ParseErrorKind::Error(self.peek().clone(), self.peek().clone(), "don't know why this would occur".into()),
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

    /// Return reference to token at current position
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    /// Return token at current position, cloned
    fn peek_clone(&self) -> Token {
        self.tokens[self.current].clone()
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
    fn consume(&mut self, token_type: TokenType, string: &str) -> Result<&Token> {
        // if current is on token type
        if self.check(token_type) {
            // iterate over it
            Ok(self.advance())
        } else {
            Err(Box::new(ParseError::new(ParseErrorKind::Error(
                Token::new(token_type, "any".into(), Literal::Nil, 0),
                self.peek_clone(),
                string.into(),
            ))))
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
            self.advance();
        }
    }
}

#[cfg(test)]
mod test {
    use crate::interpreter::InterpreterVisitor;

    #[test]
    fn parse() {
        let mut interpreter = InterpreterVisitor::new();
        match crate::run("1+1".to_string(), &mut interpreter) {
            Ok(_) => {}
            Err(e) => eprintln!("{}", e),
        };
    }
}
