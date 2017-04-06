#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

use std::ops::Index;
use tokenizer::Token;
use std::collections::HashMap;

#[derive(Clone)]
struct ParseTree {
    tokens: Vec<Token>,
    input_stream: Vec<i32>,
    memory: HashMap<String, i32>,
    current_statement: String,
    statements: Vec<String>,
    state: u32,
    depth: u32
}

impl ParseTree {
    fn next(&mut self) {
        self.state += 1;
    }

    fn get_token(&mut self) -> &Token {
        return &self.tokens.index(self.state as usize);
    }

    fn retrieve_identifier(&mut self) -> String {
        match *self.get_token() {
            Token::Identifier(ref id) => return id.to_string(),
            _ => {
                panic!("ParseTree.retrieve_identifier: token is not identifier");
            }
        }
    }

    fn read_stdin(&mut self) -> i32 {
        return self.input_stream.remove(0);
    }

    fn retrieve_integer(&mut self) -> &i32 {
        match *self.get_token() {
            Token::Integer(ref value) => return value,
            _ => panic!("ParseTree.retrieve_integer: token is not integer")
        }
    }

    fn push_statement(&mut self, statement: String) {
        for i in 0..(self.depth * 4) {
            print!(" ");
        }
        println!("{}", statement);
        self.statements.push(statement);
    }

    fn fetch_current_statement(&mut self) {
        for i in 0..(self.depth * 4) {
            print!(" ");
        }
        println!("{}", self.current_statement);
        self.statements.push(self.current_statement.to_owned());
        self.current_statement = "".to_string();
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

    fn display_variables(&mut self) {
        println!("\nVariables updated. Shown below.");
        for (identifier, value) in &self.memory {
            println!("{}: {}", identifier, value);
        }
        println!("");
    }

    fn descend(&mut self) {
        self.depth += 1;
    }

    fn ascend(&mut self) {
        self.depth -= 1;
    }
}

pub fn init_parser(file_tokens: Vec<Token>, stdin: Vec<i32>) {
    let mut this_parse_tree = ParseTree {
        tokens: file_tokens.clone(),
        input_stream: stdin.clone(),
        memory: HashMap::new(),
        current_statement: "".to_string(),
        statements: Vec::new(),
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
                tree.display_variables();
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
        parse_id(&mut tree);
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

    if tree.get_token().eq(&Token::Assignment) {
        tree.current_statement.push_str(" = ");
        tree.next(); // throw away =
        parse_exp(&mut tree);
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
    parse_cond(&mut tree);
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

    tree.current_statement.push_str("read ");
    tree.next(); // eating the 'read' token

    parse_id_list(&mut tree);
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

    tree.current_statement.push_str("write ");
    tree.next();

    parse_id_list(&mut tree);
    if tree.get_token().eq(&Token::Semicolon) {
        tree.current_statement.push(';');
        tree.fetch_current_statement();
        tree.next();
    } else {
        panic!("parse_out: expected ';'");
    }
}

fn parse_cond(mut tree: &mut ParseTree) {

    // <COMP>
    // !<COMP>
    // [<COND> && <COND>]
    // [<COND> || <COND>]

    if tree.get_token().eq(&Token::LeftSquare) {
        tree.current_statement.push('[');
        tree.next();
        parse_cond(&mut tree);
        if tree.get_token().eq(&Token::LogicalAnd) {
            tree.current_statement.push_str(" && ");
            tree.next();
            parse_cond(&mut tree);
            if tree.get_token().eq(&Token::RightSquare) {
                tree.current_statement.push(']');
                tree.next();
            } else {
                panic!("parse_cond: expected ']'");
            }
        } else if tree.get_token().eq(&Token::LogicalOr) {
            tree.current_statement.push_str(" || ");
            tree.next();
            parse_cond(&mut tree);
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
        parse_comp(&mut tree);
    } else {
        parse_comp(&mut tree);
    }
}

fn parse_comp(mut tree: &mut ParseTree) {

    // (<OP> <COMP OP> <OP>)

    if tree.get_token().eq(&Token::LeftParen) {
        tree.current_statement.push('(');
        tree.next();
        parse_op(&mut tree);
        parse_comp_op(&mut tree);
        parse_op(&mut tree);
        if tree.get_token().eq(&Token::RightParen) {
            tree.current_statement.push(')');
            tree.next();
        }
    } else {
        panic!("parse_comp: expected '('");
    }
}

fn parse_exp(mut tree: &mut ParseTree) {

    // <TRM>
    // <TRM> + <EXP>
    // <TRM> - <EXP>

    parse_trm(&mut tree);

    if tree.get_token().eq(&Token::Addition) {
        // handle addition, garble
        tree.current_statement.push_str(" + ");
        tree.next();
        parse_exp(&mut tree);
    } else if tree.get_token().eq(&Token::Subtraction) {
        // handle subtraction
        tree.current_statement.push_str(" - ");
        tree.next();
        parse_exp(&mut tree);
    }
}

fn parse_trm(mut tree: &mut ParseTree) {

    // <OP>
    // <OP> * <TRM>

    parse_op(&mut tree);

    if tree.get_token().eq(&Token::Multiplication) {
        // handle multiplication, garble
        tree.current_statement.push_str(" * ");
        tree.next();
        parse_trm(&mut tree);
    }
}

fn parse_op(mut tree: &mut ParseTree) {

    // <NO>
    // <ID>
    // (<EXP>)

    let mut id_flag: bool = false;
    let mut int_flag: bool = false;

    if tree.get_token().eq(&Token::LeftParen) {
        tree.current_statement.push('(');
        tree.next(); // left paren
        parse_exp(&mut tree);
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
            parse_id(&mut tree);
        } else if int_flag {
            int_flag = false;
            parse_int(&mut tree);
        } else {
            panic!("parse_op: invalid argument")
        }
    }
}

fn parse_comp_op(mut tree: &mut ParseTree) {

    match tree.get_token() {
        &Token::LogicalEquality => {
            tree.current_statement.push_str(" == ");
        },
        &Token::LogicalInequality => {
            tree.current_statement.push_str(" != ");
        },
        &Token::LessThan => {
            tree.current_statement.push_str(" < ");
        },
        &Token::LessThanEqual => {
            tree.current_statement.push_str(" <= ");
        },
        &Token::GreaterThan => {
            tree.current_statement.push_str(" > ");
        },
        &Token::GreaterThanEqual => {
            tree.current_statement.push_str(" >= ");
        },
        _ => panic!("parse_comp_op: unexpected comp op")
    }

    tree.next();
}

fn parse_id(mut tree: &mut ParseTree) {
    let identifier: String = tree.retrieve_identifier();
    tree.current_statement.push_str(&identifier);
    tree.insert_variable(identifier, 0);
    tree.next();
}

fn parse_int(mut tree: &mut ParseTree) {
    let integer: i32 = *tree.retrieve_integer();
    tree.current_statement.push_str(&integer.to_string());
    tree.next();
}
