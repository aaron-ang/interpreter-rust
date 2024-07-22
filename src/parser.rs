use crate::{
    expr::Expr,
    token::{Token, TokenType},
};

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self { tokens }
    }

    pub fn parse(&self) -> Vec<Expr> {
        let mut exprs = vec![];
        for token in self.tokens {
            let expr = match token.token_type {
                TokenType::TRUE => Expr::Bool(true),
                TokenType::FALSE => Expr::Bool(false),
                TokenType::NUMBER => Expr::Number(token.literal.clone().unwrap()),
                TokenType::NIL => Expr::Nil,
                _ => todo!(),
            };
            exprs.push(expr);
        }
        exprs
    }
}
