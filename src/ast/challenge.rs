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
    fn visit_binary(&mut self, left: &Expr, operator: &Token, right: &Expr) {
        // push nums to output always
        left.accept_mut(self);
        self.push_operator(operator);
        right.accept_mut(self);
    }

    fn visit_unary(&mut self, operator: &Token, right: &Expr) {
        self.push_operator(operator);
        right.accept_mut(self);
    }

    fn visit_grouping(&mut self, expr: &Expr) {
        self.push_operator(&Token::new(
            TokenType::LeftParen,
            "(".to_string(),
            Literal::Nil,
            1,
        ));
        expr.accept_mut(self);
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
            Expr::Grouping { expression } => visitor.visit_grouping(expression),
            Expr::Binary {
                left,
                operator,
                right,
            } => visitor.visit_binary(left, operator, right),
            Expr::Unary { operator, right } => visitor.visit_unary(operator, right),
            Expr::Var { name } => todo!(),
        }
    }
}
pub trait InspectorMut {
    fn visit_binary(&mut self, left: &Expr, operator: &Token, right: &Expr);
    fn visit_unary(&mut self, operator: &Token, right: &Expr);
    fn visit_grouping(&mut self, expr: &Expr);
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

    let binary_expression = Expr::Binary {
        left: Box::new(Expr::Binary {
            left: Box::new(Expr::Literal(Literal::Number(1.0))),
            operator: Token {
                token_type: TokenType::Plus,
                lexeme: "+".to_string(),
                literal: Literal::Nil,
                line: 1,
            },
            right: Box::new(Expr::Literal(Literal::Number(2.0))),
        }),
        operator: Token {
            token_type: TokenType::Star,
            lexeme: "*".to_string(),
            literal: Literal::Nil,
            line: 1,
        },
        right: Box::new(Expr::Binary {
            left: Box::new(Expr::Literal(Literal::Number(4.0))),
            operator: Token {
                token_type: TokenType::Minus,
                lexeme: "-".to_string(),
                literal: Literal::Nil,
                line: 1,
            },
            right: Box::new(Expr::Literal(Literal::Number(3.0))),
        }),
    };

    binary_expression.accept_mut(&mut visitor);

    assert_eq!(visitor.output(), "1 2 4 * + 3 -");

    // let grouping = Expr::Binary {
    //     left: Box::new(Expr::Binary {
    //         left: Box::new(Expr::Literal(Literal::Number(1.0))),
    //         operator: Token {
    //             token_type: TokenType::Plus,
    //             lexeme: "+".to_string(),
    //             literal: Literal::Nil,
    //             line: 1,
    //         },
    //         right: Box::new(Expr::Literal(Literal::Number(2.0))),
    //     }),
    //     operator: Token {
    //         token_type: TokenType::Star,
    //         lexeme: "*".to_string(),
    //         literal: Literal::Nil,
    //         line: 1,
    //     },
    //     right: Box::new(Expr::Binary {
    //         left: Box::new(Expr::Literal(Literal::Number(4.0))),
    //         operator: Token {
    //             token_type: TokenType::Minus,
    //             lexeme: "-".to_string(),
    //             literal: Literal::Nil,
    //             line: 1,
    //         },
    //         right: Box::new(Expr::Literal(Literal::Number(3.0))),
    //     }),
    // };

    // grouping.accept_mut(&mut visitor);

    // assert_eq!(visitor.output(), "1 2 + 4 3 - *");
}
