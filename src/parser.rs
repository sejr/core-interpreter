// TODO: Create parse tree object that contains tokens and other stuff.

#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use tokenizer::Token;
use std::collections::HashMap;

struct ParseTree {
    tokens: Vec<Token>,
    memory: HashMap<String, i32>,
    state: u32,
}

pub fn init_parser(file_tokens: Vec<Token>) {
    let mut this_parse_tree = ParseTree {
        tokens: file_tokens.clone(),
        memory: HashMap::new(),
        state: 0,
    };
    parse_prog(this_parse_tree);
}

/// Begins parsing by validating the main program keywords.
fn parse_prog(mut tree: ParseTree) {
    // program <DECL SEQ> begin <STMT SEQ> end
    if tree.tokens[tree.state as usize] == Token::Program {
        tree.state = tree.state + 1;
        tree = parse_decl_seq(tree);
        if tree.tokens[tree.state as usize] == Token::Begin {
            tree.state = tree.state + 1;
            tree = parse_stmt_seq(tree);
            if tree.tokens[tree.state as usize] == Token::End {
                tree.state = tree.state + 1;
                println!("Program was successfully parsed");
            } else {
                panic!("end");
            }
        } else {
            panic!("begin");
        }
    } else {
        panic!("program");
    }
}

#[allow(dead_code)]
fn parse_decl_seq(mut tree: ParseTree) -> ParseTree {
    // <DECL>
    // <DECL> <DECL SEQ>

    tree = parse_decl(tree);
    tree.state = tree.state + 1;

    if tree.tokens[tree.state as usize] == Token::Int {
        tree = parse_decl_seq(tree);
    }

    tree
}

#[allow(dead_code)]
fn parse_decl(mut tree: ParseTree) -> ParseTree {
    // int <ID LIST>

    if tree.tokens[tree.state as usize] == Token::Int {
        tree.state = tree.state + 1;
        tree = parse_id_list(tree);
    } else {
        panic!("parse_decl: failed to parse int keyword");
    }

    tree
}

#[allow(dead_code)]
fn parse_stmt_seq(mut tree: ParseTree) -> ParseTree {
    // <STMT>
    // <STMT> <STMT SEQ>

    let valid_statements: [Token; 5] = [
        Token::Assignment,
        Token::If,
        Token::Loop,
        Token::Read,
        Token::Write
    ];

    if valid_statements.contains(&tree.tokens[tree.state as usize]) {
    // if tree.tokens[tree.state as usize] in valid_statements {
        // We have a valid statement
        match tree.tokens[tree.state as usize] {
            Token::Assignment => tree = parse_assign(tree),
            Token::If => tree = parse_if(tree),
            Token::Loop => tree = parse_loop(tree),
            Token::Read => tree = parse_in(tree),
            Token::Write => tree = parse_out(tree),
            _ => panic!("parse_stmt_seq: invalid statement") // shouldn't happen
        }

    } else {
        panic!("parse_stmt_seq: invalid statement"); // also shouldn't happen
    }

    tree.state = tree.state + 1;
    if valid_statements.contains(&tree.tokens[tree.state as usize]) {
    // if tree.tokens[tree.state as usize] in valid_statements {
        tree = parse_stmt_seq(tree);
    }

    tree
}

#[allow(dead_code)]
fn parse_id_list(mut tree: ParseTree) -> ParseTree {
    // <ID>
    // <ID>, <ID LIST>

    let mut identifier: String;

    let mut id_result: (String, ParseTree);

    match tree.tokens[tree.state as usize].clone() {
        Token::Identifier(ref s) => id_result = parse_id(tree),
        _ => panic!("parse_id_list: expected identifier")
    }

    id_result.1.state = id_result.1.state + 1;

    if id_result.1.tokens[id_result.1.state as usize] == Token::Comma {
        // We need to skip over the comma and parse the next identifier
        id_result.1.state = id_result.1.state + 1;
        id_result.1 = parse_id_list(id_result.1);
    }

    id_result.1
}

#[allow(dead_code)]
fn parse_assign(mut tree: ParseTree) -> ParseTree {
    // <ID> = <EXP>;

    let mut id_result: (String, ParseTree);
    let mut exp_result: (i32, ParseTree);

    match tree.tokens[tree.state as usize].clone() {
        Token::Identifier(ref s) => {
            id_result = parse_id(tree);
            if id_result.1.tokens[id_result.1.state as usize] == Token::Assignment {
                id_result.1.state = id_result.1.state + 1;
                exp_result = parse_exp(id_result.1);
                exp_result.1.memory.insert(id_result.0, exp_result.0);
                if exp_result.1.tokens[exp_result.1.state as usize] == Token::Semicolon {
                    exp_result.1.state = exp_result.1.state + 1;
                } else {
                    panic!("parse_assign: expected ';'");
                }
            } else {
                panic!("parse_assign: expected '='");
            }
        },
        _ => panic!("parse_assign: expected identifier")
    }

    exp_result.1
}

#[allow(dead_code)]
fn parse_if(mut tree: ParseTree) -> ParseTree {
    // if <COND> then <STMT SEQ> end;
    // if <COND> then <STMT SEQ> else <STMT SEQ> end;

    tree
}

#[allow(dead_code)]
fn parse_loop(mut tree: ParseTree) -> ParseTree {
    // while <COND> loop <STMT SEQ> end;

    tree
}

#[allow(dead_code)]
fn parse_in(mut tree: ParseTree) -> ParseTree {
    // read <ID LIST>;

    tree
}

#[allow(dead_code)]
fn parse_out(mut tree: ParseTree) -> ParseTree {
    // write <ID LIST>;

    tree
}

#[allow(dead_code)]
fn parse_cond(mut tree: ParseTree) -> ParseTree {
    // <COMP>
    // !<COMP>
    // [<COND> && <COND>]
    // [<COND> || <COND>]

    tree
}

#[allow(dead_code)]
fn parse_comp(mut tree: ParseTree) -> ParseTree {
    // (<OP> <COMP OP> <OP)

    tree
}

#[allow(dead_code)]
fn parse_exp(mut tree: ParseTree) -> (i32, ParseTree) {
    // <TRM>
    // <TRM> + <EXP>
    // <TRM> - <EXP>

    (0, tree)
}

#[allow(dead_code)]
fn parse_trm(mut tree: ParseTree) -> ParseTree {
    // <OP>
    // <OP> * <TRM>

    tree
}

#[allow(dead_code)]
fn parse_op(mut tree: ParseTree) -> ParseTree {
    // <NO>
    // <ID>
    // (<EXP>)

    tree
}

#[allow(dead_code)]
fn parse_comp_op(mut tree: ParseTree) -> ParseTree {
    // !=
    // ==
    // <
    // >
    // <=
    // >=

    tree
}

fn parse_id(mut tree: ParseTree) -> (String, ParseTree) {

    let mut identifier: String;
    match tree.tokens[tree.state as usize] {
        Token::Identifier(ref s) => identifier = s.clone(),
        _ => panic!("parse_id: expected identifier")
    }

    tree.memory.insert(identifier.clone(), 0);
    tree.state = tree.state + 1;

    (identifier, tree)
}
