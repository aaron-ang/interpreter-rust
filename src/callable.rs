use anyhow::Result;

use crate::error::RuntimeError;
use crate::grammar::{Literal, Statement, Token};
use crate::interpreter::Interpreter;

pub trait LoxCallable {
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Literal>) -> Result<Literal>;
    fn to_string(&self) -> String;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Callable {
    Native {
        arity: usize,
        call: fn(&mut Interpreter, Vec<Literal>) -> Result<Literal>,
        to_string: fn() -> String,
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

    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Literal>) -> Result<Literal> {
        match self {
            Callable::Native { call, .. } => call(interpreter, arguments),
            Callable::Function(f) => {
                let mut env = interpreter.globals();
                for (param, arg) in f.params.iter().zip(arguments) {
                    env.define(&param.lexeme, arg);
                }
                let result = interpreter.execute_block_with_env(&f.body, env);
                match result {
                    Err(e) => {
                        if let Some(RuntimeError::Return(value)) = e.downcast_ref() {
                            Ok(value.clone())
                        } else {
                            Err(e)
                        }
                    }
                    _ => Ok(Literal::Nil),
                }
            }
        }
    }

    fn to_string(&self) -> String {
        match self {
            Callable::Native { to_string, .. } => to_string(),
            Callable::Function(f) => format!("<fn {}>", f.name.lexeme),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: Token,
    params: Vec<Token>,
    body: Vec<Statement>,
}

impl Function {
    pub fn new(name: Token, params: Vec<Token>, body: Vec<Statement>) -> Self {
        Self { name, params, body }
    }
}
