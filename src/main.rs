#![allow(dead_code)]

use std::env;

enum Token {
    Integer = 31,
    Identifier = 32,
    Semicolon = 12,
    OpAssignment = 14,
    OpOr = 19,
    OpEquality = 26,
    Keyword = 1,
    Error = -1,
    EOF = 0,
}

mod core {

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

        let mut skip_word:usize = 0;
        let mut skip_flag:bool = false;

        for c in 0 .. buf.len() {
            if skip_word == 0 {
                // For debugging: println!("Byte: {}", buf[c] as char);
                if buf[c] == ';' as u8 {
                    println!("Token: {} (;)", Token::Semicolon as u8);
                }

                // TODO: Review code to ensure no index-related panics
                else if buf[c] == '=' as u8 {
                    if buf[c + 1] == '=' as u8 {
                        if !skip_flag {
                            println!("Token: {} (==)", Token::OpEquality as u8);
                            skip_flag = !skip_flag;
                        }
                    } else {
                        println!("Token: {} (=)", Token::OpAssignment as u8);
                    }
                }

                // TODO: Review code to ensure no index-related panics
                else if buf[c] == '|' as u8 {
                    if c < buf.len() - 1 {
                        if buf[c + 1] == '|' as u8 {
                            println!("Token: {} (||)", Token::OpOr as u8);
                            skip_flag = !skip_flag;
                        } else {
                            println!("Token: {} (error)", Token::Error as i8);
                        }   
                    }
                }
                
                // TODO: Make this prettier
                else if (buf[c] >= 'a' as u8) && (buf[c] <= 'z' as u8) {
                    let mut keyword = (buf[c] as char).to_string();
                    // println!("{}", identifier);
                    skip_word += 1;
                    let mut new_index:usize = c + skip_word;
                    while buf[new_index] != ' ' as u8 {
                        skip_word += 1;
                        if new_index < buf.len() {
                            let addition = buf[new_index] as char;
                            keyword.push_str(&addition.to_string());
                        }
                        new_index += 1;
                    }
                    println!("Token: {} ({})", Token::Keyword as u8, keyword);
                }

                // Identifiers
                else if (buf[c] >= 'A' as u8) && (buf[c] <= 'Z' as u8) {
                    let mut identifier = (buf[c] as char).to_string();
                    skip_word += 1;
                    let mut new_index:usize = c + skip_word;
                    while (buf[new_index] != ' ' as u8) && (new_index < buf.len()) {
                        skip_word += 1;
                        let addition = buf[new_index] as char;
                        if (addition >= 'A' && addition <= 'Z') || (addition >= '0' && addition <= '9'){
                            identifier.push_str(&addition.to_string());
                            new_index += 1;
                        } else {
                            // The identifier has ended
                            break;
                        }
                    }
                    println!("Token: {} ({})", Token::Identifier as u8, identifier);
                }

            } else {
                skip_word -= 1;
            }
        }

        // buf = s.into_bytes();
        buf.clear();
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
    if !core::is_valid_input(args.len()) {
        core::print_usage();
    } else {
        // If so, we will begin parsing the input file.
        let ref file:String = args[1];
        core::parse_file(file);
    }
}
