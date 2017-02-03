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
        println!("Hello from the interpreter!");
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
