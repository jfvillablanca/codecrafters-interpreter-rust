#![allow(clippy::explicit_write)]
use multipeek::multipeek;
use multipeek::MultiPeek;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::str::Chars;

const EXIT_CODE_INVALID_CHARACTER: i32 = 65;

macro_rules! two_chars {
    ($second:expr, $token:expr, $default:expr, $lexemes:expr) => {{
        if let Some(next_lexeme) = $lexemes.peek() {
            match next_lexeme {
                $second => {
                    let token = format!("{}", $token);
                    add_token(token.as_str());
                    $lexemes.next();
                }
                _ => add_token($default),
            }
        } else {
            add_token($default);
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

fn add_token(token: &str) {
    println!("{token}");
}

fn number_tokenizer(had_error: bool, other_lexeme: char, lexemes: &mut MultiPeek<Chars<'_>>) {
    let mut number_literal = vec![other_lexeme];
    let mut with_radix_point = false;

    while let Some(&peeked_next_lexeme) = lexemes.peek() {
        if peeked_next_lexeme.is_ascii_digit() {
            if let Some(actual_next_lexeme) = lexemes.next() {
                number_literal.push(actual_next_lexeme);
                continue;
            }
        }

        if peeked_next_lexeme != '.' {
            break;
        }

        if let Some(&peeked_next_next_lexeme) = lexemes.peek_nth(1) {
            if !(peeked_next_next_lexeme.is_ascii_digit()
                && peeked_next_lexeme == '.'
                && !with_radix_point)
            {
                continue;
            }

            if let Some(radix_point) = lexemes.next() {
                with_radix_point = true;
                number_literal.push(radix_point);
            }
        }
    }

    // integers are coerced to floats
    let float_literal: String = if !number_literal.contains(&'.') {
        [number_literal.clone(), vec!['.', '0']]
            .concat()
            .into_iter()
            .collect()
    } else {
        // strip trailing zeros
        let mut number_literal = number_literal.clone();
        while let Some(&'0') = number_literal.last() {
            number_literal.pop();
        }

        number_literal.into_iter().collect()
    };

    let number_literal: String = number_literal.into_iter().collect();
    if !number_literal.is_empty() && !had_error {
        let token = format!("NUMBER {} {}", number_literal, float_literal);
        add_token(token.as_str());
    }
}

fn tokenizer(file_contents: String) {
    let mut had_error = false;
    let mut error_line_number = 1;

    let mut lexemes = multipeek(file_contents.chars());
    while let Some(lexeme) = lexemes.next() {
        match lexeme {
            '(' => add_token("LEFT_PAREN ( null"),
            ')' => add_token("RIGHT_PAREN ) null"),
            '{' => add_token("LEFT_BRACE { null"),
            '}' => add_token("RIGHT_BRACE } null"),
            ',' => add_token("COMMA , null"),
            '.' => add_token("DOT . null"),
            '-' => add_token("MINUS - null"),
            '+' => add_token("PLUS + null"),
            ';' => add_token("SEMICOLON ; null"),
            '*' => add_token("STAR * null"),
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
                        _ => add_token("SLASH / null"),
                    }
                } else {
                    add_token("SLASH / null");
                }
            }
            '"' => {
                let mut string_literal = vec![];

                loop {
                    if let Some(&peeked_next_lexeme) = lexemes.peek() {
                        if peeked_next_lexeme == '"' {
                            break;
                        }

                        if let Some(actual_next_lexeme) = lexemes.next() {
                            if actual_next_lexeme == '\n' {
                                error_line_number += 1;
                            }
                            string_literal.push(actual_next_lexeme);
                        }
                    } else {
                        had_error = true;
                        eprintln!("[line {}] Error: Unterminated string.", error_line_number);
                        break;
                    }
                }

                // advance over the closing '"'
                lexemes.next();

                let string_literal: String = string_literal.into_iter().collect();
                if !string_literal.is_empty() && !had_error {
                    let token = format!("STRING \"{}\" {}", string_literal, string_literal);
                    add_token(token.as_str());
                }
            }
            other_lexeme => {
                if other_lexeme.is_whitespace() {
                    if other_lexeme == '\n' {
                        error_line_number += 1;
                    }
                    continue;
                } else if other_lexeme.is_ascii_digit() {
                    number_tokenizer(had_error, other_lexeme, &mut lexemes);
                } else {
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
