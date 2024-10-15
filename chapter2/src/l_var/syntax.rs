use super::{BinOp, UnaryOp, Var};

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
    Assign { var: Var, exp: Exp },
    Print(Exp),
    Exp(Exp),
}
pub type Module = Vec<Stmt>;
