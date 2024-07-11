use core::panic;
use std::env;
use std::fs;
use std::io::{self, Write};

fn tokenize(char: char) -> (String, String, String) {
    let (token_type, literal) = match char {
        '(' => ("LEFT_PAREN", "null"),
        ')' => ("RIGHT_PAREN", "null"),
        '{' => ("LEFT_BRACE", "null"),
        '}' => ("RIGHT_BRACE", "null"),
        ',' => ("COMMA", "null"),
        '.' => ("DOT", "null"),
        '-' => ("MINUS", "null"),
        '+' => ("PLUS", "null"),
        ';' => ("SEMICOLON", "null"),
        '*' => ("STAR", "null"),
        _ => panic!("Unknown character: {}", char),
    };
    (
        token_type.to_string(),
        char.to_string(),
        literal.to_string(),
    )
}

fn scan(input: &str) {
    for char in input.chars() {
        let (token_type, lexeme, literal) = tokenize(char);
        println!("{} {} {}", token_type, lexeme, literal);
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

    match command.as_str() {
        "tokenize" => {
            writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            if !file_contents.is_empty() {
                scan(&file_contents);
            }
            println!("EOF  null");
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
