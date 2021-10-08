use std::collections::VecDeque;

use crate::token::{Token, TokenType};

use super::ast::InspectorMut;
use super::*;

pub struct ReversePolishNotation {
    stack: VecDeque<Token>,
    output: Vec<String>,
}

impl ReversePolishNotation {
    pub fn new() -> Self {
        ReversePolishNotation {
            stack: VecDeque::new(),
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
            _ => -1,
        }
    }
    /// Fully returns output of AST, clearing stack and output in the process
    pub fn output(&mut self) -> String {
        // pop the rest of our stack
        while let Some(s) = self.stack.pop_front() {
            self.output.push(s.lexeme);
        }
        let out = self.output.join(" ");
        self.output.clear();
        out
    }
    pub fn push_operator(&mut self, token: &Token) {
        // check
        let prio = Self::prio(token);

        // if paren pop all
        if prio == 3 {
            while let Some(c) = self.stack.pop_front() {
                if c.token_type == TokenType::LeftParen {
                    break;
                }
            }
        } else {
            while let Some(char) = self.stack.front() {
                // if the prio of current operator is greater or equal
                if prio >= Self::prio(char) {
                    // pop the top of the stack into the output
                    self.output.push(self.stack.pop_front().unwrap().lexeme);
                } else {
                    // otherwise don't modify the current stack
                    break;
                }
            }
            // push self to struct
            self.stack.push_front(token.clone());
        }
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
            Literal::None,
            1,
        ));
        expr.expression.accept_mut(self);
        self.push_operator(&Token::new(
            TokenType::RightParen,
            ")".to_string(),
            Literal::None,
            1,
        ));
    }

    // Always just push literals
    fn visit_literal(&mut self, expr: &Literal) {
        self.output.push(format!("{}", expr));
    }
}

#[test]
fn rpn() {
    use super::ast::*;
    use crate::token::Literal;
    use crate::token::Token;
    use crate::token::TokenType;
    // create a new tree
    let binary_expression = Expr::Binary(Binary::new(
        Box::new(Expr::Binary(Binary::new(
            Box::new(Expr::Literal(Literal::Number(1.0))),
            Token::new(TokenType::Plus, "+".to_string(), Literal::None, 1),
            Box::new(Expr::Literal(Literal::Number(2.0))),
        ))),
        Token::new(TokenType::Star, "*".to_string(), Literal::None, 1),
        Box::new(Expr::Binary(Binary::new(
            Box::new(Expr::Literal(Literal::Number(4.0))),
            Token::new(TokenType::Minus, "-".to_string(), Literal::None, 1),
            Box::new(Expr::Literal(Literal::Number(3.0))),
        ))),
    ));
    let mut visitor = ReversePolishNotation::new();
    binary_expression.accept_mut(&mut visitor);
    let out = visitor.output();
    println!("{}", out);

    assert_eq!(out, "1 2 + 4 3 - *");
}
