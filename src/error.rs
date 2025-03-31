use thiserror::Error;

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("[line {}] Error at '{}': {}", line, lexeme, message)]
    ParserError {
        line: usize,
        lexeme: String,
        message: String,
    },
    #[error("{0}")]
    TypeError(String),
    #[error("Undefined variable '{lexeme}'.\n[line {line}]")]
    UndefinedVariable { lexeme: String, line: usize },
    #[error("Undefined property '{0}'.")]
    UndefinedProperty(String),
    #[error("Expected {expected} arguments but got {got}.")]
    ArgumentCountError { expected: usize, got: usize },
}

#[derive(Error, Debug)]
pub enum CompileError {
    #[error("[line {}] Error at '{}': {}", line, lexeme, message)]
    ResolverError {
        line: usize,
        lexeme: String,
        message: String,
    },
}
