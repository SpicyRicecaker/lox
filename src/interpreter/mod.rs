pub mod error;
use std::fmt::Display;

use crate::{
    ast::{Expr, Stmt},
    environment::{Cactus, Environment},
    token::{Literal, Token, TokenType},
};

use self::error::{Error, ErrorKind};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// It might seem like objects are equivalent to literals
/// but the distinction is important to make because literals are in the parser's
/// domain while objects are in the runtime domain
/// We could theoretically also have classes and arbirary objects in the future
#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    String(String),
    Number(f32),
    Boolean(bool),
    Nil,
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // not sure if the derefs are needed rn
        match self {
            Object::String(s) => write!(f, "{}", s),
            Object::Number(n) => write!(f, "{}", *n),
            Object::Boolean(b) => write!(f, "{}", *b),
            Object::Nil => write!(f, "null"),
        }
    }
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

impl TreeVisitor<Object> for InterpreterVisitor {
    fn visit_binary(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Result<Object> {
        let left = self.evaluate(left)?;
        let right = self.evaluate(right)?;

        use TokenType::*;

        Ok(match operator.token_type {
            Minus => Object::Number(Self::try_num(left)? - Self::try_num(right)?),
            Slash => {
                let right = Self::try_num(right)?;
                let left = Self::try_num(left)?;
                if right == 0.0 {
                    return Err(Box::new(Error::new(ErrorKind::DivideByZero(left))));
                } else {
                    Object::Number(left / right)
                }
            }
            Star => Object::Number(Self::try_num(left)? * Self::try_num(right)?),
            Plus => {
                // deviation: too lazy to write errors for these things rn
                match left {
                    // Could use + operator to add numbers
                    Object::Number(n) => match right {
                        Object::String(r) => Object::String(format!("{}{}", n, r)),
                        _ => Object::Number(n + Self::try_num(right)?),
                    },
                    // Could also use + operator to concatenate strings
                    Object::String(l) => Object::String(format!("{}{}", l, right)),
                    _ => return Err(Box::new(Error::new(ErrorKind::FailedCast))),
                }
            }
            Greater => Object::Boolean(Self::try_num(left)? > Self::try_num(right)?),
            GreaterEqual => Object::Boolean(Self::try_num(left)? >= Self::try_num(right)?),
            Less => Object::Boolean(Self::try_num(left)? < Self::try_num(right)?),
            LessEqual => Object::Boolean(Self::try_num(left)? <= Self::try_num(right)?),
            // TODO TODO TODO Not sure if derivce(PartialEq) handles enum comparisons automatically
            BangEqual => Object::Boolean(left != right),
            EqualEqual => Object::Boolean(left == right),
            _ => panic!(),
        })
    }

    fn visit_unary(&mut self, operator: &Token, right: &Expr) -> Result<Object> {
        // format!("({} {})", expr.operator, expr.right.accept_str(self))
        let right = right.accept(self)?;

        match operator.token_type {
            TokenType::Minus => Ok(Object::Number(-Self::try_num(right)?)),
            TokenType::Bang => Ok(Object::Boolean(!Self::is_truthy(&right))),
            _ => panic!("Invalid unary type, unreachable"),
        }
    }
    fn visit_grouping(&mut self, expr: &Expr) -> Result<Object> {
        self.evaluate(expr)
    }
    fn visit_literal(&self, expr: &Literal) -> Result<Object> {
        Ok(Object::from(expr))
    }

    fn visit_variable(&self, name: &Token) -> Result<Object> {
        Ok(self.cactus.get(name, self.curr_env)?.clone())
    }

    fn visit_assign_expr(&mut self, name: &Token, value: &Expr) -> Result<Object> {
        let value = self.evaluate(value)?;
        self.cactus.assign(name, value.clone(), self.curr_env)?;
        Ok(value)
    }

    ///
    fn visit_logical_expr(
        &mut self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<Object> {
        let left = self.evaluate(left)?;

        // We wan't to short-circuit if the left expression satisfies the condition
        // This means that if left is true in an or statemnt, then the entire expression is true and we can just return left
        // But if left is false in an and statement, then the entire expression is false and we can just return left
        // TODO I'm sure there's some way to shorten
        if operator.token_type == TokenType::Or {
            if Self::is_truthy(&left) {
                return Ok(left);
            }
        } else if !Self::is_truthy(&left) {
            return Ok(left);
        }
        // Otherwise, our only choice is to evaluate right
        self.evaluate(right)
    }
}

impl Expr {
    /// Duplicate method but I can't figure out how to separate into String and Objects atm
    pub fn accept<T>(&self, visitor: &mut T) -> Result<Object>
    where
        T: TreeVisitor<Object>,
    {
        match self {
            Expr::Literal(e) => visitor.visit_literal(e),
            Expr::Grouping { expression } => visitor.visit_grouping(expression),
            Expr::Binary {
                left,
                operator,
                right,
            } => visitor.visit_binary(left, operator, right),
            Expr::Unary { operator, right } => visitor.visit_unary(operator, right),
            Expr::Variable { name } => visitor.visit_variable(name),
            Expr::Null => panic!("shouldn't be null expr"),
            Expr::Assign { name, value } => visitor.visit_assign_expr(name, value),
            Expr::Logical {
                left,
                operator,
                right,
            } => visitor.visit_logical_expr(left, operator, right),
        }
    }
}

pub trait TreeVisitor<T> {
    fn visit_binary(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Result<T>;
    fn visit_unary(&mut self, operator: &Token, right: &Expr) -> Result<T>;
    fn visit_grouping(&mut self, expr: &Expr) -> Result<T>;
    fn visit_literal(&self, expr: &Literal) -> Result<T>;
    fn visit_variable(&self, name: &Token) -> Result<T>;
    fn visit_assign_expr(&mut self, name: &Token, value: &Expr) -> Result<T>;
    fn visit_logical_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Result<T>;
}

/// Statement is a debuggin print thing
pub trait StatementVisitor {
    fn visit_expression_stmt(&mut self, stmt: &Expr) -> Result<()>;
    fn visit_print_stmt(&mut self, stmt: &Expr) -> Result<()>;
    fn visit_var_stmt(&mut self, name: &Token, initializer: &Expr) -> Result<()>;
    fn visit_if_stmt(
        &mut self,
        condition: &Expr,
        then_branch: &Stmt,
        else_branch: Option<&Stmt>,
    ) -> Result<()>;
}

impl StatementVisitor for InterpreterVisitor {
    fn visit_expression_stmt(&mut self, stmt: &Expr) -> Result<()> {
        // println!("calling expression");
        self.evaluate(stmt)?;
        Ok(())
    }

    fn visit_print_stmt(&mut self, stmt: &Expr) -> Result<()> {
        let val = self.evaluate(stmt)?;
        match val {
            Object::Nil => return Err(Box::new(Error::new(ErrorKind::UnitializedVariable))),
            v => println!("{}", v),
        }
        Ok(())
    }

    fn visit_var_stmt(&mut self, name: &Token, initializer: &Expr) -> Result<()> {
        let obj = match initializer {
            Expr::Null => Object::Nil,
            _ => self.evaluate(initializer)?,
        };

        // dbg!(self.curr_env);
        self.cactus.define(&name.lexeme, obj, self.curr_env);

        Ok(())
    }

    /// Given a `condition` and `left branch` and `right branch`, if the `condition` evaluates to `true`,
    /// execute() `left branch`, otherwise execute `right branch`
    fn visit_if_stmt(
        &mut self,
        condition: &Expr,
        then_branch: &Stmt,
        else_branch: Option<&Stmt>,
    ) -> Result<()> {
        // In an if statement, only run the code in the block if the condition is actually true
        if Self::is_truthy(&self.evaluate(condition)?) {
            // Run the if branch
            self.execute(then_branch)?;
        // Otherwise, if the expression is not truthy and we actually have an else branch
        } else if let Some(stmt) = else_branch {
            // Then execute it, passing the reference into
            self.execute(stmt)?;
        }

        Ok(())
    }
}

pub struct InterpreterVisitor {
    cactus: Cactus,
    // global_env: usize,
    curr_env: usize,
}

impl InterpreterVisitor {
    pub fn new() -> Self {
        let cactus = Cactus::new();
        // println!("setting curr env to {}", cactus.cur_env);
        let curr_env = cactus.cur_env;
        // let mut tree = Arena {arena: Vec::new()};
        // let idx = tree.push(Environment::new());

        InterpreterVisitor {
            cactus,
            // global_env: curr_env,
            curr_env,
        }
    }
    pub fn accept(&mut self, stmt: &Stmt) -> Result<()> {
        match stmt {
            Stmt::Expr(e) => self.visit_expression_stmt(e),
            Stmt::Print(e) => self.visit_print_stmt(e),
            Stmt::Var { name, initializer } => self.visit_var_stmt(name, initializer),
            Stmt::Block { statements } => self.visit_block(statements),
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => self.visit_if_stmt(condition, then_branch, else_branch.as_deref()),
        }
    }
    pub fn execute(&mut self, stmt: &Stmt) -> Result<()> {
        // println!("[dbg] calling execute()");
        self.accept(stmt)
    }
    pub fn interpret(&mut self, stmts: Vec<Stmt>) -> Result<()> {
        // let value = self.evaluate(expr)?;
        // deviation: no stringify method here because rust uses `impl Display` instead
        stmts.iter().try_for_each(|s| self.execute(s))
    }
    pub fn evaluate(&mut self, expr: &crate::ast::Expr) -> Result<Object> {
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
    // fn try_str(value: Object) -> Result<String> {
    //     if let Object::String(s) = value {
    //         Ok(s)
    //     } else {
    //         Err(Box::new(Error::new(ErrorKind::FailedCast)))
    //     }
    // }
    // fn try_bool(value: Object) -> Result<bool> {
    //     if let Object::Boolean(b) = value {
    //         Ok(b)
    //     } else {
    //         Err(Box::new(Error::new(ErrorKind::FailedCast)))
    //     }
    // }

    fn visit_block(&mut self, statements: &[Stmt]) -> Result<()> {
        // remember current environment
        let previous = self.curr_env;
        // Create a new environment for the current block
        // TODO could probably have push return `Node<ID>`
        self.curr_env = self.cactus.arena.push(Environment::new());
        // set current environment's parent to previous
        let n = self.cactus.arena.get_mut(self.curr_env).unwrap();
        n.parent = Some(previous);

        // Execute all the statements
        statements.iter().try_for_each(|s| self.accept(s))?;

        // println!("actually executing statmenet");

        // Reset environment

        // Remove latest node from vec. Because of how a parent-pointer tree (or cactus stack works) this probably always pops the child
        // **probably**
        self.cactus.arena.pop();
        // Reset parent
        self.curr_env = previous;

        // Set environment to self.current
        Ok(())
    }
}

impl Default for InterpreterVisitor {
    fn default() -> Self {
        Self::new()
    }
}
