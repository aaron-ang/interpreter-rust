use std::collections::HashMap;

use crate::grammar::*;

pub struct Interpreter {
    environment: HashMap<String, Literal>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environment: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, statements: Vec<Statement>) -> Result<(), &'static str> {
        for statement in statements {
            self.execute(statement)?;
        }
        Ok(())
    }

    fn execute(&mut self, statement: Statement) -> Result<(), &'static str> {
        match statement {
            Statement::Print(expr) => match self.evaluate(&expr)? {
                Literal::Number(n) => println!("{}", n),
                val => println!("{}", val),
            },
            Statement::Expression(expr) => {
                self.evaluate(&expr)?;
            }
            Statement::Variable { name, init } => {
                let value = match init {
                    Some(expr) => self.evaluate(&expr)?,
                    None => Literal::Nil,
                };
                self.environment.insert(name.lexeme, value);
            }
        }
        Ok(())
    }

    pub fn evaluate(&mut self, expr: &Expression) -> Result<Literal, &'static str> {
        let literal = match expr {
            Expression::Literal(l) => l.clone(),
            Expression::Group(expr) => self.evaluate(expr)?,
            Expression::Unary { op, expr } => {
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
            Expression::Binary { op, left, right } => {
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
            Expression::Variable(var) => self.get_variable(var)?,
        };
        Ok(literal)
    }

    fn get_variable(&self, var: &Token) -> Result<Literal, &'static str> {
        let lexeme = &var.lexeme;
        match self.environment.get(lexeme.as_str()) {
            Some(value) => Ok(value.clone()),
            None => {
                let msg = format!("Undefined variable '{}'.\n[line {}]", lexeme, var.line_num);
                Err(Box::leak(msg.into_boxed_str()))
            }
        }
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
