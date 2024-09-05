#![allow(clippy::explicit_write)]
use std::env;
use std::fs;
use std::io::{self, Write};

const EXIT_CODE_INVALID_CHARACTER: i32 = 65;

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
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            tokenizer(file_contents);
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}

fn tokenizer(file_contents: String) {
    let mut had_error = false;

    let mut lexemes = file_contents.chars().peekable();
    while let Some(lexeme) = lexemes.next() {
        dbg!(lexeme);
        match lexeme {
            '(' => println!("LEFT_PAREN ( null"),
            ')' => println!("RIGHT_PAREN ) null"),
            '{' => println!("LEFT_BRACE {{ null"),
            '}' => println!("RIGHT_BRACE }} null"),
            ',' => println!("COMMA , null"),
            '.' => println!("DOT . null"),
            '-' => println!("MINUS - null"),
            '+' => println!("PLUS + null"),
            ';' => println!("SEMICOLON ; null"),
            '*' => println!("STAR * null"),
            '=' => {
                if let Some(next_lexeme) = lexemes.peek() {
                    match next_lexeme {
                        '=' => {
                            println!("EQUAL_EQUAL == null");
                            lexemes.next();
                        },
                        _ => println!("EQUAL = null"),
                    }
                } else {
                    println!("EQUAL = null");
                }
            }
            invalid_lexeme => {
                had_error = true;
                eprintln!("[line 1] Error: Unexpected character: {}", invalid_lexeme);
            }
        }
    }
    println!("EOF  null");
    if had_error {
        std::process::exit(EXIT_CODE_INVALID_CHARACTER);
    }
}
