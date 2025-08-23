use super::{atm::Atom, ops::Op};

pub enum Exp {
    Atom(Atom),
    PrimOp { op: Op, args: Vec<Atom> },
}
