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

mod core_interpreter {
    pub fn hello() {
        println!("Hello from the interpreter!");
    }
}

fn main() {
    core_interpreter::hello();
}
