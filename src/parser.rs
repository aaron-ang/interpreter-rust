use crate::{
    grammar::{Expression, Literal, Statement},
    token::{Token, TokenType},
};

pub struct Parser<'a> {
    tokens: &'a [Token],
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements = vec![];
        while !self.end() {
            statements.push(self.statement()?);
        }
        Ok(statements)
    }

    fn statement(&mut self) -> Result<Statement, String> {
        if self.match_(&[TokenType::PRINT]) {
            let expression = self.expression()?;
            self.consume(&TokenType::SEMICOLON, "Expect ';' after value.")?;
            Ok(Statement::Print(expression))
        } else {
            let expression = self.expression()?;
            self.consume(&TokenType::SEMICOLON, "Expect ';' after expression.")?;
            Ok(Statement::Expression(expression))
        }
    }

    pub fn expression(&mut self) -> Result<Expression, String> {
        self.binary_operation(
            Self::comparison,
            &[TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL],
        )
    }

    fn comparison(&mut self) -> Result<Expression, String> {
        self.binary_operation(
            Self::term,
            &[
                TokenType::GREATER,
                TokenType::GREATER_EQUAL,
                TokenType::LESS,
                TokenType::LESS_EQUAL,
            ],
        )
    }

    fn term(&mut self) -> Result<Expression, String> {
        self.binary_operation(Self::factor, &[TokenType::MINUS, TokenType::PLUS])
    }

    fn factor(&mut self) -> Result<Expression, String> {
        self.binary_operation(Self::unary, &[TokenType::SLASH, TokenType::STAR])
    }

    fn binary_operation(
        &mut self,
        next_precedence: fn(&mut Self) -> Result<Expression, String>,
        operators: &[TokenType],
    ) -> Result<Expression, String> {
        let mut expression = next_precedence(self)?;
        while self.match_(operators) {
            let op = self.previous().clone();
            let right = next_precedence(self)?;
            expression = Expression::Binary {
                op,
                left: Box::new(expression),
                right: Box::new(right),
            };
        }
        Ok(expression)
    }

    pub fn unary(&mut self) -> Result<Expression, String> {
        if self.match_(&[TokenType::BANG, TokenType::MINUS]) {
            let op = self.previous().clone();
            let expr = self.unary()?;
            return Ok(Expression::Unary {
                op,
                expr: Box::new(expr),
            });
        }
        self.primary()
    }

    pub fn primary(&mut self) -> Result<Expression, String> {
        if self.match_(&[TokenType::FALSE]) {
            return Ok(Expression::Literal(Literal::Boolean(false)));
        }

        if self.match_(&[TokenType::TRUE]) {
            return Ok(Expression::Literal(Literal::Boolean(true)));
        }

        if self.match_(&[TokenType::NIL]) {
            return Ok(Expression::Literal(Literal::Nil));
        }

        if self.match_(&[TokenType::NUMBER, TokenType::STRING]) {
            return Ok(Expression::Literal(
                self.previous().literal.as_ref().unwrap().clone(),
            ));
        }

        if self.match_(&[TokenType::LEFT_PAREN]) {
            let expression = self.expression()?;
            self.consume(&TokenType::RIGHT_PAREN, "Expect ')' after expression.")?;
            return Ok(Expression::Group(Box::new(expression)));
        }

        Err(self.error(self.peek(), "Expect expression."))
    }

    fn match_(&mut self, token_types: &[TokenType]) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn consume(&mut self, token_type: &TokenType, message: &str) -> Result<&Token, String> {
        if self.check(token_type) {
            return Ok(self.advance());
        }
        Err(self.error(self.peek(), message))
    }

    fn check(&self, token_type: &TokenType) -> bool {
        !self.end() && self.peek().token_type == *token_type
    }

    fn advance(&mut self) -> &Token {
        if !self.end() {
            self.current += 1;
        }
        self.previous()
    }

    fn end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn error(&self, token: &Token, message: &str) -> String {
        format!(
            "[line {}] Error at '{}': {message}",
            token.line_num, token.lexeme
        )
    }
}
