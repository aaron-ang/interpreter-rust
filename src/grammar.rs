use std::{cell::RefCell, fmt, rc::Rc};

use crate::callable::{LoxCallable, LoxClass, LoxInstance};

#[derive(Debug, Clone, PartialEq, Eq, Hash, strum::Display)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
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

#[derive(Debug, Clone, PartialEq)]
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
        write!(f, "{} {} {}", self.token_type, self.lexeme, literal)
    }
}

#[derive(Debug, Clone)]
pub enum Literal {
    Nil,
    Boolean(bool),
    String(String),
    Number(f64),
    Function(Rc<dyn LoxCallable>),
    Class(Rc<LoxClass>),
    Instance(Rc<RefCell<LoxInstance>>),
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

impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Literal::Nil, Literal::Nil) => true,
            (Literal::Boolean(a), Literal::Boolean(b)) => a == b,
            (Literal::String(a), Literal::String(b)) => a == b,
            (Literal::Number(a), Literal::Number(b)) => a == b,
            (Literal::Function(a), Literal::Function(b)) => Rc::ptr_eq(a, b),
            (Literal::Class(a), Literal::Class(b)) => Rc::ptr_eq(a, b),
            (Literal::Instance(a), Literal::Instance(b)) => Rc::ptr_eq(a, b),
            _ => false,
        }
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = match self {
            Literal::Boolean(b) => b.to_string(),
            Literal::String(s) => s.to_string(),
            Literal::Number(n) => {
                if n.fract() == 0.0 {
                    // integer
                    format!("{:.1}", n)
                } else {
                    // float
                    n.to_string()
                }
            }
            Literal::Nil => "nil".to_string(),
            Literal::Function(c) => c.to_string(),
            Literal::Class(c) => c.to_string(),
            Literal::Instance(c) => c.borrow().to_string(),
        };
        write!(f, "{output}")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Assign {
        id: usize,
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
        arguments: Vec<Expression>,
    },
    Grouping(Box<Expression>),
    Literal(Literal),
    Logical {
        left: Box<Expression>,
        op: Token,
        right: Box<Expression>,
    },
    Set {
        object: Box<Expression>,
        name: Token,
        value: Box<Expression>,
    },
    Unary {
        op: Token,
        right: Box<Expression>,
    },
    Variable {
        id: usize,
        name: Token,
    },
    Get {
        object: Box<Expression>,
        name: Token,
    },
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Assign { name, value, .. } => {
                write!(f, "(assign {} {})", name.lexeme, value)
            }
            Expression::Binary { left, op, right } => {
                write!(f, "({} {} {})", op.lexeme, left, right)
            }
            Expression::Call { callee, arguments } => {
                let args = arguments
                    .iter()
                    .map(|arg| arg.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "(call {callee} {args})")
            }
            Expression::Grouping(g) => {
                write!(f, "(group {g})")
            }
            Expression::Literal(l) => write!(f, "{l}"),
            Expression::Logical { left, op, right } => {
                write!(f, "({} {} {})", op.lexeme, left, right)
            }
            Expression::Set {
                object,
                name,
                value,
            } => {
                write!(f, "(set {} {} {})", object, name.lexeme, value)
            }
            Expression::Unary { op, right } => {
                write!(f, "({} {})", op.lexeme, right)
            }
            Expression::Variable { name, .. } => write!(f, "(var {})", name.lexeme),
            Expression::Get { object, name } => {
                write!(f, "(get {} {})", object, name.lexeme)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Statement {
    Block(Vec<Statement>),
    Class {
        name: Token,
        methods: Vec<Statement>,
    },
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
    Function {
        name: Token,
        params: Vec<Token>,
        body: Vec<Statement>,
    },
    Return {
        keyword: Token,
        value: Option<Expression>,
    },
}
