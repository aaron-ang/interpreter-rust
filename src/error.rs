use thiserror::Error;

use crate::grammar::Literal;

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("[line {}] Error at '{}': {}", line, lexeme, message)]
    ParserError {
        line: usize,
        lexeme: String,
        message: String,
    },
    #[error("Undefined variable '{lexeme}'.\n[line {line}]")]
    UndefinedVariableError { lexeme: String, line: usize },
    #[error("Expected {expected} arguments but got {got}.")]
    ArgumentCountError { expected: usize, got: usize },
    #[error("{}", self)]
    Return(Literal),
}
