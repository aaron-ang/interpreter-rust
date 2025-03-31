use anyhow::Result;
use std::{collections::HashMap, ops::ControlFlow, rc::Rc};

use crate::callable::{LoxClass, LoxFunction, Native};
use crate::environment::Environment;
use crate::error::RuntimeError;
use crate::grammar::*;

pub type InterpreterResult<T> = Result<T, RuntimeError>;

pub struct Interpreter {
    globals: Environment,
    env: Environment,
    locals: HashMap<usize, usize>,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl Interpreter {
    pub fn new() -> Self {
        let env = Environment::new();
        env.define("clock", Literal::Function(Native::clock()));

        Interpreter {
            globals: env.clone(),
            env,
            locals: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, statements: &[Statement]) -> Result<Literal> {
        for statement in statements.iter() {
            if let ControlFlow::Break(rv) = self.execute(statement)? {
                return Ok(rv);
            }
        }
        Ok(Literal::Nil)
    }

    pub fn execute(&mut self, statement: &Statement) -> InterpreterResult<ControlFlow<Literal>> {
        match statement {
            Statement::Block(statements) => {
                let env = Environment::new_enclosed(&self.env);
                self.execute_block(statements, env)
            }
            Statement::Class { name, .. } => {
                self.env.define(&name.lexeme, Literal::Nil);
                let klass = LoxClass::new(name.lexeme.clone());
                let klass_literal = Literal::Class(Rc::new(klass));
                self.env.assign(name, &klass_literal)?;
                Ok(ControlFlow::Continue(()))
            }
            Statement::Expression(expr) => {
                self.evaluate(expr)?;
                Ok(ControlFlow::Continue(()))
            }
            Statement::If {
                condition,
                then_branch,
                else_branch,
            } => {
                if self.evaluate(condition)?.is_truthy() {
                    self.execute(then_branch)
                } else if let Some(else_branch) = else_branch {
                    self.execute(else_branch)
                } else {
                    Ok(ControlFlow::Continue(()))
                }
            }
            Statement::Print(expr) => {
                match self.evaluate(expr)? {
                    Literal::Number(n) => println!("{n}"),
                    val => println!("{val}"),
                }
                Ok(ControlFlow::Continue(()))
            }
            Statement::Variable { name, init } => {
                let value = if let Some(expr) = init {
                    self.evaluate(expr)?
                } else {
                    Literal::Nil
                };
                self.env.define(&name.lexeme, value);
                Ok(ControlFlow::Continue(()))
            }
            Statement::While { condition, body } => {
                while self.evaluate(condition)?.is_truthy() {
                    if let ControlFlow::Break(rv) = self.execute(body)? {
                        return Ok(ControlFlow::Break(rv));
                    }
                }
                Ok(ControlFlow::Continue(()))
            }
            Statement::Function { name, params, body } => {
                let fun = LoxFunction::new(name, params, body, &self.env);
                let fun_literal = Literal::Function(Rc::new(fun));
                self.env.define(&name.lexeme, fun_literal);
                Ok(ControlFlow::Continue(()))
            }
            Statement::Return { value, .. } => {
                let rv = if let Some(expr) = value {
                    self.evaluate(expr)?
                } else {
                    Literal::Nil
                };
                Ok(ControlFlow::Break(rv))
            }
        }
    }

    pub fn evaluate(&mut self, expr: &Expression) -> InterpreterResult<Literal> {
        let literal = match expr {
            Expression::Assign { id, name, value } => {
                let value = self.evaluate(value)?;
                if let Some(distance) = self.locals.get(id) {
                    self.env.assign_at(*distance, &name.lexeme, &value)?;
                } else {
                    self.globals.assign(name, &value)?;
                }
                value
            }
            Expression::Binary { left, op, right } => {
                let left = self.evaluate(left)?;
                let right = self.evaluate(right)?;
                match op.token_type {
                    TokenType::STAR => match (left, right) {
                        (Literal::Number(l), Literal::Number(r)) => Literal::Number(l * r),
                        _ => return Err(self.type_error("Operands must be numbers.")),
                    },
                    TokenType::SLASH => match (left, right) {
                        (Literal::Number(l), Literal::Number(r)) => Literal::Number(l / r),
                        _ => return Err(self.type_error("Operands must be numbers.")),
                    },
                    TokenType::PLUS => match (left, right) {
                        (Literal::Number(l), Literal::Number(r)) => Literal::Number(l + r),
                        (Literal::String(l), Literal::String(r)) => {
                            Literal::String(format!("{l}{r}"))
                        }
                        _ => return Err(self.type_error("Operands must be numbers or strings.")),
                    },
                    TokenType::MINUS => match (left, right) {
                        (Literal::Number(l), Literal::Number(r)) => Literal::Number(l - r),
                        _ => return Err(self.type_error("Operands must be numbers.")),
                    },
                    TokenType::LESS
                    | TokenType::LESS_EQUAL
                    | TokenType::GREATER
                    | TokenType::GREATER_EQUAL => match (left, right) {
                        (Literal::Number(l), Literal::Number(r)) => {
                            Literal::Boolean(compare_number(&op.token_type, l, r))
                        }
                        _ => return Err(self.type_error("Operands must be numbers.")),
                    },
                    TokenType::EQUAL_EQUAL => Literal::Boolean(left == right),
                    TokenType::BANG_EQUAL => Literal::Boolean(left != right),
                    _ => unimplemented!(),
                }
            }
            Expression::Call { callee, arguments } => {
                let callee = self.evaluate(callee)?;
                let args = arguments
                    .iter()
                    .map(|arg| self.evaluate(arg))
                    .collect::<InterpreterResult<Vec<_>>>()?;
                let callee = match callee {
                    Literal::Function(fun) => fun,
                    Literal::Class(klass) => klass,
                    _ => return Err(self.type_error("Can only call functions and classes.")),
                };
                if args.len() != callee.arity() {
                    return Err(RuntimeError::ArgumentCountError {
                        expected: callee.arity(),
                        got: args.len(),
                    });
                }
                callee.call(self, &args)?
            }
            Expression::Grouping(expr) => self.evaluate(expr)?,
            Expression::Literal(l) => l.clone(),
            Expression::Logical { left, op, right } => {
                let left = self.evaluate(left)?;
                let left_truthy = left.is_truthy();
                let eval_right = match op.token_type {
                    TokenType::OR => !left_truthy,
                    TokenType::AND => left_truthy,
                    _ => unreachable!(),
                };
                if eval_right {
                    self.evaluate(right)?
                } else {
                    left
                }
            }
            Expression::Unary { op, right } => {
                let literal = self.evaluate(right)?;
                match op.token_type {
                    TokenType::BANG => Literal::Boolean(!literal.is_truthy()),
                    TokenType::MINUS => match literal {
                        Literal::Number(n) => Literal::Number(-n),
                        _ => return Err(self.type_error("Operand must be a number.")),
                    },
                    _ => unreachable!(),
                }
            }
            Expression::Variable { id, name } => self.lookup_variable(name, id)?,
            Expression::Get { object, name } => {
                let object = self.evaluate(object)?;
                match object {
                    Literal::Instance(instance) => instance.borrow().get(name)?,
                    _ => return Err(self.type_error("Only instances have fields.")),
                }
            }
            Expression::Set {
                object,
                name,
                value,
            } => {
                let object = self.evaluate(object)?;
                match object {
                    Literal::Instance(instance) => {
                        let value = self.evaluate(value)?;
                        instance.borrow_mut().set(name, value.clone());
                        return Ok(value);
                    }
                    _ => return Err(self.type_error("Only instances have fields.")),
                }
            }
        };
        Ok(literal)
    }

    pub fn execute_block(
        &mut self,
        statements: &[Statement],
        env: Environment,
    ) -> InterpreterResult<ControlFlow<Literal>> {
        let previous_env = std::mem::replace(&mut self.env, env);
        for statement in statements {
            if let ControlFlow::Break(rv) = self.execute(statement)? {
                self.env = previous_env;
                return Ok(ControlFlow::Break(rv));
            }
        }
        self.env = previous_env;
        Ok(ControlFlow::Continue(()))
    }

    pub fn resolve(&mut self, exp_id: usize, depth: usize) {
        self.locals.insert(exp_id, depth);
    }

    fn lookup_variable(&mut self, name: &Token, exp_id: &usize) -> InterpreterResult<Literal> {
        if let Some(depth) = self.locals.get(exp_id) {
            self.env.get_at(*depth, &name.lexeme)
        } else {
            self.globals.get(name)
        }
    }

    fn type_error(&self, message: &str) -> RuntimeError {
        RuntimeError::TypeError(message.to_string())
    }
}

fn compare_number(op: &TokenType, l: f64, r: f64) -> bool {
    match op {
        TokenType::EQUAL_EQUAL => l == r,
        TokenType::BANG_EQUAL => l != r,
        TokenType::LESS => l < r,
        TokenType::LESS_EQUAL => l <= r,
        TokenType::GREATER => l > r,
        TokenType::GREATER_EQUAL => l >= r,
        _ => unreachable!(),
    }
}
