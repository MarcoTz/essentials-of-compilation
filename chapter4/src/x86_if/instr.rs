use super::{Arg, CC};
use crate::Label;

pub enum Instr {
    AddQ(Arg, Arg),
    SubQ(Arg, Arg),
    NegQ(Arg),
    MovQ(Arg),
    PushQ(Arg),
    PopQ(Arg),
    CallQ(Label, usize),
    RetQ,
    Jmp(Label),
    XorQ(Arg, Arg),
    CmpQ(Arg, Arg),
    Set(CC, Arg),
    MovZBq(Arg, Arg),
    JmpIf(CC, Label),
}
