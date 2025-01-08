pub enum Op {
    Read,
    Neg,
    Add,
    Sub,
    Not,
    Cmp(Cmp),
}

pub enum Cmp {
    Equal,
    Less,
    LessEq,
    Greater,
    GreaterEq,
}
