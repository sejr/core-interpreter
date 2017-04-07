#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

use std::ops::Index;
use tokenizer::Token;
use parser::ParseTree;
use std::collections::HashMap;

pub fn init_executor(file_tokens: Vec<Token>, stdin: Vec<i32>) {
    let mut this_execute_tree = ParseTree {
        tokens: file_tokens.clone(),
        input_stream: stdin.clone(),
        output_stream: Vec::new(),
        memory: HashMap::new(),
        current_statement: "".to_string(),
        statements: Vec::new(),
        context: Vec::new(),
        state: 0,
        depth: 0
    };

    execute_prog(&mut this_execute_tree);

    /*
     * For writing tests later:
     *
     * println!("{}", this_execute_tree.get_token());
     * edit_tree(&mut this_execute_tree);
     * println!("{}", this_execute_tree.get_token());
     * println!("{}", this_execute_tree.retrieve_variable("testing"));
     * println!("{}", this_execute_tree.retrieve_variable("should_not_work"));
     */
}

fn execute_prog(mut tree: &mut ParseTree) {
    if tree.get_token().eq(&Token::Program) {
        tree.descend();
        tree.next();
        execute_decl_seq(&mut tree);
        tree.ascend();
        if tree.get_token().eq(&Token::Begin) {
            tree.descend();
            tree.next(); // Consume the 'begin' keyword
            execute_stmt_seq(&mut tree);
            if tree.get_token().eq(&Token::End) {
                tree.ascend();
                for line in tree.output_stream.clone() {
                    println!("{}", line);
                }
            } else {
                panic!("execute_prog: expected 'end'");
            }
        } else {
            panic!("execute_prog: expected 'begin'");
        }
    } else {
        panic!("program @ {} : {}", tree.state, tree.tokens[tree.state as usize]);
    }
}

fn execute_decl_seq(mut tree: &mut ParseTree) {

    // <DECL>
    // <DECL> <DECL SEQ>

    execute_decl(&mut tree); // parses declaration and moves on

    if tree.get_token().eq(&Token::Int) {
        execute_decl_seq(&mut tree);
    }
}

fn execute_decl(mut tree: &mut ParseTree) {

    // int <ID LIST>;

    if tree.get_token().eq(&Token::Int) {
        tree.next(); // consume int
        execute_id_list(&mut tree);
        if tree.get_token().eq(&Token::Semicolon) {
            tree.next(); // an entire declaration has been consumed; move on
        } else {
            panic!("execute_decl: expected ';'");
        }
    } else {
        panic!("execute_decl: expected 'int'");
    }
}

fn execute_stmt_seq(mut tree: &mut ParseTree) {

    // <STMT>
    // <STMT> <STMT SEQ>

    let mut id_flag: bool = false;
    let mut match_flag: bool = false;

    match tree.get_token() {
        &Token::Identifier(ref i) => {
            id_flag = true;
            match_flag = true;
        },
        _ => print!("")
    }

    if tree.get_token().eq(&Token::Read) {
        match_flag = true;
        execute_in(&mut tree);
    } else if tree.get_token().eq(&Token::Write) {
        match_flag = true;
        execute_out(&mut tree);
    } else if tree.get_token().eq(&Token::If) {
        match_flag = true;
        execute_if(&mut tree);
    } else if tree.get_token().eq(&Token::While) {
        match_flag = true;
        execute_loop(&mut tree);
    }

    if id_flag {
        execute_assign(&mut tree);
    }

    if match_flag {
        execute_stmt_seq(&mut tree);
    }
}

fn execute_id_list(mut tree: &mut ParseTree) {

    // <ID>
    // <ID>, <ID LIST>

    let mut identifier: String;
    let mut id_result: (String, ParseTree);
    execute_id(&mut tree);

    if tree.get_token().eq(&Token::Comma) {
        tree.next(); // consume comma
        execute_id_list(&mut tree);
    }
}

fn execute_assign(mut tree: &mut ParseTree){

    // <ID> = <EXP>;

    tree.context.clear(); // New context

    let current_id: String = tree.retrieve_identifier();

    execute_id(&mut tree);
    if tree.get_token().eq(&Token::Assignment) {
        tree.next(); // throw away =
        let result: i32 = execute_exp(&mut tree);
        tree.insert_variable(current_id.clone(), result.clone());
        if tree.get_token().eq(&Token::Semicolon) {
            tree.next(); // throw away semicolon
        } else {
            panic!("execute_assign: missing ';'");
        }
    } else {
        panic!("execute_assign: expected '='");
    }
}

fn execute_if(mut tree: &mut ParseTree) {

    // if <COND> then <STMT SEQ> end;
    // if <COND> then <STMT SEQ> else <STMT SEQ> end;

    tree.next();
    let mut result: bool = execute_cond(&mut tree);
    if tree.get_token().eq(&Token::Then) {
        tree.descend();
        tree.next();

        if result {
            execute_stmt_seq(&mut tree);
            while !tree.get_token().eq(&Token::End) {
                tree.next();
            }
        } else {
            while !tree.get_token().eq(&Token::Else) || !tree.get_token().eq(&Token::End) {
                tree.next();
            }
        }

        tree.ascend();
        if tree.get_token().eq(&Token::End) {
            tree.next();
            if tree.get_token().eq(&Token::Semicolon) {
                tree.next();
            } else {
                panic!("execute_if: expected ';'");
            }
        } else if tree.get_token().eq(&Token::Else) {
            tree.descend();
            tree.next();
            execute_stmt_seq(&mut tree);
            tree.ascend();
            if tree.get_token().eq(&Token::End) {
                tree.next();
                if tree.get_token().eq(&Token::Semicolon) {
                    tree.next();
                } else {
                    panic!("execute_if: expected ';'");
                }
            } else {
                panic!("execute_if: expected 'end'");
            }
        } else {
            panic!("execute_if: expected 'end' or 'else'");
        }
    } else {
        panic!("execute_if: expected 'then'");
    }
}

fn execute_loop(mut tree: &mut ParseTree) {

    // while <COND> loop <STMT SEQ> end;

    let mut start_state:u32 = tree.state.clone();

    tree.next();
    let mut result: bool = execute_cond(&mut tree);
    if tree.get_token().eq(&Token::Loop) {
        tree.next();
        let loop_depth:u32 = tree.get_depth();
        tree.descend();
        if result {
            execute_stmt_seq(&mut tree);
            tree.set_state(start_state);
            execute_loop(&mut tree);
        } else {
            while !tree.get_token().eq(&Token::End) && tree.get_depth() != loop_depth {
                tree.next();
            }
            tree.ascend();
            if tree.get_token().eq(&Token::End) {
                tree.next();
                if tree.get_token().eq(&Token::Semicolon) {
                    tree.next();
                } else {
                    panic!("execute_loop: expected ';'");
                }
            } else {
                panic!("execute_loop: expected 'end'");
            }
        }
    } else {
        panic!("execute_loop: expected 'loop'");
    }
}

fn execute_in(mut tree: &mut ParseTree) {

    // read <ID LIST>;

    tree.context.clear(); // New context
    tree.next(); // eating the 'read' token

    execute_id_list(&mut tree); // Filling context

    // We are actually doing the reading here!
    for id in tree.context.clone() {
        let val: i32 = tree.read_stdin();
        tree.insert_variable(id.clone(), val.clone());
        // tree.output_stream.push(format!("{} = {}", id, val));
    }

    if tree.get_token().eq(&Token::Semicolon) {
        tree.next();
    } else {
        panic!("execute_in: expected ';'");
    }
}

fn execute_out(mut tree: &mut ParseTree) {

    // write <ID LIST>;

    tree.context.clear(); // New context
    tree.next();

    execute_id_list(&mut tree);
    for id in tree.context.clone() {
        let result: String = tree.retrieve_variable(&id).to_string();
        tree.output_stream.push(format!("{} = {}", id, result));
    }

    if tree.get_token().eq(&Token::Semicolon) {
        tree.next();
    } else {
        panic!("execute_out: expected ';'");
    }
}

fn execute_cond(mut tree: &mut ParseTree) -> bool {

    // <COMP>
    // !<COMP>
    // [<COND> && <COND>]
    // [<COND> || <COND>]

    let mut result: bool = false;

    if tree.get_token().eq(&Token::LeftSquare) {
        tree.next();
        result = execute_cond(&mut tree);
        if tree.get_token().eq(&Token::LogicalAnd) {
            tree.next();
            result = result && execute_cond(&mut tree);
            if tree.get_token().eq(&Token::RightSquare) {
                tree.next();
            } else {
                panic!("execute_cond: expected ']'");
            }
        } else if tree.get_token().eq(&Token::LogicalOr) {
            tree.next();
            result = result || execute_cond(&mut tree);
            if tree.get_token().eq(&Token::RightSquare) {
                tree.next();
            } else {
                panic!("execute_cond: expected ']'");
            }
        } else {
            panic!("execute_cond: expected '+' or '-'");
        }
    } else if tree.get_token().eq(&Token::Exclamation) {
        result = !execute_comp(&mut tree);
    } else {
        result = execute_comp(&mut tree);
    }

    result
}

fn execute_comp(mut tree: &mut ParseTree) -> bool {

    // (<OP> <COMP OP> <OP>)

    let mut result: bool = false;
    let mut op_a: i32;
    let mut op_b: i32;
    let mut cmp: u32;

    if tree.get_token().eq(&Token::LeftParen) {
        tree.next();
        op_a = execute_op(&mut tree);
        cmp = execute_comp_op(&mut tree);
        op_b = execute_op(&mut tree);
        if tree.get_token().eq(&Token::RightParen) {
            tree.next();
        }
    } else {
        panic!("execute_comp: expected '('");
    }

    match cmp {
        0 => result = op_a == op_b,
        1 => result = op_a != op_b,
        2 => result = op_a <  op_b,
        3 => result = op_a <= op_b,
        4 => result = op_a >  op_b,
        5 => result = op_a >= op_b,
        _ => panic!("execute_comp: invalid comp_op")
    }

    result
}

fn execute_exp(mut tree: &mut ParseTree) -> i32 {

    // <TRM>
    // <TRM> + <EXP>
    // <TRM> - <EXP>

    let mut result: i32 = 0;

    result = execute_trm(&mut tree);

    if tree.get_token().eq(&Token::Addition) {
        // handle addition, garble
        tree.next();
        result += execute_exp(&mut tree);
    } else if tree.get_token().eq(&Token::Subtraction) {
        // handle subtraction
        tree.next();
        result -= execute_exp(&mut tree);
    }

    result
}

fn execute_trm(mut tree: &mut ParseTree) -> i32 {

    // <OP>
    // <OP> * <TRM>

    let mut result: i32 = execute_op(&mut tree);

    if tree.get_token().eq(&Token::Multiplication) {
        // handle multiplication, garble
        tree.next();
        result *= execute_trm(&mut tree);
    }

    result
}

fn execute_op(mut tree: &mut ParseTree) -> i32 {

    // <NO>
    // <ID>
    // (<EXP>)

    let mut result:i32 = 0;

    let mut id_flag: bool = false;
    let mut int_flag: bool = false;

    if tree.get_token().eq(&Token::LeftParen) {
        tree.next(); // left paren
        result = execute_exp(&mut tree);
        if tree.get_token().eq(&Token::RightParen) {
            tree.next();
        } else {
            panic!("execute_op: missing ')'");
        }
    } else {
        match tree.get_token() {
            &Token::Identifier(ref id) => id_flag = true,
            &Token::Integer(ref i) => int_flag = true,
            _ => panic!("execute_op: token is not identifier")
        }

        if id_flag {
            id_flag = false;
            let current_id: String = tree.retrieve_identifier().clone();
            result = tree.retrieve_variable(&current_id);
            execute_id(&mut tree);
        } else if int_flag {
            int_flag = false;
            result = *tree.retrieve_integer();
            execute_int(&mut tree);
        } else {
            panic!("execute_op: invalid argument")
        }
    }

    result
}

fn execute_comp_op(mut tree: &mut ParseTree) -> u32 {

    // This is like a literal "opcode" we will be using
    // to perform the actual comparisons later on
    let mut result: u32 = 0;

    match tree.get_token() {
        &Token::LogicalEquality => {
            result = 0;
        },
        &Token::LogicalInequality => {
            result = 1;
        },
        &Token::LessThan => {
            result = 2;
        },
        &Token::LessThanEqual => {
            result = 3;
        },
        &Token::GreaterThan => {
            result = 4;
        },
        &Token::GreaterThanEqual => {
            result = 5;
        },
        _ => panic!("execute_comp_op: unexpected comp op")
    }

    tree.next();

    result
}

fn execute_id(mut tree: &mut ParseTree) {
    let identifier: String = tree.retrieve_identifier();
    tree.context.push(identifier.clone());
    tree.next();
}

fn execute_int(mut tree: &mut ParseTree) {
    let integer: i32 = *tree.retrieve_integer();
    tree.next();
}
