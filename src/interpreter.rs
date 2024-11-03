use std::collections::HashMap;

use crate::grammar::*;

pub struct Environment {
    scopes: Vec<HashMap<String, Literal>>,
}

impl Environment {
    fn new() -> Self {
        Environment {
            scopes: vec![HashMap::new()],
        }
    }

    fn declare(&mut self, name: String, value: Literal) {
        self.scopes.last_mut().unwrap().insert(name, value);
    }

    fn get(&self, name: &String) -> Option<Literal> {
        for scope in self.scopes.iter().rev() {
            if let Some(val) = scope.get(name) {
                return Some(val.clone());
            }
        }
        None
    }

    fn set(&mut self, name: &String, val: &Literal) -> bool {
        for scope in self.scopes.iter_mut().rev() {
            if scope.contains_key(name) {
                scope.insert(name.clone(), val.clone());
                return true;
            }
        }
        false
    }

    fn push(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn pop(&mut self) {
        self.scopes.pop();
    }
}

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environment: Environment::new(),
        }
    }

    pub fn interpret(&mut self, statements: Vec<Statement>) -> Result<(), &'static str> {
        for statement in statements.iter() {
            self.execute(statement)?;
        }
        Ok(())
    }

    fn execute(&mut self, statement: &Statement) -> Result<(), &'static str> {
        match statement {
            Statement::Block(statements) => {
                self.execute_block(statements)?;
            }

            Statement::Expression(expr) => {
                self.evaluate(expr)?;
            }
            Statement::If {
                condition,
                then_branch,
                else_branch,
            } => {
                if self.evaluate(condition)?.is_truthy() {
                    self.execute(then_branch)?;
                } else if let Some(else_branch) = else_branch {
                    self.execute(else_branch)?;
                }
            }
            Statement::Print(expr) => match self.evaluate(expr)? {
                Literal::Number(n) => println!("{}", n),
                val => println!("{}", val),
            },
            Statement::Variable { name, init } => {
                let value = match init {
                    Some(expr) => self.evaluate(expr)?,
                    None => Literal::Nil,
                };
                self.environment.declare(name.lexeme.clone(), value);
            }
            Statement::While { condition, body } => {
                while self.evaluate(condition)?.is_truthy() {
                    self.execute(body)?;
                }
            }
        }
        Ok(())
    }

    pub fn evaluate(&mut self, expr: &Expression) -> Result<Literal, &'static str> {
        let literal = match expr {
            Expression::Assign { name, value } => {
                let value = self.evaluate(value)?;
                self.assign_variable(name, &value)?;
                value
            }
            Expression::Binary { left, op, right } => {
                let left = self.evaluate(left)?;
                let right = self.evaluate(right)?;
                match op.token_type {
                    TokenType::STAR => match (left, right) {
                        (Literal::Number(l), Literal::Number(r)) => Literal::Number(l * r),
                        _ => return Err("Operands must be numbers."),
                    },
                    TokenType::SLASH => match (left, right) {
                        (Literal::Number(l), Literal::Number(r)) => Literal::Number(l / r),
                        _ => return Err("Operands must be numbers."),
                    },
                    TokenType::PLUS => match (left, right) {
                        (Literal::Number(l), Literal::Number(r)) => Literal::Number(l + r),
                        (Literal::String(l), Literal::String(r)) => {
                            Literal::String(format!("{}{}", l, r))
                        }
                        _ => return Err("Operands must be two numbers or two strings."),
                    },
                    TokenType::MINUS => match (left, right) {
                        (Literal::Number(l), Literal::Number(r)) => Literal::Number(l - r),
                        _ => return Err("Operands must be numbers."),
                    },
                    TokenType::LESS
                    | TokenType::LESS_EQUAL
                    | TokenType::GREATER
                    | TokenType::GREATER_EQUAL => match (left, right) {
                        (Literal::Number(l), Literal::Number(r)) => {
                            Literal::Boolean(compare_number(&op.token_type, l, r))
                        }
                        _ => return Err("Operands must be numbers."),
                    },
                    TokenType::EQUAL_EQUAL => Literal::Boolean(left == right),
                    TokenType::BANG_EQUAL => Literal::Boolean(left != right),
                    _ => todo!(),
                }
            }
            Expression::Group(expr) => self.evaluate(expr)?,
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
                    TokenType::BANG => match literal {
                        Literal::Boolean(b) => Literal::Boolean(!b),
                        Literal::Number(n) => Literal::Boolean(n == 0.0),
                        Literal::String(s) => Literal::Boolean(s.is_empty()),
                        Literal::Nil => Literal::Boolean(true),
                    },
                    TokenType::MINUS => match literal {
                        Literal::Number(n) => Literal::Number(-n),
                        _ => return Err("Operand must be a number."),
                    },
                    _ => unreachable!(),
                }
            }
            Expression::Variable(var) => self.get_variable(var)?,
        };
        Ok(literal)
    }

    fn execute_block(&mut self, statements: &Vec<Statement>) -> Result<(), &'static str> {
        self.environment.push();
        for statement in statements.iter() {
            self.execute(statement)?;
        }
        self.environment.pop();
        Ok(())
    }

    fn get_variable(&self, var: &Token) -> Result<Literal, &'static str> {
        let lexeme = &var.lexeme;
        match self.environment.get(lexeme) {
            Some(value) => Ok(value.clone()),
            None => {
                let msg = format!("Undefined variable '{}'.\n[line {}]", lexeme, var.line_num);
                Err(Box::leak(msg.into_boxed_str()))
            }
        }
    }

    fn assign_variable(&mut self, var: &Token, value: &Literal) -> Result<(), &'static str> {
        let lexeme = &var.lexeme;
        if !self.environment.set(lexeme, value) {
            let msg = format!("Undefined variable '{}'.\n[line {}]", lexeme, var.line_num);
            return Err(Box::leak(msg.into_boxed_str()));
        }
        Ok(())
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
