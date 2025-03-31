use anyhow::Result;
use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};

use crate::error::RuntimeError;
use crate::grammar::{Literal, Token};

/// `Environment` represents a variable scope.
/// Uses `Rc<RefCell<>>` to allow multiple references to the same environment
/// while still enabling mutation through these references.
#[derive(Debug, Clone)]
pub struct Environment {
    inner: Rc<RefCell<EnvironmentImpl>>,
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

impl Environment {
    pub fn new() -> Self {
        Self {
            inner: Rc::new(RefCell::new(EnvironmentImpl::new())),
        }
    }

    pub fn new_enclosed(enclosing: &Self) -> Self {
        Self {
            inner: Rc::new(RefCell::new(EnvironmentImpl::with_parent(
                enclosing.clone(),
            ))),
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

    pub fn get_at(&self, distance: usize, name: &str) -> Result<Literal> {
        self.ancestor(distance).inner.borrow().get_from_local(name)
    }

    pub fn assign_at(&self, distance: usize, name: &str, value: &Literal) -> Result<()> {
        self.ancestor(distance)
            .inner
            .borrow_mut()
            .assign_to_local(name, value);
        Ok(())
    }

    fn ancestor(&self, distance: usize) -> Environment {
        let mut env = self.clone();
        for _ in 0..distance {
            let parent = env
                .inner
                .borrow()
                .parent
                .as_ref()
                .expect("No parent environment found")
                .clone();
            env = parent;
        }
        env
    }
}

/// Contains the actual environment implementation and references to parent environments
#[derive(Debug)]
struct EnvironmentImpl {
    values: HashMap<String, Literal>,
    parent: Option<Environment>,
}

impl EnvironmentImpl {
    fn new() -> Self {
        Self {
            values: HashMap::new(),
            parent: None,
        }
    }

    fn with_parent(parent: Environment) -> Self {
        Self {
            values: HashMap::new(),
            parent: Some(parent),
        }
    }

    fn define(&mut self, name: &str, value: Literal) {
        self.values.insert(name.to_string(), value);
    }

    fn get(&self, token: &Token) -> Result<Literal> {
        if let Some(value) = self.values.get(&token.lexeme) {
            return Ok(value.clone());
        }

        if let Some(parent) = &self.parent {
            return parent.get(token);
        }

        Err(RuntimeError::UndefinedVariable {
            lexeme: token.lexeme.clone(),
            line: token.line,
        }
        .into())
    }

    fn get_from_local(&self, name: &str) -> Result<Literal> {
        Ok(self.values.get(name).cloned().unwrap_or(Literal::Nil))
    }

    fn assign(&mut self, token: &Token, value: &Literal) -> Result<()> {
        if self.values.contains_key(&token.lexeme) {
            self.assign_to_local(&token.lexeme, value);
            return Ok(());
        }

        if let Some(parent) = &self.parent {
            return parent.assign(token, value);
        }

        Err(RuntimeError::UndefinedVariable {
            lexeme: token.lexeme.clone(),
            line: token.line,
        }
        .into())
    }

    fn assign_to_local(&mut self, name: &str, value: &Literal) {
        self.values.insert(name.to_string(), value.clone());
    }
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let env = self.inner.borrow();
        write!(f, "{:?}", env.values)?;

        if let Some(ref parent) = env.parent {
            write!(f, " -> {:?}", parent)?;
        }

        Ok(())
    }
}
