use crate::grammar::Literal;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("Undefined variable '{lexeme}'.\n[line {line}]")]
    UndefinedVariableError { lexeme: String, line: usize },
    #[error("Expected {expected} arguments but got {got}.")]
    ArgumentCountError { expected: usize, got: usize },
    #[error("{}", self)]
    Return(Literal),
}
