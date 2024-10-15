pub enum Reg {
    Rsp,
    Rbp,
    Rax,
    Rbx,
    Rcx,
    Rdx,
    Rsi,
    Rdi,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}

pub enum Arg {
    Intermediate(i64),
    Reg(Reg),
    Deref(Reg, i64),
}

pub type Label = String;

pub enum Instr {
    AddQ(Arg, Arg),
    SubQ(Arg, Arg),
    Negq(Arg),
    MovQ(Arg, Arg),
    CallQ(Label, i64),
    PushQ(Arg),
    PopQ(Arg),
    RetQ,
    Jump(Label),
}

pub type Prog = Vec<Instr>;
