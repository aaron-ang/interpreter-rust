use anyhow::Result;
use std::collections::HashMap;

use crate::{
    error::CompileError,
    grammar::{Expression, Statement, Token},
    interpreter::Interpreter,
};

pub struct Resolver<'a> {
    interpreter: &'a mut Interpreter,
    scopes: Vec<HashMap<String, bool>>,
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a mut Interpreter) -> Self {
        Resolver {
            interpreter,
            scopes: Vec::new(),
        }
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: &Token) -> Result<()> {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.lexeme.clone(), false);
        }
        Ok(())
    }

    fn define(&mut self, name: &Token) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.lexeme.clone(), true);
        }
    }

    pub fn resolve(&mut self, statements: &[Statement]) -> Result<()> {
        for statement in statements {
            self.resolve_statement(statement)?;
        }
        Ok(())
    }

    fn resolve_statement(&mut self, statement: &Statement) -> Result<()> {
        match statement {
            Statement::Block(statements) => {
                self.begin_scope();
                self.resolve(statements)?;
                self.end_scope();
            }
            Statement::Variable { name, init } => {
                self.declare(name)?;
                if let Some(expr) = init {
                    self.resolve_expression(expr)?;
                }
                self.define(name);
            }
            Statement::Function { name, params, body } => {
                self.declare(name)?;
                self.define(name);
                self.resolve_function(params, body)?;
            }
            Statement::Expression(expr) => {
                self.resolve_expression(expr)?;
            }
            Statement::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.resolve_expression(condition)?;
                self.resolve_statement(then_branch)?;
                if let Some(else_branch) = else_branch {
                    self.resolve_statement(else_branch)?;
                }
            }
            Statement::Print(expr) => {
                self.resolve_expression(expr)?;
            }
            Statement::Return { keyword: _, value } => {
                if let Some(value) = value {
                    self.resolve_expression(value)?;
                }
            }
            Statement::While { condition, body } => {
                self.resolve_expression(condition)?;
                self.resolve_statement(body)?;
            }
        }
        Ok(())
    }

    fn resolve_expression(&mut self, expr: &Expression) -> Result<()> {
        match expr {
            Expression::Variable(name) => {
                if let Some(scope) = self.scopes.last() {
                    if let Some(false) = scope.get(&name.lexeme) {
                        return Err(
                            self.error(name, "Can't read local variable in its own initializer.")
                        );
                    }
                }
                self.resolve_local(expr, name);
            }
            Expression::Assign { name, value } => {
                self.resolve_expression(value)?;
                self.resolve_local(expr, name);
            }
            Expression::Binary { left, op: _, right } => {
                self.resolve_expression(left)?;
                self.resolve_expression(right)?;
            }
            Expression::Call { callee, arguments } => {
                self.resolve_expression(callee)?;
                for arg in arguments {
                    self.resolve_expression(arg)?;
                }
            }
            Expression::Grouping(expr) => {
                self.resolve_expression(expr)?;
            }
            Expression::Literal(_) => {}
            Expression::Logical { left, op: _, right } => {
                self.resolve_expression(left)?;
                self.resolve_expression(right)?;
            }
            Expression::Unary { op: _, right } => {
                self.resolve_expression(right)?;
            }
        }
        Ok(())
    }

    fn resolve_local(&mut self, expr: &Expression, name: &Token) {
        for (depth, scope) in self.scopes.iter().rev().enumerate() {
            if scope.contains_key(&name.lexeme) {
                self.interpreter.resolve(expr, depth);
                return;
            }
        }
    }

    fn resolve_function(&mut self, params: &[Token], body: &[Statement]) -> Result<()> {
        self.begin_scope();
        for param in params {
            self.declare(param)?;
            self.define(param);
        }
        self.resolve(body)?;
        self.end_scope();

        Ok(())
    }

    fn error(&self, token: &Token, message: &str) -> anyhow::Error {
        CompileError::ResolverError {
            line: token.line,
            lexeme: token.lexeme.clone(),
            message: message.to_string(),
        }
        .into()
    }
}
