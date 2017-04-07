use std::env;
mod tokenizer;
mod executor;
mod parser;

fn main() {
    // Fetching the arguments to get the input file name.
    let args: Vec<String> = env::args().collect();

    // Testing the arguments to make sure the interpreter is being called correctly.
    if !tokenizer::is_valid_input(args.len()) {
        tokenizer::print_usage();
    } else {
        // If so, we will begin parsing the input file.
        let ref file: String = args[1];
        tokenizer::init_driver(file);
    }
}
