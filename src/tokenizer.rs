#![allow(dead_code)]

//! The tokenizer takes the Core source file and converts it to a vector of parsable Tokens.

use std::fs::File;
use std::fmt;
use std::io::prelude::*;
use std::io::{BufReader,BufRead};
use std::process;
use parser;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // For errors
    Error,

    // Reserved words
    Whitespace,
    Program,
    Begin,
    End,
    Int,
    If,
    Then,
    Else,
    While,
    Loop,
    Read,
    Write,

    // Special symbols
    Semicolon,
    Comma,
    Assignment,
    Exclamation,
    LeftSquare,
    RightSquare,
    LogicalAnd,
    LogicalOr,
    LeftParen,
    RightParen,
    Addition,
    Subtraction,
    Multiplication,
    LogicalInequality,
    LogicalEquality,
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,

    // User-defined
    Integer(i32),
    Identifier(String),

    // Other
    EOF,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn exit_err () {
    println!("Error: Illegal token encountered.");
    process::exit(-1);
}

/// Verifies that the correct number (2) of arguments were passed.
///
/// # Examples
///
/// ```
/// use tokenizer;
///
/// if tokenizer::is_valid_input(2) {
///     // Interpreter was called correctly.
/// } else {
///     // Interpreter was not called correctly.
/// }
/// ```
pub fn is_valid_input(arg_count: usize) -> bool {
    /*
     * Takes an unsigned integer as input, which corresponds to the number of arguments
     * called with the Core interpreter executable. We are ensuring that the user called
     * the interpreter with one and only one additional argument, the file name.
     *
     * TODO: May move test_is_valid_input here, changing this to doc comments.
     */
    if arg_count != 3 {
        return false;
    }

    true
}

/// Informs the user of how to invoke the interpreter.
///
/// # Examples
///
/// use tokenizer;
///
/// if tokenizer::is_valid_input(4) {
///
/// }
pub fn print_usage() {
    /*
     * This is a simple function that helps the user understand how to use our interpreter.
     */
    println!("Usage: ./core_interpreter <core-source-file-name> <std-input-file-name>");
}

pub fn init_driver(file: &String, stdin: &String) {
    let output_vector: Vec<Token> = tokenize_file(file);
    let mut stdin_vector: Vec<i32> = Vec::new();

    let file = File::open(stdin).unwrap();
    for line in BufReader::new(file).lines() {
        stdin_vector.push(line.unwrap().parse().unwrap());
    }

    parser::init_parser(output_vector.clone(), stdin_vector.clone());

    // for token in output_vector {
    //     match token {
    //         Token::Error => exit_err(),
    //         _ => println!("{}", token),
    //     }
    // }
}

fn tokenize_file(file: &String) -> Vec<Token> {
    /*
     * It is in this tokenize_file() function that we will do the heavy lifting of opening input
     * files and reading in characters. Those characters will be used to form the designated
     * tokens, which will then be processed later on.
     */

    // Initializing a BufReader based on the file provided as input.
    let mut f = BufReader::new(File::open(file).expect("File failed to open."));

    // A buffer to store the characters we read in from the file (ASCII = 1 byte).
    let mut buf = Vec::<u8>::new();

    // Iterating through the characters of the file . . .
    f.read_to_end(&mut buf).expect("read_to_end failed");

    // Defining the file 'state' which allows us to keep track of the interpreter's progress.
    let mut i: usize = 0;
    let mut tokenizer_output: Vec<Token> = Vec::new();

    while i < buf.len() {

        let next_token: Token;

        match buf[i] as char {

            // Whitespace characters as defined by Rust language
            ' ' => next_token = Token::Whitespace,
            '\n' => next_token = Token::Whitespace,
            '\r' => next_token = Token::Whitespace,
            '\t' => next_token = Token::Whitespace,

            // Special tokens for parsing statements
            ';' => next_token = Token::Semicolon,
            ',' => next_token = Token::Comma,
            '[' => next_token = Token::LeftSquare,
            ']' => next_token = Token::RightSquare,
            '(' => next_token = Token::LeftParen,
            ')' => next_token = Token::RightParen,

            // Mathematical operators
            '+' => next_token = Token::Addition,
            '-' => next_token = Token::Subtraction,
            '*' => next_token = Token::Multiplication,

            // Logical operators
            '=' => next_token = tokenize_equal(&buf, &mut i),
            '!' => next_token = tokenize_inequal(&buf, &mut i),            // TODO
            '<' => next_token = tokenize_lt_lte(&buf, &mut i),             // TODO
            '>' => next_token = tokenize_gt_gte(&buf, &mut i),             // TODO
            '|' => next_token = tokenize_logical_or(&buf, &mut i),
            '&' => next_token = tokenize_logical_and(&buf, &mut i),        // TODO

            // User-defined values
            '0'...'9' => next_token = tokenize_integer(&buf, &mut i),
            'a'...'z' => next_token = tokenize_keyword(&buf, &mut i),    // TODO (Modify)
            'A'...'Z' => next_token = tokenize_identifier(&buf, &mut i),
            _ => next_token = Token::Error,
        }

        match next_token {
            Token::Whitespace => print!(""),
            _ => {
                if next_token == Token::Error {
                    tokenizer_output.push(next_token);
                    break;
                } else {
                    tokenizer_output.push(next_token);
                }
            }
        }

        i += 1;

    }

    tokenizer_output.push(Token::EOF);

    // buf = s.into_bytes();
    buf.clear();
    tokenizer_output
}

fn tokenize_equal(buf: &Vec<u8>, state: &mut usize) -> Token {
    let i: usize = *state as usize;

    if buf[i + 1] as char == '=' {
        // We have come across an equality operator.
        *state += 1;
        return Token::LogicalEquality;
    }

    // Otherwise, it is an assignment token.
    Token::Assignment
}

fn tokenize_logical_or(buf: &Vec<u8>, state: &mut usize) -> Token {
    // The OR can produce an error token if the proceeding state is not an | character.
    let i: usize = *state as usize;
    if buf[i + 1] as char == '|' {
        *state += 1;
        return Token::LogicalOr;
    }

    // For any other scenario, we fail to validate OR token and return error.
    Token::Error
}

fn tokenize_logical_and(buf: &Vec<u8>, state: &mut usize) -> Token {
    let i: usize = *state as usize;
    if buf[i + 1] as char == '&' {
        *state += 1;
        return Token::LogicalAnd;
    }

    Token::Error
}

fn tokenize_inequal(buf: &Vec<u8>, state: &mut usize) -> Token {
    let i: usize = *state as usize;
    if buf[i + 1] as char == '=' {
        *state += 1;
        return Token::LogicalInequality;
    }

    Token::Exclamation
}

fn tokenize_lt_lte(buf: &Vec<u8>, state: &mut usize) -> Token {
    let i: usize = *state as usize;
    if buf[i + 1] as char == '=' {
        *state += 1;
        return Token::LessThanEqual;
    }

    Token::LessThan
}

fn tokenize_gt_gte(buf: &Vec<u8>, state: &mut usize) -> Token {
    let i: usize = *state as usize;
    if buf[i + 1] as char == '=' {
        *state += 1;
        return Token::GreaterThanEqual;
    }

    Token::GreaterThan
}

/*
 * VALIDATED TOKENS
 *
 * The tokens below require a little extra validation. Specifically, they require whitespace
 * or one of the proceeding tokens in order to be identified correctly.
 */

fn tokenize_integer(buf: &Vec<u8>, state: &mut usize) -> Token {
    let mut i: usize = *state as usize;

    // Validating separation between tokens
    if (buf[i - 1] as char >= 'a' && buf[i - 1] as char <= 'z') ||
       (buf[i - 1] as char >= 'A' && buf[i - 1] as char <= 'Z') {
        return Token::Error;
    }

    let start_number = buf[i] as char;
    let mut integer: String = start_number.to_string();

    while i + 1 < buf.len() {
        i += 1;
        if buf[i] as char >= '0' && buf[i] as char <= '9' {
            let new_digit = buf[i] as char;
            integer.push_str(&new_digit.to_string());
        } else if (buf[i] as char >= 'a' && buf[i] as char <= 'z') ||
                  (buf[i] as char >= 'A' && buf[i] as char <= 'z') {
            return Token::Error;
        } else {
            i -= 1;
            break;
        }
    }

    // Update the state of our buffer.
    *state = i;

    // For a more detailed token
    let integer_result = integer.parse().unwrap();

    Token::Integer(integer_result)
}

fn tokenize_keyword(buf: &Vec<u8>, state: &mut usize) -> Token {
    let mut i: usize = *state as usize;

    // Adding a special check because our programs will begin with a keyword.
    if i > 0 {
        // Validating separation between tokens.
        if (buf[i - 1] as char >= 'A' && buf[i - 1] as char <= 'Z') ||
           (buf[i - 1] as char >= '0' && buf[i - 1] as char <= '9') {
            return Token::Error;
        }
    }

    let start_letter = buf[i] as char;
    let mut keyword: String = start_letter.to_string();

    while i + 1 < buf.len() {
        i += 1;
        if buf[i] as char >= 'a' && buf[i] as char <= 'z' {
            let new_char = buf[i] as char;
            keyword.push_str(&new_char.to_string());
        } else if (buf[i] as char >= 'A' && buf[i] as char <= 'Z') ||
                  (buf[i] as char >= '0' && buf[i] as char <= '9') {
            return Token::Error;
        } else {
            i -= 1;
            break;
        }
    }

    // Update the state of our buffer.
    *state = i;

    match keyword.as_ref() {
        "program" => return Token::Program,
        "begin" => return Token::Begin,
        "end" => return Token::End,
        "int" => return Token::Int,
        "if" => return Token::If,
        "then" => return Token::Then,
        "else" => return Token::Else,
        "while" => return Token::While,
        "loop" => return Token::Loop,
        "read" => return Token::Read,
        "write" => return Token::Write,
        _ => return Token::Error,
    }
}

fn tokenize_identifier(buf: &Vec<u8>, state: &mut usize) -> Token {
    let mut i: usize = *state as usize;
    // Validating separation between tokens.
    if (buf[i - 1] as char >= 'a' && buf[i - 1] as char <= 'z') ||
       (buf[i - 1] as char >= '0' && buf[i - 1] as char <= '9') {
        return Token::Error;
    }

    let start_letter = buf[i] as char;
    let mut identifier: String = start_letter.to_string();
    let mut char_flag: bool = true;
    let mut nmbr_flag: bool = true;

    while (i + 1 < buf.len()) && char_flag {
        i += 1;
        if buf[i] as char >= 'A' && buf[i] as char <= 'Z' {
            let new_char = buf[i] as char;
            identifier.push_str(&new_char.to_string());
        } else if buf[i] as char >= 'a' && buf[i] as char <= 'z' {
            return Token::Error;
        } else {
            i -= 1;
            char_flag = !char_flag;
        }
    }

    while (i + 1 < buf.len()) && nmbr_flag {
        i += 1;
        if buf[i] as char >= '0' && buf[i] as char <= '9' {
            let new_digit = buf[i] as char;
            identifier.push_str(&new_digit.to_string());
        } else if (buf[i] as char >= 'a' && buf[i] as char <= 'z') ||
                  (buf[i] as char >= 'A' && buf[i] as char <= 'z') {
            return Token::Error;
        } else {
            i -= 1;
            nmbr_flag = !nmbr_flag;
        }
    }

    *state = i;

    Token::Identifier(identifier)
}

#[cfg(test)]
mod test {

    #[test]
    fn correctly_verifies_argument_count() {
        let case_a: Vec<String> = vec!["Hello ".to_string(), "world!".to_string()];
        let case_b: Vec<String> =
            vec!["This ".to_string(), "won't ".to_string(), "work!".to_string()];
        let case_c: Vec<String> = vec!["Goodbye!".to_string()];

        assert_eq!(super::is_valid_input(case_a.len()),
                   true,
                   "Case A should be valid, but wasn't.");
        assert_eq!(super::is_valid_input(case_b.len()),
                   false,
                   "Case B should be invalid, but wasn't.");
        assert_eq!(super::is_valid_input(case_c.len()),
                   false,
                   "Case C should be invalid, but wasn't.");
    }
}
