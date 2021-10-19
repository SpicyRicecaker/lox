use crate::token::Literal;
mod challenge;
pub mod printer;

// pg https://www.craftinginterpreters.com/representing-code.html
// i have no clue wtf I'm reading, why use a visitor problem? What does the code do?
// though, as far as I can see, it's implementable in rust https://rust-unofficial.github.io/patterns/patterns/behavioural/visitor.html

// Ok, though I do know that enums make 100% sense in defining expressions.
// so let's do that below

use crate::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    // e.g. expression operator expression
    Literal(Literal),
    // e.g. "(" expression ")"
    Grouping {
        expression: Box<Expr>,
    },
    Assign {
        name: Token,
        value: Box<Expr>,
    },
    // e.g. "2323", 123
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    // e.g. ( "-" | "!" ) expression
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    // E.g. [IDENTIFIER] accesses a variable
    Variable {
        name: Token,
    },
    // null
    Null,
}

#[derive(Debug)]
pub enum Stmt {
    Expr(Expr),
    Print(Expr),
    Var { name: Token, initializer: Expr },
    Block { statements: Vec<Stmt> },
}
