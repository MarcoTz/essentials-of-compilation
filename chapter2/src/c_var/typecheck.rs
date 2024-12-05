use super::{Atm, Exp, Program, Stmt, Tail, Var};
use std::collections::HashMap;

#[derive(Debug)]
pub enum Type {
    Int,
}

pub trait Typecheck: Sized {
    fn check(&self) -> HashMap<Var, Type>;
}

pub fn check(prog: &mut Program) {
    let vars = prog.check();
    prog.types = vars;
}

impl Typecheck for Program {
    fn check(&self) -> HashMap<Var, Type> {
        let mut vars = HashMap::new();
        self.blocks
            .iter()
            .map(|(_, tail)| vars.extend(tail.check()));
        vars
    }
}

impl Typecheck for Tail {
    fn check(&self) -> HashMap<Var, Type> {
        match self {
            Tail::Return(e) => e.check(),
            Tail::Seq(stmt, tl) => {
                let mut vars = stmt.check();
                vars.extend(tl.check());
                vars
            }
        }
    }
}

impl Typecheck for Exp {
    fn check(&self) -> HashMap<Var, Type> {
        match self {
            Exp::Atm(a) => a.check(),
            Exp::Read => HashMap::new(),
            Exp::UnaryOp { op: _, exp } => exp.check(),
            Exp::BinOp { exp1, op: _, exp2 } => {
                let mut vars = exp1.check();
                vars.extend(exp2.check());
                vars
            }
        }
    }
}

impl Typecheck for Stmt {
    fn check(&self) -> HashMap<Var, Type> {
        match self {
            Stmt::Assign { var, exp } => {
                let mut vars = exp.check();
                vars.insert(var.clone(), Type::Int);
                vars
            }
        }
    }
}

impl Typecheck for Atm {
    fn check(&self) -> HashMap<Var, Type> {
        match self {
            Atm::Int(_) => HashMap::new(),
            Atm::Var(v) => HashMap::from([(v.to_owned(), Type::Int)]),
        }
    }
}
