// TODO: Create parse tree object that contains tokens and other stuff.

#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use tokenizer::Token;
use std::collections::HashMap;

#[derive(Clone)]
struct ParseTree {
    tokens: Vec<Token>,
    memory: HashMap<String, i32>,
    state: u32,
}

impl ParseTree {
    fn forward(&mut self) {
        self.state += 1;
    }

    fn get_state(&mut self) -> u32 {
        return self.state;
    }

    fn insert_variable(&mut self, identifier: String, value: i32) {
        self.memory.insert(identifier, value);
    }

    fn retrieve_variable(&mut self, identifier: &str) -> i32 {
        match self.memory.get(identifier) {
            Some(&value) => return value,
            _ => panic!("retrieve_variable: variable not present in HashMap"),
        }
    }
}

pub fn init_parser(file_tokens: Vec<Token>) {
    let mut this_parse_tree = ParseTree {
        tokens: file_tokens.clone(),
        memory: HashMap::new(),
        state: 0,
    };

    // println!("{}", this_parse_tree.get_state());
    // edit_tree(&mut this_parse_tree);
    // println!("{}", this_parse_tree.get_state());
    // println!("{}", this_parse_tree.retrieve_variable("testing"));
    // println!("{}", this_parse_tree.retrieve_variable("should_not_work"));
}

fn edit_tree(tree: &mut ParseTree) {
    tree.forward();
    tree.insert_variable("testing".to_string(), 32030);
}

/// Begins parsing by validating the main program keywords.
fn parse_prog(mut tree: ParseTree) {
    // program <DECL SEQ> begin <STMT SEQ> end
    println!("parse_prog: enter");
    if tree.tokens[tree.state as usize] == Token::Program {
        println!("parse_prog: program token");
        tree.state = tree.state + 1;
        tree = parse_decl_seq(tree);
        if tree.tokens[tree.state as usize] == Token::Begin {
            println!("parse_prog: begin token");
            tree.state = tree.state + 1;
            tree = parse_stmt_seq(tree);
            if tree.tokens[tree.state as usize] == Token::End {
                println!("parse_prog: end token");
                tree.state = tree.state + 1;
                println!("Program was successfully parsed");
            } else {
                panic!("end @ {} : {}", tree.state, tree.tokens[tree.state as usize]);
            }
        } else {
            panic!("begin @ {} : {}", tree.state, tree.tokens[tree.state as usize]);
        }
    } else {
        panic!("program @ {} : {}", tree.state, tree.tokens[tree.state as usize]);
    }
}

#[allow(dead_code)]
fn parse_decl_seq(mut tree: ParseTree) -> ParseTree {
    // <DECL>
    // <DECL> <DECL SEQ>

    println!("parse_decl_seq: enter");
    tree = parse_decl(tree);
    tree.state = tree.state + 1;

    if tree.tokens[tree.state as usize] == Token::Int {
        println!("parse_decl_seq: recurse");
        tree = parse_decl_seq(tree);
    }

    tree
}

#[allow(dead_code)]
fn parse_decl(mut tree: ParseTree) -> ParseTree {
    // int <ID LIST>

    println!("parse_decl: enter");
    if tree.tokens[tree.state as usize] == Token::Int {
        println!("parse_prog: int token");
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

    println!("parse_stmt_seq: enter");
    let valid_statements: [Token; 5] = [
        Token::Assignment,
        Token::If,
        Token::While,
        Token::Read,
        Token::Write
    ];

    if valid_statements.contains(&tree.tokens[tree.state as usize]) {
    // if tree.tokens[tree.state as usize] in valid_statements {
        // We have a valid statement
        println!("parse_prog: valid statement");
        match tree.tokens[tree.state as usize] {
            Token::Assignment => tree = parse_assign(tree),
            Token::If => tree = parse_if(tree),
            Token::While => tree = parse_loop(tree),
            Token::Read => {
                println!("read match");
                tree = parse_in(tree)
            },
            Token::Write => tree = parse_out(tree),
            _ => panic!("parse_stmt_seq: invalid statement") // shouldn't happen
        }

    } else {
        panic!("parse_stmt_seq: invalid statement"); // also shouldn't happen
    }

    tree.state = tree.state + 1;
    //println!("tree state: {}", tree.tokens[tree.state as usize].clone());
    if valid_statements.contains(&tree.tokens[tree.state as usize]) {
    // if tree.tokens[tree.state as usize] in valid_statements {
        println!("parse_prog: recurse");
        tree = parse_stmt_seq(tree);
    }

    tree
}

#[allow(dead_code)]
fn parse_id_list(mut tree: ParseTree) -> ParseTree {
    // <ID>
    // <ID>, <ID LIST>

    println!("parse_id_list: enter");
    let mut identifier: String;
    let mut id_result: (String, ParseTree);

    match tree.tokens[tree.state as usize].clone() {
        Token::Identifier(ref s) => {
            println!("parse_prog: get identifier: {}", tree.tokens[tree.state as usize].clone());
            id_result = parse_id(tree);
        },
        _ => panic!("parse_id_list: expected identifier")
    }

    // id_result.1.state = id_result.1.state + 1;

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
    println!("Inside parse_assign");
    let mut id_result: (String, ParseTree);
    let mut exp_result: (i32, ParseTree);

    tree.state = tree.state - 1;

    // println!("TREE STATE ON ASSIGN ENTER: {} {}", tree.state.clone(), tree.tokens[tree.state as usize].clone());

    match tree.tokens[tree.state as usize].clone() {
        Token::Identifier(ref s) => {
            id_result = parse_id(tree);
            if id_result.1.tokens[id_result.1.state as usize] == Token::Assignment {
                println!("trying to assign variable {}", *s);
                id_result.1.state = id_result.1.state + 1;
                exp_result = parse_exp(id_result.1);
                exp_result.1.memory.insert(id_result.0, exp_result.0);
                // println!("tree state: {}", exp_result.1.tokens[exp_result.1.state as usize].clone());
                if exp_result.1.tokens[exp_result.1.state as usize] == Token::Semicolon {
                    // println!("before tree state: {}", exp_result.1.tokens[exp_result.1.state as usize].clone());
                    // exp_result.1.state = exp_result.1.state + 1;
                    println!("after tree state: {}", exp_result.1.tokens[exp_result.1.state as usize].clone());
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
    println!("Inside parse_if");
    // if <COND> then <STMT SEQ> end;
    // if <COND> then <STMT SEQ> else <STMT SEQ> end;

    let mut cond_result: (bool, ParseTree);
    let mut stmt_result: ParseTree;

    match tree.tokens[tree.state as usize].clone() {
        Token::If => {
            tree.state = tree.state + 1;
            cond_result = parse_cond(tree);
            stmt_result = parse_stmt_seq(cond_result.1);
            match stmt_result.tokens[stmt_result.state as usize].clone() {
                Token::Then => {
                    stmt_result.state = stmt_result.state + 1;
                    stmt_result = parse_stmt_seq(stmt_result);
                    match stmt_result.tokens[stmt_result.state as usize].clone() {
                        Token::End => {
                            stmt_result.state = stmt_result.state + 1;
                            match stmt_result.tokens[stmt_result.state as usize].clone() {
                                Token::Semicolon => {
                                    stmt_result.state = stmt_result.state + 1;
                                },
                                _ => panic!("parse_if: expected ';'")
                            }
                        },
                        Token::Else => {
                            stmt_result.state = stmt_result.state + 1;
                            stmt_result = parse_stmt_seq(stmt_result);
                            match stmt_result.tokens[stmt_result.state as usize].clone() {
                                Token::End => {
                                    stmt_result.state = stmt_result.state + 1;
                                    match stmt_result.tokens[stmt_result.state as usize].clone() {
                                        Token::Semicolon => {
                                            stmt_result.state = stmt_result.state + 1;
                                        },
                                        _ => panic!("parse_if: expected ';'")
                                    }
                                },
                                _ => panic!("parse_if: expected 'end'")
                            }
                        },
                        _ => panic!("parse_if: expected 'end' or 'else'")
                    }
                },
                _ => panic!("parse_if: expected 'then'")
            }
        },
        _ => panic!("parse_if: expected 'if'")
    }

    stmt_result
}

#[allow(dead_code)]
fn parse_loop(mut tree: ParseTree) -> ParseTree {
    println!("Inside parse_loop");
    // while <COND> loop <STMT SEQ> end;

    let mut cond_result: (bool, ParseTree);
    let mut stmt_result: ParseTree;

    match tree.tokens[tree.state as usize].clone() {
        Token::While => {
            tree.state = tree.state + 1;
            cond_result = parse_cond(tree);
            println!("tree state: {}", cond_result.1.tokens[cond_result.1.state as usize].clone());
            match cond_result.1.tokens[cond_result.1.state as usize].clone() {
                Token::Loop => {
                    cond_result.1.state = cond_result.1.state + 1;
                    stmt_result = parse_stmt_seq(cond_result.1);
                    match stmt_result.tokens[stmt_result.state as usize].clone() {
                        Token::End => {
                            stmt_result.state = stmt_result.state + 1;
                            match stmt_result.tokens[stmt_result.state as usize].clone() {
                                Token::Semicolon => {
                                    stmt_result.state = stmt_result.state + 1;
                                },
                                _ => panic!("parse_loop: expected ';'")
                            }
                        },
                        _ => panic!("parse_loop: expected 'end'")
                    }
                },
                _ => panic!("parse_loop: expected 'loop'")
            }
        },
        _ => panic!("parse_loop: expected 'while'")
    }

    stmt_result
}

#[allow(dead_code)]
fn parse_in(mut tree: ParseTree) -> ParseTree {
    // read <ID LIST>;
    println!("Inside READ");
    match tree.tokens[tree.state as usize].clone() {
        Token::Read => {
            tree.state = tree.state + 1;
            tree = parse_id_list(tree);
            match tree.tokens[tree.state as usize].clone() {
                Token::Semicolon => {
                    println!("END READ");
                    tree.state = tree.state + 1;
                },
                _ => panic!("parse_in: expected ';'")
            }
        },
        _ => panic!("parse_in: expected 'read'")
    }

    tree
}

#[allow(dead_code)]
fn parse_out(mut tree: ParseTree) -> ParseTree {
    // write <ID LIST>;
    println!("Inside parse_out");
    match tree.tokens[tree.state as usize].clone() {
        Token::Write => {
            tree.state = tree.state + 1;
            tree = parse_id_list(tree);
            match tree.tokens[tree.state as usize].clone() {
                Token::Semicolon => {
                    tree.state = tree.state + 1;
                },
                _ => panic!("parse_out: expected ';'")
            }
        },
        _ => panic!("parse_out: expected 'write'")
    }

    tree
}

#[allow(dead_code, unused_assignments)]
fn parse_cond(mut tree: ParseTree) -> (bool, ParseTree) {
    println!("Inside parse_cond");
    // <COMP>
    // !<COMP>
    // [<COND> && <COND>]
    // [<COND> || <COND>]

    let mut cond_result: (bool, ParseTree) = (false, tree.clone());

    match tree.tokens[tree.state as usize].clone() {
        Token::LeftParen => {
            tree = parse_comp(tree);
        },
        Token::Exclamation => {
            tree.state = tree.state + 1; // NOT
            tree = parse_comp(tree);
        },
        Token::LeftSquare => {
            tree.state = tree.state + 1; // left bracket
            cond_result = parse_cond(tree);
            match cond_result.1.tokens[cond_result.1.state as usize].clone() {
                Token::LogicalAnd => {
                    cond_result.1.state = cond_result.1.state + 1;
                    cond_result = parse_cond(cond_result.1);
                    match cond_result.1.tokens[cond_result.1.state as usize].clone() {
                        Token::RightSquare => {
                            cond_result.1.state = cond_result.1.state + 1;
                        },
                        _ => panic!("parse_cond: expected ']'")
                    }
                },
                Token::LogicalOr => {
                    cond_result.1.state = cond_result.1.state + 1;
                    cond_result = parse_cond(cond_result.1);
                    match cond_result.1.tokens[cond_result.1.state as usize].clone() {
                        Token::RightSquare => {
                            cond_result.1.state = cond_result.1.state + 1;
                        },
                        _ => panic!("parse_cond: expected ']'")
                    }
                },
                _ => panic!("parse_cond: expected '&&' or '||'")
            }
        },
        _ => panic!("parse_cond: expected '(', '[', or '!'")
    }

    (false, cond_result.1)
}

#[allow(dead_code)]
fn parse_comp(mut tree: ParseTree) -> ParseTree {
    println!("Inside parse_comp");
    // (<OP> <COMP OP> <OP)

    match tree.tokens[tree.state as usize].clone() {
        Token::LeftParen => {
            tree.state = tree.state + 1;
            tree = parse_op(tree);
            tree = parse_comp_op(tree);
            tree = parse_op(tree);
            match tree.tokens[tree.state as usize].clone() {
                Token::RightParen => {
                    println!("before rightparen tree state: {}", tree.tokens[tree.state as usize].clone());
                    tree.state = tree.state + 1;
                    println!("after rightparen tree state: {}", tree.tokens[tree.state as usize].clone());
                    return tree;
                },
                _ => panic!("parse_comp: expected ')'")
            }
        },
        _ => panic!("parse_comp: expected '('")
    }
}

#[allow(dead_code)]
fn parse_exp(mut tree: ParseTree) -> (i32, ParseTree) {

    // <TRM>
    // <TRM> + <EXP>
    // <TRM> - <EXP>

    println!("Inside parse_exp");
    tree = parse_trm(tree);

    let mut exp_result: (i32, ParseTree);

    match tree.tokens[(tree.state as usize) + 1].clone() {
        Token::Addition => {
            tree.state = tree.state + 2;
            return parse_exp(tree);
        },
        Token::Subtraction => {
            tree.state = tree.state + 2;
            return parse_exp(tree);
        },
        _ => print!("")
    }

    (0, tree)
}

#[allow(dead_code)]
fn parse_trm(mut tree: ParseTree) -> ParseTree {

    println!("Inside parse_trm");
    // <OP>
    // <OP> * <TRM>

    tree = parse_op(tree);

    match tree.tokens[tree.state as usize].clone() {
        Token::Multiplication => {
            tree.state = tree.state + 1;

        },
        _ => print!("")
    }

    tree
}

#[allow(dead_code)]
fn parse_op(mut tree: ParseTree) -> ParseTree {

    println!("Inside parse_op");
    // <NO>
    // <ID>
    // (<EXP>)

    let mut id_result: (String, ParseTree);
    let mut exp_result: (i32, ParseTree);

    match tree.tokens[tree.state as usize].clone() {
        Token::Identifier(ref s) => {
            id_result = parse_id(tree);
            return id_result.1;
        },
        Token::Integer(ref i) => {
            exp_result = parse_int(tree);
            return exp_result.1;
        },
        Token::LeftParen => {
            tree.state = tree.state + 1;
            exp_result = parse_exp(tree);
            match exp_result.1.tokens[exp_result.1.state as usize].clone() {
                Token::RightParen => {
                    exp_result.1.state = exp_result.1.state + 1;
                },
                _ => panic!("parse_op: expected ')'")
            }
        },
        _ => panic!("parse_op: expected identifier, integer, or '('")
    }

    exp_result.1
}

#[allow(dead_code)]
fn parse_comp_op(mut tree: ParseTree) -> ParseTree {
    println!("inside parse_comp_op");
    tree.state = tree.state + 1; //TODO: Fix

    tree
}

fn parse_id(mut tree: ParseTree) -> (String, ParseTree) {

    println!("Inside parse_id");
    let mut identifier: String;
    match tree.tokens[tree.state as usize] {
        Token::Identifier(ref s) => identifier = s.clone(),
        _ => panic!("parse_id: expected identifier")
    }

    tree.memory.insert(identifier.clone(), 0);
    tree.state = tree.state + 1;

    (identifier, tree)
}

fn parse_int(mut tree: ParseTree) -> (i32, ParseTree) {

    println!("Inside parse_int");
    let mut integer: i32;
    match tree.tokens[tree.state as usize] {
        Token::Integer(ref s) => integer = s.clone(),
        _ => panic!("parse_int: expected integer")
    }

    // tree.memory.insert("", integer.clone());
    tree.state = tree.state + 1;

    (integer, tree)
}
