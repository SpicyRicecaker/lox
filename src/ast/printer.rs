use crate::token::Literal;

use super::{Binary, Expr, Grouping, Unary};

impl Expr {
    pub fn accept(&self, visitor: &Visitor) -> String {
        match self {
            Expr::Literal(e) => visitor.visit_literal(e),
            Expr::Grouping(e) => visitor.visit_grouping(e),
            Expr::Binary(e) => visitor.visit_binary(e),
            Expr::Unary(e) => visitor.visit_unary(e),
        }
    }
}

pub trait Inspector {
    fn visit_binary(&self, expr: &Binary) -> String;
    fn visit_unary(&self, expr: &Unary) -> String;
    fn visit_grouping(&self, expr: &Grouping) -> String;
    fn visit_literal(&self, expr: &Literal) -> String;
}

pub struct Visitor;

impl Visitor {
    pub fn new() -> Self {
        Visitor {}
    }
    pub fn print(&self, expr: Expr) -> String {
        expr.accept(self)
    }
}

impl Default for Visitor {
    fn default() -> Self {
        Self::new()
    }
}

impl Inspector for Visitor {
    fn visit_binary(&self, expr: &Binary) -> String {
        format!(
            "({} {} {})",
            expr.operator,
            expr.left.accept(self),
            expr.right.accept(self)
        )
    }
    fn visit_unary(&self, expr: &Unary) -> String {
        format!("({} {})", expr.operator, expr.right.accept(self))
    }
    fn visit_grouping(&self, expr: &Grouping) -> String {
        format!("(group {})", expr.expression.accept(self))
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
        let binary_expression = Expr::Binary(Binary::new(
            Box::new(Expr::Unary(Unary::new(
                Token::new(TokenType::Minus, "-".to_string(), Literal::Nil, 1),
                Box::new(Expr::Literal(Literal::Number(123.0))),
            ))),
            Token::new(TokenType::Star, "*".to_string(), Literal::Nil, 1),
            Box::new(Expr::Grouping(Grouping::new(Box::new(Expr::Literal(
                Literal::Number(45.67),
            ))))),
        ));
        let str = Visitor::new().print(binary_expression);

        assert_eq!(str, "(* (- 123) (group 45.67))");
    }
}
