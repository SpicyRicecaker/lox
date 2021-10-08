use crate::token::Literal;
mod challenge;

use self::ast::{Binary, Grouping, Inspector, Unary};

// pg https://www.craftinginterpreters.com/representing-code.html
// i have no clue wtf I'm reading, why use a visitor problem? What does the code do?
// though, as far as I can see, it's implementable in rust https://rust-unofficial.github.io/patterns/patterns/behavioural/visitor.html

// Ok, though I do know that enums make 100% sense in defining expressions.
// so let's do that below

mod ast {
    use crate::token::Literal;
    use crate::token::Token;

    use super::challenge;
    use super::Visitor;

    pub struct Binary {
        pub left: Box<Expr>,
        pub operator: Token,
        pub right: Box<Expr>,
    }

    impl Binary {
        pub fn new(left: Box<Expr>, operator: Token, right: Box<Expr>) -> Self {
            Binary {
                left,
                operator,
                right,
            }
        }
    }

    pub struct Grouping {
        pub expression: Box<Expr>,
    }

    impl Grouping {
        pub fn new(expression: Box<Expr>) -> Self {
            Grouping { expression }
        }
    }

    pub struct Unary {
        pub operator: Token,
        pub right: Box<Expr>,
    }

    impl Unary {
        pub fn new(operator: Token, right: Box<Expr>) -> Self {
            Unary { operator, right }
        }
    }

    pub enum Expr {
        // e.g. expression operator expression
        Literal(Literal),
        // e.g. "(" expression ")"
        Grouping(Grouping),
        // e.g. "2323", 123
        Binary(Binary),
        // e.g. ( "-" | "!" ) expression
        Unary(Unary),
    }

    impl Expr {
        pub fn accept(&self, visitor: &Visitor) -> String {
            match self {
                Expr::Literal(e) => visitor.visit_literal(e),
                Expr::Grouping(e) => visitor.visit_grouping(e),
                Expr::Binary(e) => visitor.visit_binary(e),
                Expr::Unary(e) => visitor.visit_unary(e),
            }
        }
        pub fn accept_mut(&self, visitor: &mut challenge::ReversePolishNotation) {
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
    pub trait InspectorMut {
        fn visit_binary(&mut self, expr: &Binary);
        fn visit_unary(&mut self, expr: &Unary);
        fn visit_grouping(&mut self, expr: &Grouping);
        fn visit_literal(&mut self, expr: &Literal);
    }
}

pub struct Visitor;

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
    use crate::tree::Visitor;

    #[test]
    fn tree() {
        use super::ast::*;
        use crate::token::Literal;
        use crate::token::Token;
        use crate::token::TokenType;
        // create a new tree
        let binary_expression = Expr::Binary(Binary::new(
            Box::new(Expr::Unary(Unary::new(
                Token::new(TokenType::Minus, "-".to_string(), Literal::None, 1),
                Box::new(Expr::Literal(Literal::Number(123.0))),
            ))),
            Token::new(TokenType::Star, "*".to_string(), Literal::None, 1),
            Box::new(Expr::Grouping(Grouping::new(Box::new(Expr::Literal(
                Literal::Number(45.67),
            ))))),
        ));
        let visitor = Visitor {};
        let res = binary_expression.accept(&visitor);

        assert_eq!(res, "(* (- 123) (group 45.67))");
    }
}
