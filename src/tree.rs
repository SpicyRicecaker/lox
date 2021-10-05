// pg https://www.craftinginterpreters.com/representing-code.html
// i have no clue wtf I'm reading, why use a visitor problem? What does the code do?
// though, as far as I can see, it's implementable in rust https://rust-unofficial.github.io/patterns/patterns/behavioural/visitor.html

// Ok, though I do know that enums make 100% sense in defining expressions.
// so let's do that below

mod ast {
    use crate::token::Literal;
    use crate::token::Token;

    pub enum Expr {
        // e.g. expression operator expression
        Binary(Box<Expr>, Token, Box<Expr>),
        // e.g. "(" expression ")"
        Grouping(Box<Expr>),
        // e.g. "2323", 123
        Literal(Literal),
        // e.g. ( "-" | "!" ) expression
        Unary(Token, Box<Expr>),
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn tree() {
        use super::ast::*;
        use crate::token::Literal;
        use crate::token::Token;
        use crate::token::TokenType;
        // create a new tree
        let expression = Expr::Binary(
            Box::new(Expr::Unary(
                Token::new(TokenType::Minus, "-".to_string(), Literal::None, 1),
                Box::new(Expr::Literal(Literal::Number(123.0))),
            )),
            Token::new(TokenType::Star, "*".to_string(), Literal::None, 1),
            Box::new(Expr::Literal(Literal::Number(45.67))),
        );
    }
}

// use crate::token::Token;

// pub struct Expr<T> {
//     pub state: T,
// }

// struct Binary<T, U> {
//     left: Expr<T>,
//     operator: Token,
//     right: Expr<U>,
// }
