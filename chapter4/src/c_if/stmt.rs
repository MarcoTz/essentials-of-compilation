use super::exp::Exp;
use crate::Var;

pub enum Stmt {
    Assign { var: Var, exp: Exp },
}
