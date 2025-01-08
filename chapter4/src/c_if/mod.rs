use crate::Var;
use std::collections::HashMap;

pub type Label = String;

pub enum Atom {
    Int(i64),
    Var(Var),
    Bool(bool),
}

pub enum Op {
    Read,
    Neg,
    Add,
    Sub,
    Not,
    Cmp(Cmp),
}

pub enum Cmp {
    Equal,
    Less,
    LessEq,
    Greater,
    GreaterEq,
}

pub enum Exp {
    Atom(Atom),
    PrimOp { op: Op, args: Vec<Atom> },
}

pub enum Stmt {
    Assign { var: Var, exp: Exp },
}

pub enum Tail {
    Return(Exp),
    Seq(Stmt, Box<Tail>),
    Goto(Label),
    IfStmt {
        cmp: Cmp,
        atm_left: Atom,
        atm_right: Atom,
        goto_then: Label,
        goto_else: Label,
    },
}

pub struct Program {
    pub blocks: HashMap<Label, Tail>,
}
