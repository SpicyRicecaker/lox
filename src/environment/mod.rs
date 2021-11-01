use std::collections::HashMap;

use crate::{interpreter::Object, token::Token};

use self::error::{ErrorKind, RuntimeError};

pub mod error;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Code following tutorial from https://dev.to/deciduously/no-more-tears-no-more-knots-arena-allocated-trees-in-rust-44k6
#[derive(Debug)]
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
    pub fn new() -> Self {
        Self { arena: Vec::new() }
    }
    pub fn get(&self, id: usize) -> Option<&Node<T>> {
        self.arena.get(id)
    }
    pub fn pop(&mut self) {
        self.arena.pop();
    }
    pub fn remove(&mut self, id: usize) {
        self.arena.remove(id);
    }
    pub fn get_mut(&mut self, id: usize) -> Option<&mut Node<T>> {
        self.arena.get_mut(id)
    }
    pub fn push(&mut self, val: T) -> usize {
        let idx = self.arena.len();

        self.push_node(Node {
            idx,
            val,
            parent: None,
        });

        idx
    }
    fn push_node(&mut self, node: Node<T>) {
        self.arena.push(node)
    }
}

impl<T> Default for Arena<T>
where
    T: PartialEq,
{
    fn default() -> Self {
        Self::new()
    }
}

/// Define a wrapper around `Arena<T>`, since the above implementation is pretty widespread
/// We call it `Cactus` (short for `CactusStack`), a name for `Parent-Pointer Tree`
#[derive(Debug)]
pub struct Cactus {
    pub arena: Arena<Environment>,
    pub cur_env: usize,
}

impl Cactus {
    pub fn new() -> Self {
        let mut arena = Arena::new();
        let cur_env = arena.push(Environment::new());
        // dbg!(&arena);
        Cactus { arena, cur_env }
    }

    pub fn define(&mut self, name: &str, obj: Object, cur_env: usize) {
        // dbg!(&self);
        let node = self.arena.get_mut(cur_env).unwrap();
        node.define(name, obj);
    }

    /// Lookup has to be recursive and look at all parents (enclosing scopes)
    /// TODO could get rid of the `.unwrap` to make it more idiomatic
    pub fn get(&self, name: &Token, cur_env: usize) -> Result<&Object> {
        // First get the current environment reference, from the arena
        // Next check if the current environment holds such a name
        // get current env
        let env = self.arena.get(cur_env).unwrap();
        if let Some(n) = env.val.values.get(&name.lexeme) {
            Ok(n)
        // otherwise, recurse with the parent
        } else if let Some(p) = env.parent {
            // unwrap here, because we assume that arena does hold the parent
            self.get(name, p)
        } else {
            Err(Box::new(RuntimeError::new(ErrorKind::UndefinedVariable(
                name.clone(),
            ))))
        }
    }
    /// This func is essentially the same as `.get()` except we don't return anything so we don't have to worry about lifetimes
    pub fn assign(&mut self, name: &Token, obj: Object, cur_env: usize) -> Result<()> {
        let env = self.arena.get_mut(cur_env).unwrap();
        // first check if the current environment holds the variable
        if let Some(enclosing) = env.val.values.get_mut(&name.lexeme) {
            *enclosing = obj;
            Ok(())
        } else if let Some(p) = env.parent {
            // unwrap, because we assume arena holds the parent
            self.assign(name, obj, p)
        } else {
            Err(Box::new(RuntimeError::new(ErrorKind::UndefinedVariable(
                name.clone(),
            ))))
        }
    }
}

impl Default for Cactus {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, PartialEq)]
pub struct Node<T> {
    idx: usize,
    pub val: T,
    pub parent: Option<usize>,
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
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}
