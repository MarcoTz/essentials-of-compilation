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

impl From<i64> for Exp {
    fn from(i: i64) -> Exp {
        Exp::Constant(i)
    }
}

impl From<String> for Exp {
    fn from(st: String) -> Exp {
        Exp::Name(st)
    }
}

pub enum Stmt {
    Assign { name: Var, exp: Exp },
    Print(Exp),
    Exp(Exp),
}

pub struct Module {
    pub stmts: Vec<Stmt>,
}
