use thiserror::Error;

#[derive(Error, Debug)]
pub enum LoxError {
    #[error("[line {line}] {error_type} Error at '{lexeme}': {message}")]
    SyntaxError {
        line: usize,
        error_type: &'static str,
        lexeme: String,
        message: String,
    },

    #[error("{0}")]
    TypeError(String),

    #[error("Undefined variable '{name}'.\n[line {line}]")]
    UndefinedVariable { name: String, line: usize },

    #[error("Undefined property '{0}'.")]
    UndefinedProperty(String),

    #[error("Expected {expected} arguments but got {got}.")]
    ArgumentCountError { expected: usize, got: usize },
}

impl LoxError {
    pub fn parser_error(line: usize, lexeme: &str, message: &str) -> Self {
        Self::SyntaxError {
            line,
            error_type: "Parser",
            lexeme: lexeme.to_string(),
            message: message.to_string(),
        }
    }

    pub fn resolver_error(line: usize, lexeme: &str, message: &str) -> Self {
        Self::SyntaxError {
            line,
            error_type: "Resolver",
            lexeme: lexeme.to_string(),
            message: message.to_string(),
        }
    }
}
