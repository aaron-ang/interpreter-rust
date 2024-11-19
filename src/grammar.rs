use std::fmt;

use crate::interpreter::Interpreter;

#[derive(Debug, PartialEq, Clone)]
#[allow(non_camel_case_types)]
pub enum TokenType {
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,

    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    EQUAL,
    EQUAL_EQUAL,
    BANG,
    BANG_EQUAL,
    LESS,
    LESS_EQUAL,
    GREATER,
    GREATER_EQUAL,

    IDENTIFIER,
    STRING,
    NUMBER,

    AND,
    CLASS,
    ELSE,
    FALSE,
    FOR,
    FUN,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

impl TokenType {
    pub fn get_token_type(identifier: &str) -> Self {
        match identifier {
            "and" => Self::AND,
            "class" => Self::CLASS,
            "else" => Self::ELSE,
            "false" => Self::FALSE,
            "for" => Self::FOR,
            "fun" => Self::FUN,
            "if" => Self::IF,
            "nil" => Self::NIL,
            "or" => Self::OR,
            "print" => Self::PRINT,
            "return" => Self::RETURN,
            "super" => Self::SUPER,
            "this" => Self::THIS,
            "true" => Self::TRUE,
            "var" => Self::VAR,
            "while" => Self::WHILE,
            _ => Self::IDENTIFIER,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let literal = match &self.literal {
            Some(value) => value.to_string(),
            None => "null".to_string(),
        };
        write!(f, "{:?} {} {}", self.token_type, self.lexeme, literal)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Boolean(bool),
    String(String),
    Number(f64),
    Nil,
    Callable(Callable),
}

impl Literal {
    pub fn is_truthy(&self) -> bool {
        match self {
            Literal::Boolean(b) => *b,
            Literal::Nil => false,
            _ => true, // Everything else is truthy, including empty strings
        }
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Boolean(b) => write!(f, "{b}"),
            Literal::String(s) => write!(f, "{s}"),
            Literal::Number(n) => {
                let int = n.trunc();
                if int == *n {
                    write!(f, "{int}.0")
                } else {
                    write!(f, "{n}")
                }
            }
            Literal::Nil => write!(f, "nil"),
            Literal::Callable(c) => write!(f, "{}", c.to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Assign {
        name: Token,
        value: Box<Expression>,
    },
    Binary {
        left: Box<Expression>,
        op: Token,
        right: Box<Expression>,
    },
    Call {
        callee: Box<Expression>,
        paren: Token, // to report location of runtime error caused by a function call
        arguments: Vec<Expression>,
    },
    Grouping(Box<Expression>),
    Literal(Literal),
    Logical {
        left: Box<Expression>,
        op: Token,
        right: Box<Expression>,
    },
    Unary {
        op: Token,
        right: Box<Expression>,
    },
    Variable(Token),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Assign { name, value } => {
                write!(f, "(assign {} {})", name.lexeme, value)
            }
            Expression::Binary { left, op, right } => {
                write!(f, "({} {} {})", op.lexeme, left, right)
            }
            Expression::Call {
                callee,
                paren: _,
                arguments,
            } => {
                let args = arguments
                    .iter()
                    .map(|arg| arg.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "(call {} {})", callee, args)
            }
            Expression::Grouping(g) => {
                write!(f, "(group {g})")
            }
            Expression::Literal(l) => write!(f, "{l}"),
            Expression::Logical { left, op, right } => {
                write!(f, "({} {} {})", op.lexeme, left, right)
            }
            Expression::Unary { op, right } => {
                write!(f, "({} {})", op.lexeme, right)
            }
            Expression::Variable(name) => write!(f, "(var {})", name.lexeme),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Statement {
    Block(Vec<Statement>),
    Expression(Expression),
    If {
        condition: Expression,
        then_branch: Box<Statement>,
        else_branch: Option<Box<Statement>>,
    },
    Print(Expression),
    Variable {
        name: Token,
        init: Option<Expression>,
    },
    While {
        condition: Expression,
        body: Box<Statement>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Callable {
    arity_fn: fn(&Callable) -> usize,
    call_fn: fn(&mut Interpreter, Vec<Literal>) -> Result<Literal, &'static str>,
    to_string_fn: fn(&Callable) -> &str,
}

impl Callable {
    pub fn new(
        arity: fn(&Callable) -> usize,
        call: fn(&mut Interpreter, Vec<Literal>) -> Result<Literal, &'static str>,
        to_string: fn(&Callable) -> &str,
    ) -> Self {
        Callable {
            arity_fn: arity,
            call_fn: call,
            to_string_fn: to_string,
        }
    }

    pub fn arity(&self) -> usize {
        (self.arity_fn)(self)
    }

    pub fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Literal>,
    ) -> Result<Literal, &'static str> {
        (self.call_fn)(interpreter, arguments)
    }

    fn to_string(&self) -> &str {
        (self.to_string_fn)(self)
    }
}
