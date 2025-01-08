use crate::Var;

pub enum Atom {
    Int(i64),
    Var(Var),
    Bool(bool),
}
