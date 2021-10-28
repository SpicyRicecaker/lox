use std::{
    borrow::Borrow,
    cell::{Ref, RefCell},
    collections::HashMap,
    rc::Rc,
};

use crate::{interpreter::Object, token::Token};

use self::error::{ErrorKind, RuntimeError};

pub mod error;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Code following tutorial from https://dev.to/deciduously/no-more-tears-no-more-knots-arena-allocated-trees-in-rust-44k6
pub struct Arena<T>
where
    T: PartialEq,
{
    arena: Vec<Node<T>>,
}

impl<T> Arena<T>
where
    T: PartialEq,
{
    pub fn get(&self, id: usize) -> &Node<T> {
        &self.arena[id]
    }
    pub fn get_mut(&mut self, id: usize) -> &mut Node<T> {
        &mut self.arena[id]
    }
    pub fn push_new_node(&mut self, val: T) -> usize {
        let idx = self.arena.len();

        self.push(Node {
            idx,
            val,
            parent: None,
        });

        idx
    }
    fn push(&mut self, node: Node<T>) {
        self.arena.push(node)
    }
}

pub struct Node<T> {
    idx: usize,
    pub val: T,
    parent: Option<usize>,
}

#[derive(Debug, PartialEq)]
pub struct Environment {
    values: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
        }
    }
    pub fn define(&mut self, name: &str, obj: Object) {
        self.values.insert(name.to_string(), obj);
    }
    /// Lookup has to be recursive and look at all parents (enclosing scopes)
    pub fn get(&self, name: &Token) -> Result<&Object> {
        if let Some(id) = &self.parent {
            arena.nodes[*id].get(name, arena)
        } else if let Some(obj) = self.values.get(&name.lexeme) {
            Ok(obj)
        } else {
            Err(Box::new(RuntimeError::new(ErrorKind::UndefinedVariable(
                name.clone(),
            ))))
        }
    }
    pub fn assign(&mut self, name: &Token, obj: Object) -> Result<()> {
        if let Some(enclosing) = &mut self.enclosing {
            enclosing.borrow_mut().assign(name, obj);
            Ok(())
        } else if let Some(v) = self.values.get_mut(&name.lexeme) {
            *v = obj;
            Ok(())
        } else {
            Err(Box::new(RuntimeError::new(ErrorKind::UndefinedVariable(
                name.clone(),
            ))))
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}
