use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::process;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Token {
    // For errors
    Error               = -1,

    // Reserved words
    Whitespace          = 0,
    Program             = 1,
    Begin               = 2,
    End                 = 3,
    Int                 = 4,
    If                  = 5,
    Then                = 6,
    Else                = 7,
    While               = 8,
    Loop                = 9,
    Read                = 10,
    Write               = 11,

    // Special symbols
    Semicolon           = 12,
    Comma               = 13,
    Assignment          = 14,
    Exclamation         = 15,
    LeftSquare          = 16,
    RightSquare         = 17,
    LogicalAnd          = 18,
    LogicalOr           = 19,
    LeftParen           = 20,
    RightParen          = 21,
    Addition            = 22,
    Subtraction         = 23,
    Multiplication      = 24,
    LogicalInequality   = 25,
    LogicalEquality     = 26,
    LessThan            = 27,
    GreaterThan         = 28,
    LessThanEqual       = 29,
    GreaterThanEqual    = 30,

    // User-defined
    Integer             = 31,
    Identifier          = 32,

    // Other
    EOF                 = 33,
}

pub fn is_valid_input(arg_count:usize) -> bool {
    /*
     * Takes an unsigned integer as input, which corresponds to the number of arguments
     * called with the Core interpreter executable. We are ensuring that the user called
     * the interpreter with one and only one additional argument, the file name.
     *
     * TODO: May move test_is_valid_input here, changing this to doc comments.
     */
    if arg_count != 2 {
        return false;
    }

    true
}

pub fn print_usage() {
    /*
     * This is a simple function that helps the user understand how to use our interpreter.
     */
    println!("Usage: ./core_interpreter <core-source-file-name>");
}

pub fn init_driver(file:&String) {
    let output_vector:Vec<Token> = parse_file(file);
    for token in output_vector {
        match token {
            Token::Error => exit_err(),
            _ => println!("{}", token as i32),
        }
    }
}

fn parse_file(file:&String) -> Vec<Token> {
    /*
     * It is in this parse_file() function that we will do the heavy lifting of opening input
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
    let mut i:usize = 0;
    let mut tokenizer_output:Vec<Token> = Vec::new();

    while i < buf.len() {

        let next_token:Token;

        match buf[i] as char {
            
            // Whitespace characters as defined by Rust language
            ' ' => next_token = Token::Whitespace,
            '\n'=> next_token = Token::Whitespace,
            '\r'=> next_token = Token::Whitespace,
            '\t'=> next_token = Token::Whitespace,
            
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
            '=' => next_token = parse_equal(&buf, &mut i),
            '!' => next_token = parse_inequal(&buf, &mut i),            // TODO
            '<' => next_token = parse_lt_lte(&buf, &mut i),             // TODO
            '>' => next_token = parse_gt_gte(&buf, &mut i),             // TODO 
            '|' => next_token = parse_logical_or(&buf, &mut i),
            '&' => next_token = parse_logical_and(&buf, &mut i),        // TODO

            // User-defined values
            '0' ... '9' => next_token = parse_integer(&buf, &mut i),
            'a' ... 'z' => next_token = parse_keyword(&buf, &mut i),    // TODO (Modify)
            'A' ... 'Z' => next_token = parse_identifier(&buf, &mut i),
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

fn exit_err () {
    println!("Error: Illegal token encountered.");
    process::exit(-1);
}

fn parse_equal (buf: &Vec<u8>, state: &mut usize) -> Token {
    let i:usize = *state as usize;
    
    if buf[i + 1] as char == '=' {
        // We have come across an equality operator.
        *state += 1;
        return Token::LogicalEquality;
    }
    
    // Otherwise, it is an assignment token.
    Token::Assignment
}

fn parse_logical_or (buf: &Vec<u8>, state: &mut usize) -> Token {
    // The OR can produce an error token if the proceeding state is not an | character.
    let i:usize = *state as usize;
    if buf[i + 1] as char == '|' {
        *state += 1;
        return Token::LogicalOr;
    }
    
    // For any other scenario, we fail to validate OR token and return error.
    Token::Error
}

fn parse_logical_and (buf: &Vec<u8>, state: &mut usize) -> Token {
    let i:usize = *state as usize;
    if buf[i + 1] as char == '&' {
        *state += 1;
        return Token::LogicalAnd;
    }
    
    Token::Error
}

fn parse_inequal(buf: &Vec<u8>, state: &mut usize) -> Token {
    let i:usize = *state as usize;
    if buf[i + 1] as char == '=' {
        *state += 1;
        return Token::LogicalInequality;
    }

    Token::Exclamation
}

fn parse_lt_lte(buf: &Vec<u8>, state: &mut usize) -> Token {
    let i:usize = *state as usize;
    if buf[i + 1] as char == '=' {
        *state +=1;
        return Token::LessThanEqual;
    }

    Token::LessThan
}

fn parse_gt_gte(buf: &Vec<u8>, state: &mut usize) -> Token {
    let i:usize = *state as usize;
    if buf[i + 1] as char == '=' {
        *state +=1;
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

fn parse_integer (buf: &Vec<u8>, state: &mut usize) -> Token {        
    let mut i:usize = *state as usize;

    // Validating separation between tokens
    if (buf[i - 1] as char >= 'a' && buf[i - 1] as char <= 'z') || 
       (buf[i - 1] as char >= 'A' && buf[i - 1] as char <= 'Z') {
        return Token::Error;
    }

    let start_number = buf[i] as char;
    let mut integer:String = start_number.to_string();

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

    Token::Integer
}

fn parse_keyword (buf: &Vec<u8>, state: &mut usize) -> Token {
    let mut i:usize = *state as usize;
    
    // Adding a special check because our programs will begin with a keyword.
    if i > 0 {
        // Validating separation between tokens.
        if (buf[i - 1] as char >= 'A' && buf[i - 1] as char <= 'Z') ||
           (buf[i - 1] as char >= '0' && buf[i - 1] as char <= '9') {
            return Token::Error;
        }
    }

    let start_letter = buf[i] as char;
    let mut keyword:String = start_letter.to_string();

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
        "program"   => return Token::Program,
        "begin"     => return Token::Begin,
        "end"       => return Token::End,
        "int"       => return Token::Int,
        "if"        => return Token::If,
        "then"      => return Token::Then,
        "else"      => return Token::Else,
        "while"     => return Token::While,
        "loop"      => return Token::Loop,
        "read"      => return Token::Read,
        "write"     => return Token::Write,
        _           => return Token::Error
    }
}

fn parse_identifier (buf: &Vec<u8>, state: &mut usize) -> Token {
    let mut i:usize = *state as usize;
    // Validating separation between tokens.
    if (buf[i - 1] as char >= 'a' && buf[i - 1] as char <= 'z') ||
       (buf[i - 1] as char >= '0' && buf[i - 1] as char <= '9') {
        return Token::Error;
    }

    let start_letter = buf[i] as char;
    let mut identifier:String = start_letter.to_string();
    let mut char_flag:bool = true;
    let mut nmbr_flag:bool = true;

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

    Token::Identifier
}

#[cfg(test)]
mod test {  
    
    #[test]
    fn correctly_verifies_argument_count () {
        let case_a: Vec<String> = vec!["Hello ".to_string(), "world!".to_string()];
        let case_b: Vec<String> = vec!["This ".to_string(), "won't ".to_string(), "work!".to_string()];
        let case_c: Vec<String> = vec!["Goodbye!".to_string()];

        assert_eq!(super::is_valid_input(case_a.len()), true, "Case A should be valid, but wasn't.");
        assert_eq!(super::is_valid_input(case_b.len()), false, "Case B should be invalid, but wasn't.");
        assert_eq!(super::is_valid_input(case_c.len()), false, "Case C should be invalid, but wasn't.");
    }
}
