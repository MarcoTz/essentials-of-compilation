use super::{
    exp::Exp,
    functions::{Call, StmtFunction},
};
use std::fmt;

pub enum Stmt {
    Call(Call<StmtFunction>),
    Exp(Exp),
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Stmt::Call(call) => call.fmt(f),
            Stmt::Exp(e) => e.fmt(f),
        }
    }
}

impl Stmt {
    pub fn is_stmt(&self) -> bool {
        match self {
            Stmt::Call(Call {
                name: StmtFunction::Print,
                args,
            }) => args.len() == 1 && args.first().map(|e| e.is_exp()).unwrap_or(false),
            Stmt::Exp(e) => e.is_exp(),
        }
    }
}
