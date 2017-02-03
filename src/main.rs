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
    pub fn hello() {
        use std::env;
        println!("Core was called with these args:");
        for argument in env::args() {
            println!("{}", argument);
        }
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
    core::hello();
}
