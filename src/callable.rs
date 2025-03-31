use std::{
    cell::RefCell,
    collections::HashMap,
    fmt,
    ops::ControlFlow,
    rc::Rc,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::error::RuntimeError;
use crate::grammar::{Literal, Statement, Token};
use crate::interpreter::Interpreter;
use crate::{environment::Environment, interpreter::InterpreterResult};

pub trait LoxCallable: fmt::Debug {
    fn arity(&self) -> usize;
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: &[Literal],
    ) -> InterpreterResult<Literal>;
    fn to_string(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct LoxFunction {
    pub name: Token,
    params: Vec<Token>,
    body: Vec<Statement>,
    closure: Environment,
}

impl LoxFunction {
    pub fn new(name: &Token, params: &[Token], body: &[Statement], closure: &Environment) -> Self {
        Self {
            name: name.clone(),
            params: params.to_vec(),
            body: body.to_vec(),
            closure: closure.clone(),
        }
    }

    fn execute(
        &self,
        interpreter: &mut Interpreter,
        arguments: &[Literal],
    ) -> InterpreterResult<Literal> {
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

impl LoxCallable for LoxFunction {
    fn arity(&self) -> usize {
        self.params.len()
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: &[Literal],
    ) -> InterpreterResult<Literal> {
        self.execute(interpreter, arguments)
    }

    fn to_string(&self) -> String {
        format!("<fn {}>", self.name.lexeme)
    }
}

#[derive(Debug, Clone)]
pub struct Native {
    arity: usize,
    call: fn(&mut Interpreter, &[Literal]) -> InterpreterResult<Literal>,
}

impl Native {
    pub fn clock() -> Rc<dyn LoxCallable> {
        let clock_fn = Self {
            arity: 0,
            call: |_, _| {
                let start = SystemTime::now();
                let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap();
                Ok(Literal::Number(since_the_epoch.as_secs_f64()))
            },
        };
        Rc::new(clock_fn)
    }
}

impl LoxCallable for Native {
    fn arity(&self) -> usize {
        self.arity
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: &[Literal],
    ) -> InterpreterResult<Literal> {
        (self.call)(interpreter, arguments)
    }

    fn to_string(&self) -> String {
        "<native fn>".to_string()
    }
}

#[derive(Debug, Clone)]
pub struct LoxClass {
    pub name: String,
}

impl LoxClass {
    pub fn new(name: String) -> Self {
        LoxClass { name }
    }
}

impl LoxCallable for LoxClass {
    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: &[Literal],
    ) -> InterpreterResult<Literal> {
        let instance = LoxInstance::new(self.clone());
        Ok(Literal::Instance(Rc::new(RefCell::new(instance))))
    }

    fn to_string(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug, Clone)]
pub struct LoxInstance {
    klass: LoxClass,
    fields: HashMap<String, Literal>,
}

impl LoxInstance {
    fn new(klass: LoxClass) -> Self {
        LoxInstance {
            klass,
            fields: HashMap::new(),
        }
    }

    pub fn get(&self, name: &Token) -> InterpreterResult<Literal> {
        if let Some(value) = self.fields.get(&name.lexeme) {
            return Ok(value.clone());
        }

        Err(RuntimeError::UndefinedProperty(name.lexeme.clone()))
    }

    pub fn set(&mut self, name: &Token, value: Literal) {
        self.fields.insert(name.lexeme.clone(), value);
    }
}

impl LoxCallable for LoxInstance {
    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: &[Literal],
    ) -> InterpreterResult<Literal> {
        let err_msg = format!("'{}' object is not callable", self.klass.name);
        Err(RuntimeError::TypeError(err_msg))
    }

    fn to_string(&self) -> String {
        format!("{} instance", self.klass.name)
    }
}
