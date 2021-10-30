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
    pub fn get(&self, id: usize) -> Option<&Node<T>> {
        self.arena.get(id)
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
}

impl Node<Environment> {
    pub fn define(&mut self, name: &str, obj: Object) {
        self.val.values.insert(name.to_string(), obj);
    }
    /// Lookup has to be recursive and look at all parents (enclosing scopes)
    pub fn get<'a>(&'a self, name: &Token, arena: &'a Arena<Environment>) -> Result<&Object> {
        // First get the current environment reference, from the arena
        // Next check if the current environment holds such a name
        if let Some(n) = self.val.values.get(&name.lexeme) {
            Ok(n)
        // otherwise, recurse with the parent
        } else if let Some(p) = self.parent {
            arena.get(p).unwrap().get(name, arena)
        } else {
            Err(Box::new(RuntimeError::new(ErrorKind::UndefinedVariable(
                name.clone(),
            ))))
        }
    }
    /// This func is essentially the same as `.get()` except we don't return anything so we don't have to worry about lifetimes
    pub fn assign(
        &mut self,
        name: &Token,
        obj: Object,
        arena: &mut Arena<Environment>,
    ) -> Result<()> {
        // first check if the current environment holds the variable
        if let Some(enclosing) = self.val.values.get_mut(&name.lexeme) {
            *enclosing = obj;
            Ok(())
        } else if let Some(p) = self.parent {
            arena.get(p).unwrap().assign(name, obj, arena)
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
