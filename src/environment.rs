use anyhow::Result;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::error::RuntimeError;
use crate::grammar::{Literal, Token};

#[derive(Debug, Clone)]
pub struct Environment {
    inner: Rc<RefCell<EnvironmentImpl>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            inner: Rc::new(RefCell::new(EnvironmentImpl::new())),
        }
    }

    pub fn new_enclosed(enclosing: &Environment) -> Self {
        Environment {
            inner: Rc::new(RefCell::new(EnvironmentImpl {
                scope: HashMap::new(),
                enclosing: Some(enclosing.clone()),
            })),
        }
    }

    pub fn define(&self, name: &str, value: Literal) {
        self.inner.borrow_mut().define(name, value);
    }

    pub fn get(&self, token: &Token) -> Result<Literal> {
        self.inner.borrow().get(token)
    }

    pub fn assign(&self, token: &Token, value: &Literal) -> Result<()> {
        self.inner.borrow_mut().assign(token, value)
    }
}

#[derive(Debug)]
struct EnvironmentImpl {
    scope: HashMap<String, Literal>,
    enclosing: Option<Environment>,
}

impl EnvironmentImpl {
    fn new() -> Self {
        EnvironmentImpl {
            scope: HashMap::new(),
            enclosing: None,
        }
    }

    fn define(&mut self, name: &str, value: Literal) {
        self.scope.insert(name.to_string(), value);
    }

    pub fn get(&self, token: &Token) -> Result<Literal> {
        if let Some(value) = self.scope.get(&token.lexeme) {
            return Ok(value.clone());
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.get(token)
        } else {
            let err = RuntimeError::UndefinedVariable {
                lexeme: token.lexeme.to_string(),
                line: token.line,
            };
            Err(err.into())
        }
    }

    pub fn assign(&mut self, token: &Token, val: &Literal) -> Result<()> {
        if self.scope.contains_key(&token.lexeme) {
            self.scope.insert(token.lexeme.clone(), val.clone());
            Ok(())
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.assign(token, val)
        } else {
            let err = RuntimeError::UndefinedVariable {
                lexeme: token.lexeme.to_string(),
                line: token.line,
            };
            Err(err.into())
        }
    }
}
