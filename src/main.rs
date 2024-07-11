use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;

fn tokenize(char: char) -> (i32, String, String, String) {
    let mut ret = 0;
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
        _ => {
            ret = 65;
            ("", "")
        }
    };
    (
        ret,
        token_type.to_string(),
        char.to_string(),
        literal.to_string(),
    )
}

fn scan(input: &str) -> i32 {
    let mut exit_code = 0;
    let lines = input.lines();
    for (i, line) in lines.enumerate() {
        let line_num = i + 1;
        for char in line.chars() {
            let (ret, token_type, lexeme, literal) = tokenize(char);
            if ret == 0 {
                println!("{} {} {}", token_type, lexeme, literal);
            } else {
                exit_code = ret;
                eprintln!("[line {}] Error: Unexpected character: {}", line_num, char);
            }
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

            let exit_code = scan(&file_contents);
            println!("EOF  null");
            exit(exit_code);
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
