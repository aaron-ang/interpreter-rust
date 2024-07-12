use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;

mod token;

use crate::token::Token;

fn tokenize(input: &str) -> i32 {
    let mut exit_code = 0;
    let lines = input.lines();
    for (i, line) in lines.enumerate() {
        let line_num = i + 1;
        let mut chars = line.chars().peekable();
        let mut tokens = vec![];
        while let Some(char) = chars.next() {
            if let Some(token) = Token::get_token(char, &mut chars) {
                tokens.push(token);
            } else {
                eprintln!("[line {}] Error: Unexpected character: {}", line_num, char);
                exit_code = 65;
            }
        }
        for token in tokens {
            println!("{}", token);
        }
    }
    exit_code
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

            let exit_code = tokenize(&file_contents);
            println!("EOF  null");
            exit(exit_code);
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
