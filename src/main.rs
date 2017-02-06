#![allow(dead_code)]

use std::env;

enum Token {
    // For syntax errors
    Error           = -1,

    // Important symbols
    EOF             = 0,
    Keyword         = 1,    
    Semicolon       = 12,
    Assignment      = 14,

    LogicalOr       = 19,
    LogicalEquality = 26,

    // User-defined symbols
    Integer         = 31,
    Identifier      = 32,
}

mod lexer { 

    use Token;
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;

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

    pub fn parse_file(file:&String) {
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
        
        /*
        // Throwing them all into a string because I don't care about memory
        let s = String::from_utf8(buf).expect("from_utf8 failed");
        for c in s.chars() {
            println!("{}", c);
        }*/

        // let mut skip_word:usize = 0;
        // let mut skip_flag:bool = false;

        let mut i:usize = 0;

        while i < buf.len() {

            let next_token:Token = Token::Error;

            match buf[i] as char {
                ' ' => println!(""),
                '=' => parse_equal(&buf, &mut i),
                ';' => parse_semicolon(),
                '|' => parse_logical_or(),
                '0' ... '9' => parse_integer(),
                'a' ... 'z' => parse_keyword(),
                'A' ... 'Z' => parse_identifier(),
                          _ => println!("ERROR")
            }
        /*
            // println!("{}", buf[i] as char);

            // HANDLE EQUALITY AND ASSIGNMENT
            
            if buf[i] as char == '=' {
                i += 1;
                if buf[i] as char == '=' {
                    println!("==");
                } else {
                    println!("=");
                    i -= 1;
                }
            }

            // HANDLE LOGICAL OR

            else if buf[i] as char == '|' {
                i += 1;
                if buf[i] as char == '|' {
                    println!("||");
                } else {
                    println!("Error: expected ||, got |");
                }
            }

            // HANDLE SEMICOLON

            else if buf[i] as char == ';' {
                println!(";");
            }

            // HANDLE KEYWORD

            else if buf[i] as char >= 'a' && buf[i] as char <= 'z' {
                // println!("LOWER-CASE LETTER");
                // BEGIN IDENTIFIER; PROCEED UNTIL NEXT NON-LOWERCASE CHAR
                let start_letter = buf[i] as char;
                let mut keyword:String = start_letter.to_string();
                i += 1;

                while buf[i] as char >= 'a' && buf[i] as char <= 'z' {
                    let new_char = buf[i] as char;
                    keyword.push_str(&new_char.to_string());
                    i += 1;
                }
               
                println!("{}", keyword);
            }

            // HANDLE IDENTIFIER

            else if buf[i] as char >= 'A' && buf[i] as char <= 'Z' {
                // println!("IDENTIFIER");
                
                let start_letter = buf[i] as char;
                let mut identifier:String = start_letter.to_string();

                let mut char_flag:bool = true;
                let mut nmbr_flag:bool = true;
                
                while (i + 1 < buf.len()) && char_flag {
                    i += 1;
                    if buf[i] as char >= 'A' && buf[i] as char <= 'Z' {
                        let new_char = buf[i] as char;
                        identifier.push_str(&new_char.to_string());
                        // i += 1;
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
                        // i += 1;
                    } else {
                        i -= 1;
                        nmbr_flag = !nmbr_flag;
                    }
                }

                // println!("Both complted");
                println!("{}", identifier);
            }

            // HANDLE INTEGER
            
            else if buf[i] as char >= '0' && buf[i] as char <= '9' {
                let start_number = buf[i] as char;
                let mut integer:String = start_number.to_string();
                
                while i + 1 < buf.len() {
                    i += 1;
                    if buf[i] as char >= '0' && buf[i] as char <= '9' {
                        let new_digit = buf[i] as char;
                        integer.push_str(&new_digit.to_string());
                    } else {
                        i -= 1;
                        break;
                    }
                }

                println!("{}", integer);
            }

            */
            // Move to the next byte in the input file
            i += 1;
        }

        // buf = s.into_bytes();
        buf.clear();
    }

    fn parse_equal (buf: &Vec<u8>, state: &mut usize) {
        let i:usize = *state as usize;
        println!("{}", buf[i]);
        // Moving the state forward after we have successfully parsed the equality or assignment
        // *state += 5;
    }

    fn parse_semicolon () {
        println!("SEMICOLON");
    }

    fn parse_logical_or () {
        println!("OR");
    }

    fn parse_integer () {
        println!("INTEGER");
    }

    fn parse_keyword () {
        println!("KEYWORD");
    }

    fn parse_identifier () {
        println!("IDENTIFIER");
    }

    #[cfg(test)]
    mod test {
        #[test]
        fn test_is_valid_input() {
            let case_a: Vec<String> = vec!["Hello ".to_string(), "world!".to_string()];
            let case_b: Vec<String> = vec!["This ".to_string(), "won't ".to_string(), "work!".to_string()];
            let case_c: Vec<String> = vec!["Goodbye!".to_string()];

            assert_eq!(super::is_valid_input(case_a.len()), true, "Case A should be valid, but wasn't.");
            assert_eq!(super::is_valid_input(case_b.len()), false, "Case B should be invalid, but wasn't.");
            assert_eq!(super::is_valid_input(case_c.len()), false, "Case C should be invalid, but wasn't.");
        }
    }
}

fn main() {
    // Fetching the arguments to get the input file name.
    let args: Vec<String> = env::args().collect();

    // Testing the arguments to make sure the interpreter is being called correctly.
    if !lexer::is_valid_input(args.len()) {
        lexer::print_usage();
    } else {
        // If so, we will begin parsing the input file.
        let ref file:String = args[1];
        lexer::parse_file(file);
    }
}
