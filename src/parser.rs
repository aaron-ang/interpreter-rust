use std::iter::Peekable;
use std::slice::Iter;

use crate::{
    exit,
    expr::Expr,
    token::{Token, TokenType},
};

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    exprs: Vec<Expr>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self {
            tokens,
            exprs: vec![],
        }
    }

    pub fn parse(&mut self) -> &Vec<Expr> {
        let mut tokens = self.tokens.iter().peekable();
        while let Some(token) = tokens.next() {
            let expr = self.get_expr(token, &mut tokens);
            self.exprs.push(expr);
        }
        &self.exprs
    }

    fn get_expr(&mut self, token: &Token, tokens: &mut Peekable<Iter<Token>>) -> Expr {
        let expr = match token.token_type {
            TokenType::TRUE => Expr::Bool(true),
            TokenType::FALSE => Expr::Bool(false),
            TokenType::NUMBER => Expr::Number(token.literal.clone().unwrap()),
            TokenType::STRING => Expr::String(token.literal.clone().unwrap()),
            TokenType::LEFT_PAREN => {
                while let Some(token) = tokens.next() {
                    if token.token_type == TokenType::RIGHT_PAREN {
                        break;
                    }
                    if tokens.peek().is_none() {
                        eprintln!("Error: Unmatched parentheses.");
                        exit(65);
                    }
                    let expr = self.get_expr(token, tokens);
                    self.exprs.push(expr);
                }
                // no expression was found after the left parenthesis
                if self.exprs.is_empty() {
                    expr_error(token)
                }
                Expr::Group(self.exprs.drain(..).collect())
            }
            TokenType::RIGHT_PAREN => {
                // right parenthesis was reached before the end of the expression
                expr_error(token)
            }
            TokenType::BANG => Expr::Unary(
                token.clone(),
                Box::new(self.get_expr(tokens.next().unwrap(), tokens)),
            ),
            TokenType::STAR
            | TokenType::SLASH
            | TokenType::PLUS
            | TokenType::LESS
            | TokenType::GREATER
            | TokenType::LESS_EQUAL
            | TokenType::GREATER_EQUAL
            | TokenType::EQUAL_EQUAL
            | TokenType::BANG_EQUAL => {
                if self.exprs.is_empty() {
                    expr_error(token)
                }
                let left = self.exprs.pop().unwrap();
                let next_token = tokens.next();
                if next_token.is_none() {
                    expr_error(token)
                }
                let right = self.get_expr(next_token.unwrap(), tokens);
                Expr::Binary(token.clone(), Box::new(left), Box::new(right))
            }
            TokenType::MINUS => {
                let next_token = tokens.next();
                if next_token.is_none() {
                    expr_error(token)
                }
                if self.exprs.is_empty() {
                    Expr::Unary(
                        token.clone(),
                        Box::new(self.get_expr(next_token.unwrap(), tokens)),
                    )
                } else {
                    let left = self.exprs.pop().unwrap();
                    let right = self.get_expr(next_token.unwrap(), tokens);
                    Expr::Binary(token.clone(), Box::new(left), Box::new(right))
                }
            }
            TokenType::NIL => Expr::Nil,
            _ => todo!(),
        };
        expr
    }
}

fn expr_error(token: &Token) -> ! {
    eprintln!(
        "[line {}] Error at '{}': Expect expression.",
        token.line_num, token.lexeme
    );
    exit(65);
}
