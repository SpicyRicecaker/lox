use std::collections::HashMap;

use crate::{interpreter::Object, token::Token};

use self::error::{ErrorKind, RuntimeError};

pub mod error;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
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
    pub fn get(&self, name: &Token) -> Result<&Object> {
        if let Some(obj) = self.values.get(&name.lexeme) {
            Ok(obj)
        } else {
            Err(Box::new(RuntimeError::new(ErrorKind::UndefinedVariable(
                name.clone(),
            ))))
        }
    }
    pub fn assign(&mut self, name: &Token, obj: Object) -> Result<()> {
        if let Some(v) = self.values.get_mut(&name.lexeme) {
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
