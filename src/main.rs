use std::env;

enum Token {
    Integer     = 31,
    Identifier  = 32,
    Semicolon   = 12,
    Equal       = 14,
    Or          = 19,
    And         = 26,
    Error       = -1,
    EOF         = 0,
}

mod core {

    use std::fs::File;
    use std::io::{BufRead, BufReader};

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
        println!("Processing input {}", file);

        // Initializing a BufReader based on the file provided as input.
        let mut f = BufReader::new(File::open(file).expect("File failed to open."));

        // A buffer to store the characters we read in from the file (ASCII = 1 byte).
        let mut buf = Vec::<u8>::new();

        // Iterating through the characters of the file . . .
        while f.read_until(b'\n', &mut buf).expect("read_until failed") != 0 {

            // Moving the read data into a string
            let s = String::from_utf8(buf).expect("from_utf8 failed");
            for c in s.chars() {
                println!("Char: {}", c);
                
                /* This is where our algorithm for token matching is going to come into play. We
                 * have to check for the special symbols (; = || ==), lowercase words, integers
                 * (sequences of unsigned numbers, including zeroes), and identifiers. Each token
                 * starts with a unique character that will fire off a different parsing function.
                 *
                 * match byte {
                 * 
                 *     ;     => handle semicolon
                 *     =     => handle equality/assignment*
                 *     |     => handle OR
                 *     0-9   => handle integer
                 *     a-z   => handle lowercase words
                 *     A-Z   => handle identifiers
                 *
                 *     * if we catch an =, we check if there's another = after. if so, then we
                 *       treat it as a test for equality; otherwise, its assignment
                 * }
                 */
            }
            buf = s.into_bytes();
            buf.clear();
        }
    }

    fn parse_symbol () {
        // Parses a special symbol; currently supporting ;, ==, =, and ||
    }

    fn parse_word () {
        // Parses a word, currently defined as any sequence of lowercase characters.
    }

    fn parse_identifier () {
        // Parses an identifier, currently defined as any sequence of uppercase characters followed
        // by any sequence of numbers.
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
