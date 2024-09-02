use std::fmt::Display;

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

    COMMENT,

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
    pub line_num: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<Literal>,
        line_num: usize,
    ) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line_num,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.literal {
            Some(value) => write!(f, "{:?} {} {value}", self.token_type, self.lexeme),
            None => write!(f, "{:?} {} null", self.token_type, self.lexeme),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Literal {
    Boolean(bool),
    String(String),
    Number(f64),
    Nil,
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Literal),
    Group(Box<Expression>),
    Unary {
        op: Token,
        expr: Box<Expression>,
    },
    Binary {
        op: Token,
        left: Box<Expression>,
        right: Box<Expression>,
    },
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Literal(l) => write!(f, "{l}"),
            Expression::Group(g) => {
                write!(f, "(group {g})")
            }
            Expression::Unary { op, expr } => {
                write!(f, "({} {})", op.lexeme, expr)
            }
            Expression::Binary { op, left, right } => {
                write!(f, "({} {} {})", op.lexeme, left, right)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Statement {
    Expression(Expression),
    Print(Expression),
}
