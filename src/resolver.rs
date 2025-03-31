use anyhow::Result;
use std::collections::HashMap;

use crate::{
    error::CompileError,
    grammar::{Expression, Statement, Token},
    interpreter::Interpreter,
};

#[derive(Debug, Clone, Copy, PartialEq)]
enum FunctionType {
    None,
    Function,
}

type ResolverResult<T> = Result<T, CompileError>;

pub struct Resolver<'a> {
    interpreter: &'a mut Interpreter,
    scopes: Vec<HashMap<String, bool>>,
    current_function: FunctionType,
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a mut Interpreter) -> Self {
        Resolver {
            interpreter,
            scopes: Vec::new(),
            current_function: FunctionType::None,
        }
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: &Token) -> ResolverResult<()> {
        if let Some(scope) = self.scopes.last_mut() {
            if scope.contains_key(&name.lexeme) {
                return Err(self.error(name, "Already a variable with this name in this scope."));
            }
            scope.insert(name.lexeme.clone(), false);
        }
        Ok(())
    }

    fn define(&mut self, name: &Token) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.lexeme.clone(), true);
        }
    }

    pub fn resolve(&mut self, statements: &[Statement]) -> ResolverResult<()> {
        for statement in statements {
            self.resolve_statement(statement)?;
        }
        Ok(())
    }

    fn resolve_statement(&mut self, statement: &Statement) -> ResolverResult<()> {
        match statement {
            Statement::Block(statements) => {
                self.begin_scope();
                self.resolve(statements)?;
                self.end_scope();
            }
            Statement::Class { name, .. } => {
                self.declare(name)?;
                self.define(name);
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
                self.resolve_function(params, body, FunctionType::Function)?;
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
            Statement::Return { keyword, value } => {
                if self.current_function == FunctionType::None {
                    return Err(self.error(keyword, "Can't return from top-level code."));
                }
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

    fn resolve_expression(&mut self, expr: &Expression) -> ResolverResult<()> {
        match expr {
            Expression::Variable { id, name } => {
                if let Some(scope) = self.scopes.last() {
                    if let Some(false) = scope.get(&name.lexeme) {
                        return Err(
                            self.error(name, "Can't read local variable in its own initializer.")
                        );
                    }
                }
                self.resolve_local(*id, name);
            }
            Expression::Assign { id, name, value } => {
                self.resolve_expression(value)?;
                self.resolve_local(*id, name);
            }
            Expression::Binary { left, right, .. } => {
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
            Expression::Logical { left, right, .. } => {
                self.resolve_expression(left)?;
                self.resolve_expression(right)?;
            }
            Expression::Unary { right, .. } => {
                self.resolve_expression(right)?;
            }
            Expression::Get { object, .. } => {
                self.resolve_expression(object)?;
            }
            Expression::Set { object, value, .. } => {
                self.resolve_expression(value)?;
                self.resolve_expression(object)?;
            }
        }
        Ok(())
    }

    fn resolve_local(&mut self, exp_id: usize, name: &Token) {
        for (depth, scope) in self.scopes.iter().rev().enumerate() {
            if scope.contains_key(&name.lexeme) {
                self.interpreter.resolve(exp_id, depth);
                return;
            }
        }
    }

    fn resolve_function(
        &mut self,
        params: &[Token],
        body: &[Statement],
        fun_ty: FunctionType,
    ) -> ResolverResult<()> {
        let enclosing_fun = self.current_function;
        self.current_function = fun_ty;

        self.begin_scope();
        for param in params {
            self.declare(param)?;
            self.define(param);
        }
        self.resolve(body)?;
        self.end_scope();

        self.current_function = enclosing_fun;
        Ok(())
    }

    fn error(&self, token: &Token, message: &str) -> CompileError {
        CompileError::ResolverError {
            line: token.line,
            lexeme: token.lexeme.clone(),
            message: message.to_string(),
        }
    }
}
