use crate::token::Literal;
mod challenge;
pub mod printer;

// pg https://www.craftinginterpreters.com/representing-code.html
// i have no clue wtf I'm reading, why use a visitor problem? What does the code do?
// though, as far as I can see, it's implementable in rust https://rust-unofficial.github.io/patterns/patterns/behavioural/visitor.html

// Ok, though I do know that enums make 100% sense in defining expressions.
// so let's do that below

use crate::token::Token;

#[derive(Clone)]
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

#[derive(Clone)]
pub struct Grouping {
    pub expression: Box<Expr>,
}

impl Grouping {
    pub fn new(expression: Box<Expr>) -> Self {
        Grouping { expression }
    }
}

#[derive(Clone)]
pub struct Unary {
    pub operator: Token,
    pub right: Box<Expr>,
}

impl Unary {
    pub fn new(operator: Token, right: Box<Expr>) -> Self {
        Unary { operator, right }
    }
}

#[derive(Clone)]
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
