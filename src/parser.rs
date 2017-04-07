#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

use std::ops::Index;
use tokenizer::Token;
use std::collections::HashMap;

#[derive(Clone)]
pub struct ParseTree {
    pub tokens: Vec<Token>,
    pub memory: HashMap<String, i32>,
    pub current_statement: String,
    pub statements: Vec<String>,
    pub context: Vec<String>,
    pub state: u32,
    pub depth: u32
}

impl ParseTree {
    pub fn next(&mut self) {
        self.state += 1;
    }

    pub fn get_token(&mut self) -> &Token {
        return &self.tokens.index(self.state as usize);
    }

    pub fn retrieve_identifier(&mut self) -> String {
        match *self.get_token() {
            Token::Identifier(ref id) => return id.to_string(),
            _ => {
                panic!("ParseTree.retrieve_identifier: token is not identifier");
            }
        }
    }

    // pub fn read_stdin(&mut self) -> i32 {
    //     // return self.input_stream.remove(0);
    // }

    pub fn retrieve_integer(&mut self) -> &i32 {
        match *self.get_token() {
            Token::Integer(ref value) => return value,
            _ => panic!("ParseTree.retrieve_integer: token is not integer")
        }
    }

    pub fn push_statement(&mut self, statement: String) {
        for i in 0..(self.depth * 4) {
            print!(" ");
        }
        println!("{}", statement);
        self.statements.push(statement);
    }

    pub fn fetch_current_statement(&mut self) {
        for i in 0..(self.depth * 4) {
            print!(" ");
        }
        println!("{}", self.current_statement);
        self.statements.push(self.current_statement.to_owned());
        self.current_statement = "".to_string();
    }

    pub fn insert_variable(&mut self, identifier: String, value: i32) {
        self.memory.insert(identifier, value);
    }

    pub fn retrieve_variable(&mut self, identifier: &str) -> i32 {
        match self.memory.get(identifier) {
            Some(&value) => return value,
            _ => panic!("retrieve_variable: variable not present in HashMap"),
        }
    }

    pub fn display_variables(&mut self) {
        println!("\nVariables updated. Shown below.");
        for (identifier, value) in &self.memory {
            println!("{}: {}", identifier, value);
        }
        println!("");
    }

    pub fn set_state(&mut self, state: u32) {
        self.state = state;
    }

    pub fn get_depth(&mut self) -> u32 {
        self.depth
    }

    pub fn descend(&mut self) {
        self.depth += 1;
    }

    pub fn ascend(&mut self) {
        self.depth -= 1;
    }
}

pub fn init_parser(file_tokens: Vec<Token>, stdin: Vec<i32>) {
    let mut this_parse_tree = ParseTree {
        tokens: file_tokens.clone(),
        memory: HashMap::new(),
        current_statement: "".to_string(),
        statements: Vec::new(),
        context: Vec::new(),
        state: 0,
        depth: 0
    };

    parse_prog(&mut this_parse_tree);

    /*
     * For writing tests later:
     *
     * println!("{}", this_parse_tree.get_token());
     * edit_tree(&mut this_parse_tree);
     * println!("{}", this_parse_tree.get_token());
     * println!("{}", this_parse_tree.retrieve_variable("testing"));
     * println!("{}", this_parse_tree.retrieve_variable("should_not_work"));
     */
}

fn parse_prog(mut tree: &mut ParseTree) {

    // program <DECL SEQ> begin <STMT SEQ> end

    if tree.get_token().eq(&Token::Program) {
        tree.push_statement("program".to_string());
        tree.descend();
        tree.next(); // Consume the 'program' keyword
        parse_decl_seq(&mut tree);
        tree.ascend();
        if tree.get_token().eq(&Token::Begin) {
            tree.push_statement("begin".to_string());
            tree.descend();
            tree.next(); // Consume the 'begin' keyword
            parse_stmt_seq(&mut tree);
            if tree.get_token().eq(&Token::End) {
                tree.ascend();
                tree.push_statement("end".to_string());
            } else {
                panic!("parse_prog: expected 'end'");
            }
        } else {
            panic!("parse_prog: expected 'begin'");
        }
    } else {
        panic!("program @ {} : {}", tree.state, tree.tokens[tree.state as usize]);
    }
}

fn parse_decl_seq(mut tree: &mut ParseTree) {

    // <DECL>
    // <DECL> <DECL SEQ>

    parse_decl(&mut tree); // parses declaration and moves on

    if tree.get_token().eq(&Token::Int) {
        parse_decl_seq(&mut tree);
    }
}

fn parse_decl(mut tree: &mut ParseTree) {

    // int <ID LIST>;

    if tree.get_token().eq(&Token::Int) {
        tree.current_statement.push_str("int ");
        tree.next(); // consume int
        parse_id_list(&mut tree);
        if tree.get_token().eq(&Token::Semicolon) {
            tree.current_statement.push(';');
            tree.fetch_current_statement();
            tree.next(); // an entire declaration has been consumed; move on
        } else {
            panic!("parse_decl: expected ';'");
        }
    } else {
        panic!("parse_decl: expected 'int'");
    }
}

fn parse_stmt_seq(mut tree: &mut ParseTree) {

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
        parse_in(&mut tree);
    } else if tree.get_token().eq(&Token::Write) {
        match_flag = true;
        parse_out(&mut tree);
    } else if tree.get_token().eq(&Token::If) {
        match_flag = true;
        parse_if(&mut tree);
    } else if tree.get_token().eq(&Token::While) {
        match_flag = true;
        parse_loop(&mut tree);
    }

    if id_flag {
        parse_assign(&mut tree);
    }

    if match_flag {
        parse_stmt_seq(&mut tree);
    }
}

fn parse_id_list(mut tree: &mut ParseTree) {

    // <ID>
    // <ID>, <ID LIST>

    let mut identifier: String;
    let mut id_result: (String, ParseTree);
    parse_id(&mut tree);

    if tree.get_token().eq(&Token::Comma) {
        tree.current_statement.push_str(", ");
        tree.next(); // consume comma
        parse_id_list(&mut tree);
    }
}

fn parse_assign(mut tree: &mut ParseTree){

    // <ID> = <EXP>;

    tree.context.clear(); // New context

    let current_id: String = tree.retrieve_identifier();

    parse_id(&mut tree);
    if tree.get_token().eq(&Token::Assignment) {
        tree.current_statement.push_str(" = ");
        tree.next(); // throw away =
        let result: i32 = parse_exp(&mut tree);
        tree.insert_variable(current_id.clone(), result.clone());
        // tree.output_stream.push(format!("{} = {}", current_id, result.to_string()));
        if tree.get_token().eq(&Token::Semicolon) {
            tree.current_statement.push(';');
            tree.fetch_current_statement();
            tree.next(); // throw away semicolon
        } else {
            panic!("parse_assign: missing ';'");
        }
    } else {
        panic!("parse_assign: expected '='");
    }
}

fn parse_if(mut tree: &mut ParseTree) {

    // if <COND> then <STMT SEQ> end;
    // if <COND> then <STMT SEQ> else <STMT SEQ> end;

    tree.current_statement.push_str("if ");
    tree.next();
    parse_cond(&mut tree);
    if tree.get_token().eq(&Token::Then) {
        tree.current_statement.push_str(" then");
        tree.fetch_current_statement();
        tree.descend();
        tree.next();
        parse_stmt_seq(&mut tree);
        tree.ascend();
        if tree.get_token().eq(&Token::End) {
            tree.current_statement.push_str("end");
            tree.next();
            if tree.get_token().eq(&Token::Semicolon) {
                tree.current_statement.push_str(";");
                tree.fetch_current_statement();
                tree.next();
            } else {
                panic!("parse_if: expected ';'");
            }
        } else if tree.get_token().eq(&Token::Else) {
            tree.current_statement.push_str("else");
            tree.fetch_current_statement();
            tree.descend();
            tree.next();
            parse_stmt_seq(&mut tree);
            tree.ascend();
            if tree.get_token().eq(&Token::End) {
                tree.current_statement.push_str("end");
                tree.next();
                if tree.get_token().eq(&Token::Semicolon) {
                    tree.current_statement.push_str(";");
                    tree.fetch_current_statement();
                    tree.next();
                } else {
                    panic!("parse_if: expected ';'");
                }
            } else {
                panic!("parse_if: expected 'end'");
            }
        } else {
            panic!("parse_if: expected 'end' or 'else'");
        }
    } else {
        panic!("parse_if: expected 'then'");
    }
}

fn parse_loop(mut tree: &mut ParseTree) {

    // while <COND> loop <STMT SEQ> end;

    tree.current_statement.push_str("while ");
    tree.next();
    let result: bool = parse_cond(&mut tree);
    if tree.get_token().eq(&Token::Loop) {
        tree.current_statement.push_str(" loop");
        tree.fetch_current_statement();
        tree.next();
        tree.descend();
        parse_stmt_seq(&mut tree);
        tree.ascend();
        if tree.get_token().eq(&Token::End) {
            tree.current_statement.push_str("end");
            tree.next();
            if tree.get_token().eq(&Token::Semicolon) {
                tree.current_statement.push_str(";");
                tree.fetch_current_statement();
                tree.next();
            } else {
                panic!("parse_loop: expected ';'");
            }
        } else {
            panic!("parse_loop: expected 'end'");
        }
    } else {
        panic!("parse_loop: expected 'loop'");
    }
}

fn parse_in(mut tree: &mut ParseTree) {

    // read <ID LIST>;

    tree.context.clear(); // New context
    tree.current_statement.push_str("read ");
    tree.next(); // eating the 'read' token

    parse_id_list(&mut tree); // Filling context

    // We are actually doing the reading here!
    // for id in tree.context.clone() {
    //     // let val: i32 = tree.read_stdin();
    //     // tree.insert_variable(id.clone(), val.clone());
    //     // tree.output_stream.push(format!("{} = {}", id, val));
    // }

    if tree.get_token().eq(&Token::Semicolon) {
        tree.current_statement.push(';');
        tree.fetch_current_statement();
        tree.next();
    } else {
        panic!("parse_in: expected ';'");
    }
}

fn parse_out(mut tree: &mut ParseTree) {

    // write <ID LIST>;

    tree.context.clear(); // New context
    tree.current_statement.push_str("write ");
    tree.next();

    parse_id_list(&mut tree);
    // for id in tree.context.clone() {
    //     let result: String = tree.retrieve_variable(&id).to_string();
    //     tree.output_stream.push(format!("{} = {}", id, result));
    // }

    if tree.get_token().eq(&Token::Semicolon) {
        tree.current_statement.push(';');
        tree.fetch_current_statement();
        tree.next();
    } else {
        panic!("parse_out: expected ';'");
    }
}

fn parse_cond(mut tree: &mut ParseTree) -> bool {

    // <COMP>
    // !<COMP>
    // [<COND> && <COND>]
    // [<COND> || <COND>]

    let mut result: bool = false;

    if tree.get_token().eq(&Token::LeftSquare) {
        tree.current_statement.push('[');
        tree.next();
        result = parse_cond(&mut tree);
        if tree.get_token().eq(&Token::LogicalAnd) {
            tree.current_statement.push_str(" && ");
            tree.next();
            let result_and: bool = parse_cond(&mut tree);
            result = result && result_and;
            if tree.get_token().eq(&Token::RightSquare) {
                tree.current_statement.push(']');
                tree.next();
            } else {
                panic!("parse_cond: expected ']'");
            }
        } else if tree.get_token().eq(&Token::LogicalOr) {
            tree.current_statement.push_str(" || ");
            tree.next();
            let result_or: bool = parse_cond(&mut tree);
            result = result || result_or;
            if tree.get_token().eq(&Token::RightSquare) {
                tree.current_statement.push(']');
                tree.next();
            } else {
                panic!("parse_cond: expected ']'");
            }
        } else {
            panic!("parse_cond: expected '+' or '-'");
        }
    } else if tree.get_token().eq(&Token::Exclamation) {
        tree.current_statement.push_str("!");
        result = !parse_comp(&mut tree);
    } else {
        result = parse_comp(&mut tree);
    }

    result
}

fn parse_comp(mut tree: &mut ParseTree) -> bool {

    // (<OP> <COMP OP> <OP>)

    let mut result: bool = false;
    let mut op_a: i32;
    let mut op_b: i32;
    let mut cmp: u32;

    if tree.get_token().eq(&Token::LeftParen) {
        tree.current_statement.push('(');
        tree.next();
        op_a = parse_op(&mut tree);
        cmp = parse_comp_op(&mut tree);
        op_b = parse_op(&mut tree);
        if tree.get_token().eq(&Token::RightParen) {
            tree.current_statement.push(')');
            tree.next();
        }
    } else {
        panic!("parse_comp: expected '('");
    }

    match cmp {
        0 => result = op_a == op_b,
        1 => result = op_a != op_b,
        2 => result = op_a <  op_b,
        3 => result = op_a <= op_b,
        4 => result = op_a >  op_b,
        5 => result = op_a >= op_b,
        _ => panic!("parse_comp: invalid comp_op")
    }

    result
}

fn parse_exp(mut tree: &mut ParseTree) -> i32 {

    // <TRM>
    // <TRM> + <EXP>
    // <TRM> - <EXP>

    let mut result: i32 = 0;

    result = parse_trm(&mut tree);

    if tree.get_token().eq(&Token::Addition) {
        // handle addition, garble
        tree.current_statement.push_str(" + ");
        tree.next();
        result += parse_exp(&mut tree);
    } else if tree.get_token().eq(&Token::Subtraction) {
        // handle subtraction
        tree.current_statement.push_str(" - ");
        tree.next();
        result -= parse_exp(&mut tree);
    }

    result
}

fn parse_trm(mut tree: &mut ParseTree) -> i32 {

    // <OP>
    // <OP> * <TRM>

    let mut result: i32 = parse_op(&mut tree);

    if tree.get_token().eq(&Token::Multiplication) {
        // handle multiplication, garble
        tree.current_statement.push_str(" * ");
        tree.next();
        result *= parse_trm(&mut tree);
    }

    result
}

fn parse_op(mut tree: &mut ParseTree) -> i32 {

    // <NO>
    // <ID>
    // (<EXP>)

    let mut result:i32 = 0;

    let mut id_flag: bool = false;
    let mut int_flag: bool = false;

    if tree.get_token().eq(&Token::LeftParen) {
        tree.current_statement.push('(');
        tree.next(); // left paren
        result = parse_exp(&mut tree);
        if tree.get_token().eq(&Token::RightParen) {
            tree.current_statement.push(')');
            tree.next();
        } else {
            panic!("parse_op: missing ')'");
        }
    } else {
        match tree.get_token() {
            &Token::Identifier(ref id) => id_flag = true,
            &Token::Integer(ref i) => int_flag = true,
            _ => panic!("parse_op: token is not identifier")
        }

        if id_flag {
            id_flag = false;
            let current_id: String = tree.retrieve_identifier().clone();
            result = tree.retrieve_variable(&current_id);
            parse_id(&mut tree);
        } else if int_flag {
            int_flag = false;
            result = *tree.retrieve_integer();
            parse_int(&mut tree);
        } else {
            panic!("parse_op: invalid argument")
        }
    }

    result
}

fn parse_comp_op(mut tree: &mut ParseTree) -> u32 {

    // This is like a literal "opcode" we will be using
    // to perform the actual comparisons later on
    let mut result: u32 = 0;

    match tree.get_token() {
        &Token::LogicalEquality => {
            tree.current_statement.push_str(" == ");
            result = 0;
        },
        &Token::LogicalInequality => {
            tree.current_statement.push_str(" != ");
            result = 1;
        },
        &Token::LessThan => {
            tree.current_statement.push_str(" < ");
            result = 2;
        },
        &Token::LessThanEqual => {
            tree.current_statement.push_str(" <= ");
            result = 3;
        },
        &Token::GreaterThan => {
            tree.current_statement.push_str(" > ");
            result = 4;
        },
        &Token::GreaterThanEqual => {
            tree.current_statement.push_str(" >= ");
            result = 5;
        },
        _ => panic!("parse_comp_op: unexpected comp op")
    }

    tree.next();

    result
}

fn parse_id(mut tree: &mut ParseTree) {
    let identifier: String = tree.retrieve_identifier();
    tree.current_statement.push_str(&identifier);
    tree.context.push(identifier.clone());
    if !tree.memory.contains_key(&identifier.clone()) {
        tree.insert_variable(identifier.clone(), 0);
    }
    tree.next();
}

fn parse_int(mut tree: &mut ParseTree) {
    let integer: i32 = *tree.retrieve_integer();
    tree.current_statement.push_str(&integer.to_string());
    tree.next();
}
