use crate::token::{Literal, Token};

use super::Expr;

impl Expr {
    pub fn accept_str(&self, visitor: &Visitor) -> String {
        match self {
            Expr::Literal(e) => visitor.visit_literal(e),
            Expr::Grouping { expression } => visitor.visit_grouping(expression),
            Expr::Binary {
                left,
                operator,
                right,
            } => visitor.visit_binary(left, operator, right),
            Expr::Unary { operator, right } => visitor.visit_unary(operator, right),
            _ => {
                panic!()
            }
        }
    }
}

pub trait Inspector<T> {
    fn visit_binary(&self, left: &Expr, operator: &Token, right: &Expr) -> T;
    fn visit_unary(&self, operator: &Token, right: &Expr) -> T;
    fn visit_grouping(&self, expr: &Expr) -> T;
    fn visit_literal(&self, expr: &Literal) -> T;
}

pub struct Visitor;

impl Visitor {
    pub fn new() -> Self {
        Visitor {}
    }
    pub fn print(&self, expr: &Expr) -> String {
        expr.accept_str(self)
    }
}

impl Default for Visitor {
    fn default() -> Self {
        Self::new()
    }
}

impl Inspector<String> for Visitor {
    fn visit_binary(&self, left: &Expr, operator: &Token, right: &Expr) -> String {
        format!(
            "({} {} {})",
            operator,
            left.accept_str(self),
            right.accept_str(self)
        )
    }
    fn visit_unary(&self, operator: &Token, right: &Expr) -> String {
        format!("({} {})", operator, right.accept_str(self))
    }
    fn visit_grouping(&self, expr: &Expr) -> String {
        format!("(group {})", expr.accept_str(self))
    }
    fn visit_literal(&self, expr: &Literal) -> String {
        format!("{}", expr)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn tree() {
        use super::*;
        use crate::token::Literal;
        use crate::token::Token;
        use crate::token::TokenType;
        // create a new tree
        let binary_expression = Expr::Binary {
            left: Box::new(Expr::Unary {
                operator: Token {
                    token_type: TokenType::Minus,
                    lexeme: "-".to_string(),
                    literal: Literal::Nil,
                    line: 1,
                },
                right: Box::new(Expr::Literal(Literal::Number(123.0))),
            }),
            operator: Token {
                token_type: TokenType::Star,
                lexeme: "*".to_string(),
                literal: Literal::Nil,
                line: 1,
            },
            right: Box::new(Expr::Grouping {
                expression: Box::new(Expr::Literal(Literal::Number(45.67))),
            }),
        };
        let str = Visitor::new().print(&binary_expression);

        assert_eq!(str, "(* (- 123) (group 45.67))");
    }
}
