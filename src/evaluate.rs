use crate::expr::Expr;
use crate::token::{Token, TokenType};

pub struct Evaluator<'a> {
    exprs: &'a Vec<Expr>,
    tokens: &'a Vec<Token>,
}

impl<'a> Evaluator<'a> {
    pub fn new(exprs: &'a Vec<Expr>, tokens: &'a Vec<Token>) -> Self {
        Self { exprs, tokens }
    }

    pub fn evaluate(&self) {
        for token in self.tokens {
            println!("{}", self.token_to_string(token));
        }
    }

    fn token_to_string(&self, token: &Token) -> String {
        match &token.token_type {
            TokenType::STRING => token.literal.clone().unwrap_or("null".to_string()),
            _ => token.lexeme.clone(),
        }
    }
}
