use anyhow::Result;
use std::{collections::HashMap, ops::ControlFlow, rc::Rc};

use crate::{
    callable::{LoxClass, LoxFunction, Native},
    constants::{errors::*, INIT_METHOD, SUPER_KEYWORD, THIS_KEYWORD},
    environment::Environment,
    error::LoxError,
    grammar::{Expression, Literal, Statement, Token, TokenType},
};

pub type InterpreterResult<T> = Result<T, LoxError>;

pub struct Interpreter {
    globals: Environment,
    environment: Environment,
    locals: HashMap<usize, usize>,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl Interpreter {
    pub fn new() -> Self {
        let environment = Environment::new();
        environment.define("clock", Literal::Function(Native::clock()));

        Interpreter {
            globals: environment.clone(),
            environment,
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
                let env = Environment::new_enclosed(&self.environment);
                self.execute_block(statements, env)
            }
            Statement::Class {
                name,
                superclass,
                methods,
            } => {
                let superclass = if let Some(expr) = superclass {
                    match self.evaluate(expr)? {
                        Literal::Class(class) => Some(class),
                        _ => return Err(self.type_error("Superclass must be a class.")),
                    }
                } else {
                    None
                };

                self.environment.define(&name.lexeme, Literal::Nil);

                if let Some(ref superclass) = superclass {
                    self.environment = Environment::new_enclosed(&self.environment);
                    self.environment
                        .define(SUPER_KEYWORD, Literal::Class(superclass.clone()));
                }

                let methods = methods
                    .iter()
                    .map(|method| {
                        let is_initializer = method.name.lexeme == INIT_METHOD;
                        let function =
                            Rc::new(LoxFunction::new(method, &self.environment, is_initializer));
                        (method.name.lexeme.clone(), function)
                    })
                    .collect();

                if superclass.is_some() {
                    self.environment = self.environment.ancestor(1);
                }
                let class = Rc::new(LoxClass::new(name.lexeme.clone(), superclass, methods));
                self.environment.assign(name, &Literal::Class(class))?;

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
                self.environment.define(&name.lexeme, value);
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
            Statement::Function(fun) => {
                let fun = LoxFunction::new(fun, &self.environment, false);
                let name = fun.name();
                self.environment
                    .define(&name, Literal::Function(Rc::new(fun)));
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

    pub fn execute_block(
        &mut self,
        statements: &[Statement],
        env: Environment,
    ) -> InterpreterResult<ControlFlow<Literal>> {
        let previous_env = std::mem::replace(&mut self.environment, env);
        for statement in statements {
            if let ControlFlow::Break(rv) = self.execute(statement)? {
                self.environment = previous_env;
                return Ok(ControlFlow::Break(rv));
            }
        }
        self.environment = previous_env;
        Ok(ControlFlow::Continue(()))
    }

    pub fn evaluate(&mut self, expr: &Expression) -> InterpreterResult<Literal> {
        let literal = match expr {
            Expression::Assign { id, name, value } => {
                let value = self.evaluate(value)?;
                if let Some(distance) = self.locals.get(id) {
                    self.environment
                        .assign_at(*distance, &name.lexeme, &value)?;
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
                        _ => return Err(self.type_error(OPERANDS_MUST_BE_NUMBERS)),
                    },
                    TokenType::SLASH => match (left, right) {
                        (Literal::Number(l), Literal::Number(r)) => Literal::Number(l / r),
                        _ => return Err(self.type_error(OPERANDS_MUST_BE_NUMBERS)),
                    },
                    TokenType::PLUS => match (left, right) {
                        (Literal::Number(l), Literal::Number(r)) => Literal::Number(l + r),
                        (Literal::String(l), Literal::String(r)) => {
                            Literal::String(format!("{l}{r}"))
                        }
                        _ => return Err(self.type_error(OPERANDS_MUST_BE_NUMBERS_OR_STRINGS)),
                    },
                    TokenType::MINUS => match (left, right) {
                        (Literal::Number(l), Literal::Number(r)) => Literal::Number(l - r),
                        _ => return Err(self.type_error(OPERANDS_MUST_BE_NUMBERS)),
                    },
                    TokenType::LESS
                    | TokenType::LESS_EQUAL
                    | TokenType::GREATER
                    | TokenType::GREATER_EQUAL => match (left, right) {
                        (Literal::Number(l), Literal::Number(r)) => {
                            Literal::Boolean(compare_number(&op.token_type, l, r))
                        }
                        _ => return Err(self.type_error(OPERANDS_MUST_BE_NUMBERS)),
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
                    return Err(LoxError::ArgumentCountError {
                        expected: callee.arity(),
                        got: args.len(),
                    });
                }
                callee.call(self, &args)?
            }
            Expression::Get { object, name } => {
                let object = self.evaluate(object)?;
                match object {
                    Literal::Instance(instance) => instance.borrow().get(name)?,
                    _ => return Err(self.type_error(ONLY_INSTANCES_HAVE_FIELDS)),
                }
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
                    _ => return Err(self.type_error(ONLY_INSTANCES_HAVE_FIELDS)),
                }
            }
            Expression::Super {
                id,
                keyword: _,
                method,
            } => {
                let distance = *self.locals.get(id).expect("Super expression not resolved");

                let Literal::Class(superclass) = self.environment.get_at(distance, "super")? else {
                    return Err(self.type_error("Super reference must be a class."));
                };

                let Literal::Instance(object) =
                    self.environment.get_at(distance - 1, THIS_KEYWORD)?
                else {
                    return Err(self.type_error("'this' reference must be an instance."));
                };

                let Some(method) = superclass.find_method(&method.lexeme) else {
                    let err_msg = format!("Undefined property '{}'.", method.lexeme);
                    return Err(self.type_error(&err_msg));
                };

                Literal::Function(Rc::new(method.bind(object)?))
            }
            Expression::This { id, keyword } => self.lookup_variable(id, keyword)?,
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
            Expression::Variable { id, name } => self.lookup_variable(id, name)?,
        };
        Ok(literal)
    }

    pub fn resolve(&mut self, exp_id: usize, depth: usize) {
        self.locals.insert(exp_id, depth);
    }

    fn lookup_variable(&mut self, exp_id: &usize, name: &Token) -> InterpreterResult<Literal> {
        if let Some(depth) = self.locals.get(exp_id) {
            self.environment.get_at(*depth, &name.lexeme)
        } else {
            self.globals.get(name)
        }
    }

    fn type_error(&self, message: &str) -> LoxError {
        LoxError::TypeError(message.to_string())
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
