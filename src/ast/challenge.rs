use crate::token::{Token, TokenType};

use super::*;

pub struct ReversePolishNotation {
    stack: Vec<Token>,
    output: Vec<String>,
}

impl ReversePolishNotation {
    pub fn new() -> Self {
        ReversePolishNotation {
            stack: Vec::new(),
            output: Vec::new(),
        }
    }
    pub fn prio(token: &Token) -> i32 {
        match token.token_type {
            TokenType::Star => 2,
            TokenType::Slash => 2,
            TokenType::Plus => 1,
            TokenType::Minus => 1,
            TokenType::RightParen => 3,
            TokenType::LeftParen => 4,
            _ => -1,
        }
    }
    /// Fully returns output of AST, clearing stack and output in the process
    pub fn output(&mut self) -> String {
        // pop the rest of our stack
        while let Some(s) = self.stack.pop() {
            self.output.push(s.lexeme);
        }
        let out = self.output.join(" ");
        self.output.clear();
        out
    }
    pub fn push_operator(&mut self, token: &Token) {
        // get prio of current token
        let prio = Self::prio(token);

        // if paren pop all
        if prio == 3 {
            while let Some(c) = self.stack.pop() {
                if c.token_type == TokenType::LeftParen {
                    break;
                } else {
                    // add to output
                    self.output.push(c.lexeme);
                }
            }
        } else {
            while let Some(char) = self.stack.last() {
                // if the prio of current operator is greater or equal
                // only right paren has the right to remove left paren
                if char.token_type != TokenType::LeftParen && prio <= Self::prio(char) {
                    // pop the top of the stack into the output
                    self.output.push(self.stack.pop().unwrap().lexeme);
                } else {
                    // otherwise don't modify the current stack
                    break;
                }
            }
            // push self to struct
            self.stack.push(token.clone());
        }
    }
}

impl Default for ReversePolishNotation {
    fn default() -> Self {
        Self::new()
    }
}

impl InspectorMut for ReversePolishNotation {
    fn visit_binary(&mut self, expr: &Binary) {
        // push nums to output always
        expr.left.accept_mut(self);
        self.push_operator(&expr.operator);
        expr.right.accept_mut(self);
    }

    fn visit_unary(&mut self, expr: &Unary) {
        self.push_operator(&expr.operator);
        expr.right.accept_mut(self);
    }

    fn visit_grouping(&mut self, expr: &Grouping) {
        self.push_operator(&Token::new(
            TokenType::LeftParen,
            "(".to_string(),
            Literal::Nil,
            1,
        ));
        expr.expression.accept_mut(self);
        self.push_operator(&Token::new(
            TokenType::RightParen,
            ")".to_string(),
            Literal::Nil,
            1,
        ));
    }

    // Always just push literals
    fn visit_literal(&mut self, expr: &Literal) {
        self.output.push(format!("{}", expr));
    }
}

impl Expr {
    pub fn accept_mut(&self, visitor: &mut challenge::ReversePolishNotation) {
        match self {
            Expr::Literal(e) => visitor.visit_literal(e),
            Expr::Grouping(e) => visitor.visit_grouping(e),
            Expr::Binary(e) => visitor.visit_binary(e),
            Expr::Unary(e) => visitor.visit_unary(e),
        }
    }
}
pub trait InspectorMut {
    fn visit_binary(&mut self, expr: &Binary);
    fn visit_unary(&mut self, expr: &Unary);
    fn visit_grouping(&mut self, expr: &Grouping);
    fn visit_literal(&mut self, expr: &Literal);
}

#[test]
fn rpn() {
    use super::*;
    use crate::token::Literal;
    use crate::token::Token;
    use crate::token::TokenType;

    let mut visitor = ReversePolishNotation::new();

    // create a new tree
    let binary_expression = Expr::Binary(Binary::new(
        Box::new(Expr::Binary(Binary::new(
            Box::new(Expr::Literal(Literal::Number(1.0))),
            Token::new(TokenType::Plus, "+".to_string(), Literal::Nil, 1),
            Box::new(Expr::Literal(Literal::Number(2.0))),
        ))),
        Token::new(TokenType::Star, "*".to_string(), Literal::Nil, 1),
        Box::new(Expr::Binary(Binary::new(
            Box::new(Expr::Literal(Literal::Number(4.0))),
            Token::new(TokenType::Minus, "-".to_string(), Literal::Nil, 1),
            Box::new(Expr::Literal(Literal::Number(3.0))),
        ))),
    ));

    binary_expression.accept_mut(&mut visitor);

    assert_eq!(visitor.output(), "1 2 4 * + 3 -");

    let grouping = Expr::Binary(Binary::new(
        Box::new(Expr::Grouping(Grouping::new(Box::new(Expr::Binary(
            Binary::new(
                Box::new(Expr::Literal(Literal::Number(1.0))),
                Token::new(TokenType::Plus, "+".to_string(), Literal::Nil, 1),
                Box::new(Expr::Literal(Literal::Number(2.0))),
            ),
        ))))),
        Token::new(TokenType::Star, "*".to_string(), Literal::Nil, 1),
        Box::new(Expr::Grouping(Grouping::new(Box::new(Expr::Binary(
            Binary::new(
                Box::new(Expr::Literal(Literal::Number(4.0))),
                Token::new(TokenType::Plus, "-".to_string(), Literal::Nil, 1),
                Box::new(Expr::Literal(Literal::Number(3.0))),
            ),
        ))))),
    ));

    grouping.accept_mut(&mut visitor);

    assert_eq!(visitor.output(), "1 2 + 4 3 - *");
}
