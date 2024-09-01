use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;
use std::str::Lines;

mod eval;
mod expr;
mod parser;
mod token;

use eval::eval;
use expr::Expr;
use parser::Parser;
use token::{Token, TokenType};

fn tokenize_lines(lines: Lines) -> (Vec<Token>, i32) {
    let mut exit_code = 0;
    let mut tokens = vec![];
    for (i, line) in lines.enumerate() {
        let line_num = i + 1;
        let mut chars = line.chars().peekable();

        while let Some(char) = chars.next() {
            if char == ' ' || char == '\t' {
                continue;
            }
            if let Some(token) = Token::get_token(char, &mut chars, line_num) {
                if token.token_type == TokenType::COMMENT {
                    break; // go to next line
                }
                tokens.push(token);
            } else {
                exit_code = 65;
            }
        }
    }
    return (tokens, exit_code);
}

fn tokenize(input: &str) {
    let lines = input.lines();
    let (tokens, exit_code) = tokenize_lines(lines);
    for token in tokens {
        println!("{}", token);
    }
    println!("EOF  null");
    exit(exit_code);
}

fn parse(input: &str) {
    let (tokens, exit_code) = tokenize_lines(input.lines());
    if exit_code != 0 {
        exit(exit_code);
    }
    let mut parser = Parser::new(&tokens);
    let exprs = parser.parse();
    for expr in exprs {
        println!("{}", expr);
    }
}

fn evaluate(input: &str) {
    let (tokens, exit_code) = tokenize_lines(input.lines());
    if exit_code != 0 {
        exit(exit_code);
    }
    let mut parser = Parser::new(&tokens);
    let exprs = parser.parse();
    for expr in exprs {
        match eval(expr) {
            Ok(value) => println!("{}", value),
            Err(msg) => {
                eprintln!("{}", msg);
                exit(70);
            }
        }
    }
}

fn run(input: &str) {
    let (tokens, exit_code) = tokenize_lines(input.lines());
    if exit_code != 0 {
        exit(exit_code);
    }
    let mut parser = Parser::new(&tokens);
    let exprs = parser.parse();
    for expr in exprs {
        match eval(expr) {
            Ok(value) => {
                if matches!(expr, Expr::Print(_)) {
                    println!("{}", value)
                }
            }
            Err(msg) => {
                eprintln!("{}", msg);
                exit(70)
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
        String::new()
    });

    match command.as_str() {
        "tokenize" => tokenize(&file_contents),
        "parse" => parse(&file_contents),
        "evaluate" => evaluate(&file_contents),
        "run" => run(&file_contents),
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
