use std::{env, fs, process::exit, str::FromStr};

use strum::EnumString;

use interpreter_starter_rust::*;

#[derive(EnumString)]
#[strum(serialize_all = "lowercase")]
enum Command {
    Tokenize,
    Parse,
    Evaluate,
    Run,
}

impl Command {
    fn execute(&self, input: &str) {
        let mut scanner = Scanner::new(input);
        let tokens = scanner.scan_tokens();

        if let Command::Tokenize = self {
            for token in tokens {
                println!("{token}");
            }
            if scanner.error {
                exit(SYNTAX_ERROR);
            }
            return;
        }

        if scanner.error {
            exit(SYNTAX_ERROR);
        }

        match self {
            Command::Parse => {
                let mut parser = Parser::new(&tokens);
                let expression = handle_syntax_error(parser.expression());
                println!("{expression}");
            }
            Command::Evaluate => {
                let mut parser = Parser::new(&tokens);
                let expr = handle_syntax_error(parser.expression());

                let mut interpreter = Interpreter::new();
                let val = handle_runtime_error(interpreter.evaluate(&expr));

                match val {
                    Literal::Number(n) => println!("{n}"),
                    _ => println!("{val}"),
                }
            }
            Command::Run => {
                let mut parser = Parser::new(&tokens);
                let statements = handle_syntax_error(parser.parse());

                let mut interpreter = Interpreter::new();
                let mut resolver = Resolver::new(&mut interpreter);

                handle_syntax_error(resolver.resolve(&statements));
                handle_runtime_error(interpreter.interpret(&statements));
            }
            Command::Tokenize => unreachable!(),
        }
    }
}

fn handle_syntax_error<T>(result: Result<T, impl std::fmt::Display>) -> T {
    match result {
        Ok(value) => value,
        Err(err) => {
            eprintln!("{err}");
            exit(SYNTAX_ERROR);
        }
    }
}

fn handle_runtime_error<T>(result: Result<T, impl std::fmt::Display>) -> T {
    match result {
        Ok(value) => value,
        Err(err) => {
            eprintln!("{err}");
            exit(RUNTIME_ERROR);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!(
            "Usage: {} [tokenize|parse|evaluate|run] <filename>",
            args[0]
        );
        exit(COMMAND_LINE_USAGE);
    }

    let command = Command::from_str(&args[1]).unwrap_or_else(|_| {
        eprintln!("Unknown command: {}", args[1]);
        exit(COMMAND_LINE_USAGE);
    });

    let filename = &args[2];
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Failed to read file {filename}");
        exit(CANNOT_OPEN_INPUT);
    });

    command.execute(&file_contents);
}
