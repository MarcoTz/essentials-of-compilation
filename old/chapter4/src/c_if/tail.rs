use super::{Atom, Cmp, Exp, Label, Stmt};

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
