use crate::{BinOp, UnaryOp, Var};

pub enum Exp {
    Name(Var),
    Constant(i64),
    InputInt,
    UnaryOp {
        op: UnaryOp,
        exp: Box<Exp>,
    },
    BinOp {
        exp1: Box<Exp>,
        op: BinOp,
        exp2: Box<Exp>,
    },
}

pub enum Stmt {
    Assign { name: Var, exp: Exp },
    Print(Exp),
    Exp(Exp),
}

pub struct Module {
    pub stmts: Vec<Stmt>,
}
