pub mod error;
use std::convert::TryFrom;

use crate::{
    ast::{
        printer::{Inspector, Visitor},
        Binary, Expr, Grouping, Unary,
    },
    token::{Literal, TokenType},
};

use self::error::{Error, ErrorKind};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// It might seem like objects are equivalent to literals
/// but the distinction is important to make because literals are in the parser's
/// domain while objects are in the runtime domain
/// We could theoretically also have classes and arbirary objects in the future
pub enum Object {
    String(String),
    Number(f32),
    Boolean(bool),
    Nil,
}

impl From<&Literal> for Object {
    fn from(l: &Literal) -> Self {
        match l {
            Literal::String(s) => Self::String(s.clone()),
            Literal::Number(n) => Self::Number(*n),
            Literal::Boolean(b) => Self::Boolean(*b),
            Literal::Nil => Self::Nil,
        }
    }
}

impl InspectorResult<Object> for Interpreter {
    fn visit_binary(&self, expr: &Binary) -> Result<Object> {
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;

        match expr.operator.token_type {
            TokenType::Minus => Ok(Object::Number(Self::try_num(left)? - Self::try_num(right)?)),
            TokenType::Slash => Ok(Object::Number(Self::try_num(left)? / Self::try_num(right)?)),
            TokenType::Star => Ok(Object::Number(Self::try_num(left)? * Self::try_num(right)?)),
            _ => panic!(),
        }
    }

    fn visit_unary(&self, expr: &Unary) -> Result<Object> {
        // format!("({} {})", expr.operator, expr.right.accept_str(self))
        let right = expr.right.accept(self)?;

        match expr.operator.token_type {
            TokenType::Minus => Ok(Object::Number(-Self::try_num(right)?)),
            TokenType::Bang => Ok(Object::Boolean(!Self::is_truthy(&right))),
            _ => panic!("Invalid unary type, unreachable"),
        }
    }
    fn visit_grouping(&self, expr: &Grouping) -> Result<Object> {
        self.evaluate(&expr.expression)
    }
    fn visit_literal(&self, expr: &Literal) -> Result<Object> {
        Ok(Object::from(expr))
    }
}

impl Expr {
    /// Duplicate method but I can't figure out how to separate into String and Objects atm
    pub fn accept<T>(&self, visitor: &T) -> Result<Object>
    where
        T: InspectorResult<Object>,
    {
        match self {
            Expr::Literal(e) => visitor.visit_literal(e),
            Expr::Grouping(e) => visitor.visit_grouping(e),
            Expr::Binary(e) => visitor.visit_binary(e),
            Expr::Unary(e) => visitor.visit_unary(e),
        }
    }
}

pub trait InspectorResult<T> {
    fn visit_binary(&self, expr: &Binary) -> Result<T>;
    fn visit_unary(&self, expr: &Unary) -> Result<T>;
    fn visit_grouping(&self, expr: &Grouping) -> Result<T>;
    fn visit_literal(&self, expr: &Literal) -> Result<T>;
}

struct Interpreter {}

impl Interpreter {
    fn evaluate(&self, expr: &crate::ast::Expr) -> Result<Object> {
        expr.accept(self)
    }
    fn is_truthy(ob: &Object) -> bool {
        match ob {
            Object::Boolean(b) => *b,
            Object::Nil => false,
            _ => true,
        }
    }
    fn try_num(value: Object) -> Result<f32> {
        if let Object::Number(n) = value {
            Ok(n)
        } else {
            Err(Box::new(Error::new(ErrorKind::FailedCast)))
        }
    }
    fn try_str(value: Object) -> Result<String> {
        if let Object::String(s) = value {
            Ok(s)
        } else {
            Err(Box::new(Error::new(ErrorKind::FailedCast)))
        }
    }
    fn try_bool(value: Object) -> Result<bool> {
        if let Object::Boolean(b) = value {
            Ok(b)
        } else {
            Err(Box::new(Error::new(ErrorKind::FailedCast)))
        }
    }
}
