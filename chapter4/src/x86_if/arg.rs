use super::{ByteReg, Reg};

pub enum Arg {
    Immediate(i64),
    Reg(Reg),
    Deref(Reg, i64),
    ByteReg(ByteReg),
}

pub enum CC {
    E,
    L,
    Le,
    G,
    Ge,
}
