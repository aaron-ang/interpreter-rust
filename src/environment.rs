use anyhow::Result;
use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};

use crate::error::RuntimeError;
use crate::grammar::{Literal, Token};

#[derive(Debug, Clone)]
pub struct Environment {
    inner: Rc<RefCell<EnvironmentImpl>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            inner: Rc::new(RefCell::new(EnvironmentImpl::new())),
        }
    }

    pub fn new_enclosed(enclosing: &Self) -> Self {
        Self {
            inner: Rc::new(RefCell::new(EnvironmentImpl::enclose(
                enclosing.inner.clone(),
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
        self.ancestor(distance).borrow().get_no_parent(name)
    }

    pub fn assign_at(&self, distance: usize, name: &str, value: &Literal) -> Result<()> {
        Ok(self
            .ancestor(distance)
            .borrow_mut()
            .assign_no_parent(name, value))
    }

    fn ancestor(&self, distance: usize) -> Rc<RefCell<EnvironmentImpl>> {
        let mut env = self.inner.clone();
        for _ in 0..distance {
            let next_env = env
                .borrow()
                .enclosing
                .as_ref()
                .expect("No enclosing environment found")
                .clone();
            env = next_env;
        }
        env
    }
}

#[derive(Debug)]
struct EnvironmentImpl {
    values: HashMap<String, Literal>,
    enclosing: Option<Rc<RefCell<EnvironmentImpl>>>,
}

impl EnvironmentImpl {
    fn new() -> Self {
        Self {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    fn enclose(inner: Rc<RefCell<EnvironmentImpl>>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: Some(inner),
        }
    }

    fn define(&mut self, name: &str, value: Literal) {
        self.values.insert(name.to_string(), value);
    }

    fn get(&self, token: &Token) -> Result<Literal> {
        if let Some(value) = self.values.get(&token.lexeme) {
            return Ok(value.clone());
        }

        if let Some(enclosing) = &self.enclosing {
            return enclosing.borrow().get(token);
        }

        Err(RuntimeError::UndefinedVariable {
            lexeme: token.lexeme.clone(),
            line: token.line,
        }
        .into())
    }

    fn get_no_parent(&self, name: &str) -> Result<Literal> {
        Ok(self.values.get(name).cloned().unwrap_or(Literal::Nil))
    }

    fn assign(&mut self, token: &Token, val: &Literal) -> Result<()> {
        if self.values.contains_key(&token.lexeme) {
            self.values.insert(token.lexeme.clone(), val.clone());
            return Ok(());
        }

        if let Some(enclosing) = &self.enclosing {
            return enclosing.borrow_mut().assign(token, val);
        }

        Err(RuntimeError::UndefinedVariable {
            lexeme: token.lexeme.clone(),
            line: token.line,
        }
        .into())
    }

    fn assign_no_parent(&mut self, name: &str, val: &Literal) {
        self.values.insert(name.to_string(), val.clone());
    }
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let env = self.inner.borrow();
        write!(f, "{:?}", env.values)?;

        if let Some(ref enclosing) = env.enclosing {
            write!(f, " -> {:?}", enclosing)?;
        }

        Ok(())
    }
}
