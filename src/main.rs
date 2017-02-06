#![allow(dead_code)]

use std::env;

#[derive(Copy, Clone)]
enum Token {
    // For syntax errors
    Error           = -1,

    // Important symbols
    EOF             = 0,
    Keyword         = 1,
    Whitespace      = 99,
    Semicolon       = 12,
    Assignment      = 14,

    LogicalOr       = 19,
    LogicalEquality = 26,

    // User-defined symbols
    Integer         = 31,
    Identifier      = 32,
}

mod tokenizer { 

    use Token;
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;
    use std::process;

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
        } else {
            return true;
        }
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

            // TODO: Add check for whitespace (then ignore if true.) 
            //     ? https://doc.rust-lang.org/std/primitive.char.html#method.is_whitespace
            match buf[i] as char {
                ' ' => next_token = Token::Whitespace,
                ';' => next_token = Token::Semicolon,
                '=' => next_token = parse_equal(&buf, &mut i),
                '|' => next_token = parse_logical_or(&buf, &mut i),
                '0' ... '9' => next_token = parse_integer(&buf, &mut i),
                'a' ... 'z' => next_token = parse_keyword(&buf, &mut i),
                'A' ... 'Z' => next_token = parse_identifier(&buf, &mut i),
                          _ => next_token = Token::Error,
            }

            match next_token {
                Token::Whitespace => print!(""),
                _ => tokenizer_output.push(next_token),
            }

            i += 1;

        }

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
            } else {
                i -= 1;
                break;
            }
        }

        // Update the state of our buffer.
        *state = i;

        Token::Keyword
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
        fn test_is_valid_input () {
            let case_a: Vec<String> = vec!["Hello ".to_string(), "world!".to_string()];
            let case_b: Vec<String> = vec!["This ".to_string(), "won't ".to_string(), "work!".to_string()];
            let case_c: Vec<String> = vec!["Goodbye!".to_string()];

            assert_eq!(super::is_valid_input(case_a.len()), true, "Case A should be valid, but wasn't.");
            assert_eq!(super::is_valid_input(case_b.len()), false, "Case B should be invalid, but wasn't.");
            assert_eq!(super::is_valid_input(case_c.len()), false, "Case C should be invalid, but wasn't.");
        }

        fn correctly_tokenizes_test_input_01 () {
            let test_output_01:Vec<Token> == Vec::new();
        }
        
        fn correctly_tokenizes_test_input_02 () {}

        fn correctly_tokenizes_test_input_03 () {}

        fn correctly_tokenizes_test_input_04 () {}

        fn correctly_tokenizes_test_input_05 () {}

        fn correctly_tokenizes_test_input_06 () {}

        fn correctly_tokenizes_test_input_07 () {}

        fn correctly_tokenizes_test_input_08 () {}

        fn correctly_tokenizes_test_input_09 () {}

        fn correctly_tokenizes_test_input_10 () {}

    }
}

fn main() {
    // Fetching the arguments to get the input file name.
    let args: Vec<String> = env::args().collect();

    // Testing the arguments to make sure the interpreter is being called correctly.
    if !tokenizer::is_valid_input(args.len()) {
        tokenizer::print_usage();
    } else {
        // If so, we will begin parsing the input file.
        let ref file:String = args[1];
        tokenizer::init_driver(file);
    }
}
