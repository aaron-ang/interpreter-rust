use anyhow::Result;
use std::{fmt, ops::ControlFlow};

use crate::environment::Environment;
use crate::grammar::{Literal, Statement, Token};
use crate::interpreter::Interpreter;

pub trait LoxCallable: fmt::Debug {
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &mut Interpreter, arguments: &[Literal]) -> Result<Literal>;
    fn to_string(&self) -> String;
}

#[derive(Debug, Clone)]
pub enum Callable {
    Native {
        arity: usize,
        call: fn(&mut Interpreter, &[Literal]) -> Result<Literal>,
    },
    Function(Function),
}

impl LoxCallable for Callable {
    fn arity(&self) -> usize {
        match self {
            Callable::Native { arity, .. } => *arity,
            Callable::Function(f) => f.params.len(),
        }
    }

    fn call(&self, interpreter: &mut Interpreter, arguments: &[Literal]) -> Result<Literal> {
        match self {
            Callable::Native { call, .. } => call(interpreter, arguments),
            Callable::Function(func) => func.execute(interpreter, arguments),
        }
    }

    fn to_string(&self) -> String {
        match self {
            Callable::Native { .. } => "<native fn>".to_string(),
            Callable::Function(f) => format!("<fn {}>", f.name.lexeme),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: Token,
    params: Vec<Token>,
    body: Vec<Statement>,
    closure: Environment,
}

impl Function {
    pub fn new(name: &Token, params: &[Token], body: &[Statement], closure: &Environment) -> Self {
        Self {
            name: name.clone(),
            params: params.to_vec(),
            body: body.to_vec(),
            closure: closure.clone(),
        }
    }

    fn execute(&self, interpreter: &mut Interpreter, arguments: &[Literal]) -> Result<Literal> {
        let env = Environment::new_enclosed(&self.closure);
        // Bind parameters to arguments
        for (param, arg) in self.params.iter().zip(arguments) {
            env.define(&param.lexeme, arg.clone());
        }
        // Execute function body in the new environment
        match interpreter.execute_block(&self.body, env)? {
            ControlFlow::Break(value) => Ok(value),
            ControlFlow::Continue(()) => Ok(Literal::Nil),
        }
    }
}
