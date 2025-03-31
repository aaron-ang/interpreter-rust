use anyhow::Result;

use crate::{
    error::RuntimeError,
    grammar::{Expression, Function, Literal, Statement, Token, TokenType},
};

type ParserResult<T> = Result<T, RuntimeError>;

pub struct Parser<'a> {
    tokens: &'a [Token],
    current: usize,
    next_id: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Parser {
            tokens,
            current: 0,
            next_id: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>> {
        let mut statements = vec![];
        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        Ok(statements)
    }

    fn declaration(&mut self) -> ParserResult<Statement> {
        if self.match_(&[TokenType::CLASS]) {
            self.class_declaration()
        } else if self.match_(&[TokenType::FUN]) {
            Ok(Statement::Function(self.function("function")?))
        } else if self.match_(&[TokenType::VAR]) {
            self.variable()
        } else {
            self.statement()
        }
    }

    fn class_declaration(&mut self) -> ParserResult<Statement> {
        let name = self
            .consume(&TokenType::IDENTIFIER, "Expect class name.")?
            .clone();
        self.consume(&TokenType::LEFT_BRACE, "Expect '{' before class body.")?;
        let mut methods = vec![];
        while !self.check(&TokenType::RIGHT_BRACE) && !self.is_at_end() {
            methods.push(self.function("method")?);
        }
        self.consume(&TokenType::RIGHT_BRACE, "Expect '}' after class body.")?;
        Ok(Statement::Class { name, methods })
    }

    fn function(&mut self, kind: &str) -> ParserResult<Function> {
        let name = self
            .consume(&TokenType::IDENTIFIER, &format!("Expect {kind} name."))?
            .clone();
        self.consume(
            &TokenType::LEFT_PAREN,
            &format!("Expect '(' after {kind} name."),
        )?;
        let mut params = vec![];
        if !self.check(&TokenType::RIGHT_PAREN) {
            loop {
                if params.len() >= 255 {
                    return Err(Parser::error(
                        self.peek(),
                        "Cannot have more than 255 parameters.",
                    ));
                }
                params.push(
                    self.consume(&TokenType::IDENTIFIER, "Expect parameter name.")?
                        .clone(),
                );
                if !self.match_(&[TokenType::COMMA]) {
                    break;
                }
            }
        }
        self.consume(&TokenType::RIGHT_PAREN, "Expect ')' after parameters.")?;
        self.consume(
            &TokenType::LEFT_BRACE,
            &format!("Expect '{{' before {kind} body."),
        )?;
        let body = self.block()?;
        Ok(Function { name, params, body })
    }

    fn variable(&mut self) -> ParserResult<Statement> {
        let name = self
            .consume(&TokenType::IDENTIFIER, "Expect variable name.")?
            .clone();
        let init = if self.match_(&[TokenType::EQUAL]) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(
            &TokenType::SEMICOLON,
            "Expect ';' after variable declaration.",
        )?;
        Ok(Statement::Variable { name, init })
    }

    fn statement(&mut self) -> ParserResult<Statement> {
        if self.match_(&[TokenType::FOR]) {
            self.for_statement()
        } else if self.match_(&[TokenType::IF]) {
            self.if_statement()
        } else if self.match_(&[TokenType::PRINT]) {
            let expression = self.expression()?;
            self.consume(&TokenType::SEMICOLON, "Expect ';' after value.")?;
            Ok(Statement::Print(expression))
        } else if self.match_(&[TokenType::RETURN]) {
            self.return_statement()
        } else if self.match_(&[TokenType::WHILE]) {
            self.while_statement()
        } else if self.match_(&[TokenType::LEFT_BRACE]) {
            Ok(Statement::Block(self.block()?))
        } else {
            let expression = self.expression()?;
            self.consume(&TokenType::SEMICOLON, "Expect ';' after expression.")?;
            Ok(Statement::Expression(expression))
        }
    }

    fn for_statement(&mut self) -> ParserResult<Statement> {
        // Consume the opening parenthesis
        self.consume(&TokenType::LEFT_PAREN, "Expect '(' after 'for'.")?;

        // Initialize the initializer
        let initializer = if self.match_(&[TokenType::SEMICOLON]) {
            None
        } else if self.match_(&[TokenType::VAR]) {
            Some(self.variable()?)
        } else {
            Some(self.statement()?)
        };

        // Get the condition
        let condition = if !self.check(&TokenType::SEMICOLON) {
            self.expression()?
        } else {
            Expression::Literal(Literal::Boolean(true))
        };
        self.consume(&TokenType::SEMICOLON, "Expect ';' after loop condition.")?;

        // Get the increment
        let increment = if !self.check(&TokenType::RIGHT_PAREN) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(&TokenType::RIGHT_PAREN, "Expect ')' after for clauses.")?;

        // Get the body
        let mut body = self.statement()?;

        // Desugar the for loop into a while loop
        // First, add the increment to the end of the body if it exists
        if let Some(inc) = increment {
            body = Statement::Block(vec![body, Statement::Expression(inc)]);
        }

        // Then create the while loop with the condition and body
        body = Statement::While {
            condition,
            body: Box::new(body),
        };

        // Finally, if there's an initializer, add it before the while loop in a block
        if let Some(init) = initializer {
            body = Statement::Block(vec![init, body]);
        }

        Ok(body)
    }

    fn if_statement(&mut self) -> ParserResult<Statement> {
        self.consume(&TokenType::LEFT_PAREN, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(&TokenType::RIGHT_PAREN, "Expect ')' after if condition.")?;
        let then_branch = Box::new(self.statement()?);
        let else_branch = if self.match_(&[TokenType::ELSE]) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };
        Ok(Statement::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn return_statement(&mut self) -> ParserResult<Statement> {
        let keyword = self.previous().clone();
        let value = if !self.check(&TokenType::SEMICOLON) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(&TokenType::SEMICOLON, "Expect ';' after return value.")?;
        Ok(Statement::Return { keyword, value })
    }

    fn while_statement(&mut self) -> ParserResult<Statement> {
        self.consume(&TokenType::LEFT_PAREN, "Expect '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume(&TokenType::RIGHT_PAREN, "Expect ')' after condition.")?;
        let body = Box::new(self.statement()?);
        Ok(Statement::While { condition, body })
    }

    fn block(&mut self) -> ParserResult<Vec<Statement>> {
        let mut statements = vec![];
        while !self.check(&TokenType::RIGHT_BRACE) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        self.consume(&TokenType::RIGHT_BRACE, "Expect '}' after block.")?;
        Ok(statements)
    }

    pub fn expression(&mut self) -> ParserResult<Expression> {
        let expr = self.logic_or()?;
        if !self.match_(&[TokenType::EQUAL]) {
            return Ok(expr);
        }

        let value = self.expression()?;
        match expr {
            Expression::Variable { name, .. } => Ok(Expression::Assign {
                id: self.next_id(),
                name,
                value: Box::new(value),
            }),
            Expression::Get { object, name } => Ok(Expression::Set {
                object,
                name,
                value: Box::new(value),
            }),
            _ => Err(Parser::error(self.previous(), "Invalid assignment target.")),
        }
    }

    fn logic_or(&mut self) -> ParserResult<Expression> {
        self.logical_operation(&[TokenType::OR], Self::logic_and)
    }

    fn logic_and(&mut self) -> ParserResult<Expression> {
        self.logical_operation(&[TokenType::AND], Self::equality)
    }

    fn logical_operation(
        &mut self,
        operators: &[TokenType],
        next_precedence: fn(&mut Self) -> ParserResult<Expression>,
    ) -> ParserResult<Expression> {
        let mut left = next_precedence(self)?;
        while self.match_(operators) {
            let op = self.previous().clone();
            let right = next_precedence(self)?;
            left = Expression::Logical {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn equality(&mut self) -> ParserResult<Expression> {
        self.binary_operation(
            &[TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL],
            Self::comparison,
        )
    }

    fn comparison(&mut self) -> ParserResult<Expression> {
        self.binary_operation(
            &[
                TokenType::GREATER,
                TokenType::GREATER_EQUAL,
                TokenType::LESS,
                TokenType::LESS_EQUAL,
            ],
            Self::term,
        )
    }

    fn term(&mut self) -> ParserResult<Expression> {
        self.binary_operation(&[TokenType::MINUS, TokenType::PLUS], Self::factor)
    }

    fn factor(&mut self) -> ParserResult<Expression> {
        self.binary_operation(&[TokenType::SLASH, TokenType::STAR], Self::unary)
    }

    fn binary_operation(
        &mut self,
        operators: &[TokenType],
        next_precedence: fn(&mut Self) -> ParserResult<Expression>,
    ) -> ParserResult<Expression> {
        let mut left = next_precedence(self)?;
        while self.match_(operators) {
            let op = self.previous().clone();
            let right = next_precedence(self)?;
            left = Expression::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn unary(&mut self) -> ParserResult<Expression> {
        if self.match_(&[TokenType::BANG, TokenType::MINUS]) {
            let op = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expression::Unary {
                op,
                right: Box::new(right),
            });
        }
        self.call()
    }

    fn call(&mut self) -> ParserResult<Expression> {
        let mut expression = self.primary()?;
        loop {
            if self.match_(&[TokenType::LEFT_PAREN]) {
                expression = self.finish_call(expression)?;
            } else if self.match_(&[TokenType::DOT]) {
                let name = self
                    .consume(&TokenType::IDENTIFIER, "Expect property name after '.'")?
                    .clone();
                expression = Expression::Get {
                    object: Box::new(expression),
                    name,
                };
            } else {
                break;
            }
        }
        Ok(expression)
    }

    fn finish_call(&mut self, callee: Expression) -> ParserResult<Expression> {
        let mut arguments = vec![];
        if !self.check(&TokenType::RIGHT_PAREN) {
            loop {
                if arguments.len() >= 255 {
                    return Err(Parser::error(
                        self.peek(),
                        "Cannot have more than 255 arguments.",
                    ));
                }
                arguments.push(self.expression()?);
                if !self.match_(&[TokenType::COMMA]) {
                    break;
                }
            }
        }
        self.consume(&TokenType::RIGHT_PAREN, "Expect ')' after arguments.")?;
        Ok(Expression::Call {
            callee: Box::new(callee),
            arguments,
        })
    }

    fn primary(&mut self) -> ParserResult<Expression> {
        let expr = if self.match_(&[TokenType::FALSE]) {
            Expression::Literal(Literal::Boolean(false))
        } else if self.match_(&[TokenType::TRUE]) {
            Expression::Literal(Literal::Boolean(true))
        } else if self.match_(&[TokenType::NIL]) {
            Expression::Literal(Literal::Nil)
        } else if self.match_(&[TokenType::NUMBER, TokenType::STRING]) {
            Expression::Literal(self.previous().literal.clone().unwrap())
        } else if self.match_(&[TokenType::THIS]) {
            Expression::This {
                id: self.next_id(),
                keyword: self.previous().clone(),
            }
        } else if self.match_(&[TokenType::IDENTIFIER]) {
            Expression::Variable {
                id: self.next_id(),
                name: self.previous().clone(),
            }
        } else if self.match_(&[TokenType::LEFT_PAREN]) {
            let expr = self.expression()?;
            self.consume(&TokenType::RIGHT_PAREN, "Expect ')' after expression.")?;
            Expression::Grouping(Box::new(expr))
        } else {
            return Err(Parser::error(self.peek(), "Expect expression."));
        };
        Ok(expr)
    }

    fn match_(&mut self, token_types: &[TokenType]) -> bool {
        token_types.iter().any(|token_type| {
            if self.check(token_type) {
                self.advance();
                true
            } else {
                false
            }
        })
    }

    fn consume(&mut self, token_type: &TokenType, message: &str) -> ParserResult<&Token> {
        if self.check(token_type) {
            return Ok(self.advance());
        }
        Err(Parser::error(self.peek(), message))
    }

    fn check(&self, token_type: &TokenType) -> bool {
        !self.is_at_end() && self.peek().token_type == *token_type
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn next_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn error(token: &Token, message: &str) -> RuntimeError {
        RuntimeError::ParserError {
            line: token.line,
            lexeme: token.lexeme.clone(),
            message: message.to_string(),
        }
    }
}
