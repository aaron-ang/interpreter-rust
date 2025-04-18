use anyhow::Result;
use std::collections::HashMap;

use crate::{
    constants::{errors::*, INIT_METHOD, SUPER_KEYWORD, THIS_KEYWORD},
    error::LoxError,
    grammar::{Expression, Function, Statement, Token},
    interpreter::Interpreter,
};

type ResolverResult<T> = Result<T, LoxError>;

#[derive(Debug, Clone, Copy, PartialEq)]
enum FunctionType {
    None,
    Function,
    Initializer,
    Method,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ClassType {
    None,
    Class,
    Subclass,
}

pub struct Resolver<'a> {
    interpreter: &'a mut Interpreter,
    scopes: Vec<HashMap<String, bool>>,
    current_function: FunctionType,
    current_class: ClassType,
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a mut Interpreter) -> Self {
        Resolver {
            interpreter,
            scopes: Vec::new(),
            current_function: FunctionType::None,
            current_class: ClassType::None,
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
                return Err(self.error(name, DUPLICATE_VARIABLE));
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
            Statement::Class {
                name,
                superclass,
                methods,
            } => {
                let enclosing_class = self.current_class;
                self.current_class = ClassType::Class;
                self.declare(name)?;
                self.define(name);

                // Handle inheritance
                if let Some(superclass) = superclass {
                    match superclass {
                        Expression::Variable { name: sup_name, .. } => {
                            if name.lexeme == sup_name.lexeme {
                                return Err(self.error(name, CLASS_INHERIT_SELF));
                            }
                            self.current_class = ClassType::Subclass;
                            self.resolve_expression(superclass)?;
                            self.begin_scope();
                            self.scopes
                                .last_mut()
                                .unwrap()
                                .insert(SUPER_KEYWORD.to_string(), true);
                        }
                        _ => unreachable!("Superclass should be a variable"),
                    }
                }

                // Set up scope for methods with "this" defined
                self.begin_scope();
                if let Some(scope) = self.scopes.last_mut() {
                    scope.insert(THIS_KEYWORD.to_string(), true);
                }

                // Resolve each method
                for method in methods {
                    let function_type = if method.name.lexeme == INIT_METHOD {
                        FunctionType::Initializer
                    } else {
                        FunctionType::Method
                    };
                    self.resolve_function(method, function_type)?;
                }

                self.end_scope();
                if superclass.is_some() {
                    self.end_scope();
                }
                self.current_class = enclosing_class;
            }
            Statement::Variable { name, init } => {
                self.declare(name)?;
                if let Some(expr) = init {
                    self.resolve_expression(expr)?;
                }
                self.define(name);
            }
            Statement::Function(fun) => {
                self.declare(&fun.name)?;
                self.define(&fun.name);
                self.resolve_function(fun, FunctionType::Function)?;
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
                    return Err(self.error(keyword, RETURN_FROM_TOP_LEVEL));
                }
                if let Some(value) = value {
                    if self.current_function == FunctionType::Initializer {
                        return Err(self.error(keyword, RETURN_FROM_INITIALIZER));
                    }
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
                        return Err(self.error(name, VARIABLE_IN_OWN_INITIALIZER));
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
            Expression::This { id, keyword } => {
                if self.current_class == ClassType::None {
                    return Err(self.error(keyword, THIS_OUTSIDE_CLASS));
                }
                self.resolve_local(*id, keyword);
            }
            Expression::Super { id, keyword, .. } => {
                if self.current_class == ClassType::None {
                    return Err(self.error(keyword, SUPER_OUTSIDE_CLASS));
                } else if self.current_class != ClassType::Subclass {
                    return Err(self.error(keyword, SUPER_WITHOUT_SUPERCLASS));
                }
                self.resolve_local(*id, keyword);
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

    fn resolve_function(&mut self, fun: &Function, fun_ty: FunctionType) -> ResolverResult<()> {
        let enclosing_fun = self.current_function;
        self.current_function = fun_ty;

        self.begin_scope();
        for param in &fun.params {
            self.declare(param)?;
            self.define(param);
        }
        self.resolve(&fun.body)?;
        self.end_scope();

        self.current_function = enclosing_fun;
        Ok(())
    }

    fn error(&self, token: &Token, message: &str) -> LoxError {
        LoxError::resolver_error(token.line, &token.lexeme, message)
    }
}
