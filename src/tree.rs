// pg https://www.craftinginterpreters.com/representing-code.html
// i have no clue wtf I'm reading, why use a visitor problem? What does the code do?
// though, as far as I can see, it's implementable in rust https://rust-unofficial.github.io/patterns/patterns/behavioural/visitor.html

use crate::token::Token;

pub struct Expr<T> {
    pub state: T,
}

struct Binary<T, U> {
    left: Expr<T>,
    operator: Token,
    right: Expr<U>,
}
