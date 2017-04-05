fn parse_prog() {
    // program <DECL SEQ> begin <STMT SEQ> end
}

fn parse_decl_seq() {
    // <DECL>
    // <DECL> <DECL SEQ>
}

fn parse_decl() {
    // int <ID LIST>
}

fn parse_stmt_seq() {
    // <STMT>
    // <STMT> <STMT SEQ>
}

fn parse_id_list() {
    // <ID>
    // <ID>, <ID LIST>
}

fn parse_stmt() {
    // <ASSIGN>
    // <IF>
    // <LOOP>
    // <IN>
    // <OUT>
}

fn parse_assign() {
    // <ID> = <EXP>;
}

fn parse_if() {
    // if <COND> then <STMT SEQ> end;
    // if <COND> then <STMT SEQ> else <STMT SEQ> end;
}

fn parse_loop() {
    // while <COND> loop <STMT SEQ> end;
}

fn parse_in() {
    // read <ID LIST>;
}

fn parse_out() {
    // write <ID LIST>;
}

fn parse_cond() {
    // <COMP>
    // !<COMP>
    // [<COND> && <COND>]
    // [<COND> || <COND>]
}

fn parse_comp() {
    // (<OP> <COMP OP> <OP)
}

fn parse_exp() {
    // <TRM>
    // <TRM> + <EXP>
    // <TRM> - <EXP>
}

fn parse_trm() {
    // <OP>
    // <OP> * <TRM>
}

fn parse_op() {
    // <NO>
    // <ID>
    // (<EXP>)
}

fn parse_comp_op() {
    // !=
    // ==
    // <
    // >
    // <=
    // >=
}

fn parse_id() {
    // I think we have handled this.
}
