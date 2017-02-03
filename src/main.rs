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
    use std::env;
    pub fn is_valid_input() -> bool {
        let args: Vec<String> = env::args().collect();
        if args.len() != 2 {
            return false;
        } else {
            return true;
        }
    }

    pub fn print_usage() {
        println!("Usage: ./core_interpreter <core-source-file-name>");
    }

    #[cfg(test)]
    mod test {
        #[test]
        fn test_eq() {
            assert_eq!(1, 1);
        }
    }
}

fn main() {
    if (!core::is_valid_input()) {
        core::print_usage();
    } else {
        println!("Hello!");
    }
}
