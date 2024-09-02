use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;

mod grammar;
mod interpreter;
mod parser;
mod scanner;

use grammar::*;
use interpreter::Interpreter;
use parser::Parser;
use scanner::Scanner;

fn tokenize(input: &str) {
    let mut scanner = Scanner::new(input);
    let tokens = scanner.scan_tokens();
    for token in tokens {
        println!("{}", token);
    }
    if scanner.error {
        exit(65);
    }
}

fn parse(input: &str) {
    let mut scanner = Scanner::new(input);
    let tokens = scanner.scan_tokens();
    if scanner.error {
        exit(65);
    }

    let mut parser = Parser::new(&tokens);
    match parser.expression() {
        Ok(expression) => println!("{expression}"),
        Err(msg) => {
            eprintln!("{}", msg);
            exit(65);
        }
    }
}

fn evaluate(input: &str) {
    let mut scanner = Scanner::new(input);
    let tokens = scanner.scan_tokens();
    if scanner.error {
        exit(65);
    }

    let mut parser = Parser::new(&tokens);
    let expr = match parser.expression() {
        Ok(expr) => expr,
        Err(msg) => {
            eprintln!("{}", msg);
            exit(65);
        }
    };

    let mut interpreter = Interpreter::new();
    match interpreter.evaluate(&expr) {
        Ok(val) => match val {
            Literal::Number(n) => println!("{}", n),
            _ => println!("{}", val),
        },
        Err(msg) => {
            eprintln!("{}", msg);
            exit(70);
        }
    }
}

fn run(input: &str) {
    let mut scanner = Scanner::new(input);
    let tokens = scanner.scan_tokens();
    if scanner.error {
        exit(65);
    }

    let mut parser = Parser::new(&tokens);
    let statements = match parser.parse() {
        Ok(statements) => statements,
        Err(msg) => {
            eprintln!("{}", msg);
            exit(65);
        }
    };

    let mut interpreter = Interpreter::new();
    match interpreter.interpret(statements) {
        Ok(_) => {}
        Err(msg) => {
            eprintln!("{}", msg);
            exit(70);
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
