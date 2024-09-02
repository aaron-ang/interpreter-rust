use std::process::exit;

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

    pub fn parse(&mut self) -> Vec<Statement> {
        let mut statements = vec![];
        while !self.end() {
            statements.push(self.statement());
        }
        statements
    }

    fn statement(&mut self) -> Statement {
        if self.match_(&[TokenType::PRINT]) {
            let expression = self.expression();
            self.consume(&TokenType::SEMICOLON, "Expect ';' after value.");
            Statement::Print(expression)
        } else {
            let expression = self.expression();
            self.consume(&TokenType::SEMICOLON, "Expect ';' after expression.");
            Statement::Expression(expression)
        }
    }

    pub fn expression(&mut self) -> Expression {
        self.binary_operation(
            Self::comparison,
            &[TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL],
        )
    }

    fn comparison(&mut self) -> Expression {
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

    fn term(&mut self) -> Expression {
        self.binary_operation(Self::factor, &[TokenType::MINUS, TokenType::PLUS])
    }

    fn factor(&mut self) -> Expression {
        self.binary_operation(Self::unary, &[TokenType::SLASH, TokenType::STAR])
    }

    fn binary_operation(
        &mut self,
        next_precedence: fn(&mut Self) -> Expression,
        operators: &[TokenType],
    ) -> Expression {
        let mut expression = next_precedence(self);
        while self.match_(operators) {
            let op = self.previous().clone();
            let right = next_precedence(self);
            expression = Expression::Binary {
                op,
                left: Box::new(expression),
                right: Box::new(right),
            };
        }
        expression
    }

    pub fn unary(&mut self) -> Expression {
        if self.match_(&[TokenType::BANG, TokenType::MINUS]) {
            let op = self.previous().clone();
            let expr = self.unary();
            return Expression::Unary {
                op,
                expr: Box::new(expr),
            };
        }
        self.primary()
    }

    pub fn primary(&mut self) -> Expression {
        if self.match_(&[TokenType::FALSE]) {
            return Expression::Literal(Literal::Boolean(false));
        }

        if self.match_(&[TokenType::TRUE]) {
            return Expression::Literal(Literal::Boolean(true));
        }

        if self.match_(&[TokenType::NIL]) {
            return Expression::Literal(Literal::Nil);
        }

        if self.match_(&[TokenType::NUMBER, TokenType::STRING]) {
            return Expression::Literal(self.previous().literal.as_ref().unwrap().clone());
        }

        if self.match_(&[TokenType::LEFT_PAREN]) {
            let expression = self.expression();
            self.consume(&TokenType::RIGHT_PAREN, "Expect ')' after expression.");
            return Expression::Group(Box::new(expression));
        }

        self.error(self.peek(), "Expect expression.")
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

    fn consume(&mut self, token_type: &TokenType, message: &str) -> &Token {
        if self.check(token_type) {
            return self.advance();
        }
        self.error(self.peek(), message)
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.end() {
            return false;
        }
        self.peek().token_type == *token_type
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

    fn error(&self, token: &Token, message: &str) -> ! {
        eprintln!(
            "[line {}] Error at '{}': {message}",
            token.line_num, token.lexeme
        );
        exit(65);
    }
}
