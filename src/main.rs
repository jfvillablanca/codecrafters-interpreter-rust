#![allow(clippy::explicit_write)]
use std::env;
use std::fs;
use std::io::{self, Write};

const EXIT_CODE_INVALID_CHARACTER: i32 = 65;

macro_rules! two_chars {
    ($second:expr, $token:expr, $default:expr, $lexemes:expr) => {{
        if let Some(next_lexeme) = $lexemes.peek() {
            match next_lexeme {
                $second => {
                    println!("{}", $token);
                    $lexemes.next();
                }
                _ => println!($default),
            }
        } else {
            println!($default);
        }
    }};
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
        }
    }
}

fn tokenizer(file_contents: String) {
    let mut had_error = false;
    let mut error_line_number = 1;

    let mut lexemes = file_contents.chars().peekable();
    while let Some(lexeme) = lexemes.next() {
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
            '=' => two_chars!('=', "EQUAL_EQUAL == null", "EQUAL = null", lexemes),
            '!' => two_chars!('=', "BANG_EQUAL != null", "BANG ! null", lexemes),
            '<' => two_chars!('=', "LESS_EQUAL <= null", "LESS < null", lexemes),
            '>' => two_chars!('=', "GREATER_EQUAL >= null", "GREATER > null", lexemes),
            '/' => {
                if let Some(next_lexeme) = lexemes.peek() {
                    match next_lexeme {
                        '/' => {
                            lexemes.next();
                            while let Some(next_next_lexeme) = lexemes.next() {
                                if next_next_lexeme == '\n' {
                                    error_line_number += 1;
                                    break;
                                }
                            }
                        }
                        _ => println!("SLASH / null"),
                    }
                } else {
                    println!("SLASH / null");
                }
            }
            other_lexeme => {
                if other_lexeme.is_whitespace() {
                    if other_lexeme == '\n' {
                        error_line_number += 1;
                    }
                    continue;
                } else {
                    dbg!(other_lexeme, error_line_number);
                    had_error = true;
                    eprintln!(
                        "[line {}] Error: Unexpected character: {}",
                        error_line_number, other_lexeme
                    );
                }
            }
        }
    }
    println!("EOF  null");
    if had_error {
        std::process::exit(EXIT_CODE_INVALID_CHARACTER);
    }
}
