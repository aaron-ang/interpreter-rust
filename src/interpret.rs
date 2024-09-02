use std::collections::HashMap;

use crate::grammar::{Expression, Literal, Statement};
use crate::token::TokenType;

pub struct Interpreter {
    environment: HashMap<String, Literal>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, statements: Vec<Statement>) -> Result<(), &'static str> {
        for statement in statements {
            self.execute(statement)?;
        }
        Ok(())
    }

    pub fn execute(&mut self, statement: Statement) -> Result<(), &'static str> {
        match statement {
            Statement::Print(expr) => {
                println!("{}", self.evaluate(&expr)?);
            }
            Statement::Expression(expr) => {
                self.evaluate(&expr)?;
            }
        }
        Ok(())
    }

    pub fn evaluate(&mut self, expr: &Expression) -> Result<Literal, &'static str> {
        let value = match expr {
            Expression::Literal(l) => match l {
                Literal::Boolean(b) => Literal::Boolean(*b),
                Literal::String(s) => Literal::String(s.to_string()),
                Literal::Number(n) => Literal::Number(*n),
                Literal::Nil => Literal::Nil,
            },
            Expression::Group(expr) => self.evaluate(expr)?,
            Expression::Unary(op, expr) => {
                let literal = self.evaluate(expr)?;
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
            Expression::Binary(op, left, right) => {
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
                    TokenType::EQUAL_EQUAL | TokenType::BANG_EQUAL => {
                        if !variant_eq(&left, &right) {
                            Literal::Boolean(false)
                        } else {
                            match (left, right) {
                                (Literal::Number(l), Literal::Number(r)) => {
                                    Literal::Boolean(compare_number(&op.token_type, l, r))
                                }
                                (Literal::String(l), Literal::String(r)) => {
                                    Literal::Boolean(compare_string(&op.token_type, l, r))
                                }
                                _ => unreachable!(),
                            }
                        }
                    }
                    _ => todo!(),
                }
            }
        };
        Ok(value)
    }
}

fn variant_eq<T>(a: &T, b: &T) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
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

fn compare_string(op: &TokenType, l: String, r: String) -> bool {
    match op {
        TokenType::EQUAL_EQUAL => l == r,
        TokenType::BANG_EQUAL => l != r,
        _ => unreachable!(),
    }
}
