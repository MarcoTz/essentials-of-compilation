use std::fmt;

#[derive(Debug)]
pub enum Cmp {
    Equal,
    Less,
    LessEq,
    Greater,
    GreaterEq,
}

#[derive(Debug)]
pub enum Op {
    Read,
    Plus,
    Neg,
    Cmp(Cmp),
    And,
    Or,
    Not,
}

impl From<Cmp> for Op {
    fn from(cmp: Cmp) -> Op {
        Op::Cmp(cmp)
    }
}

impl fmt::Display for Cmp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cmp::Equal => f.write_str("="),
            Cmp::Less => f.write_str("<"),
            Cmp::LessEq => f.write_str("<="),
            Cmp::Greater => f.write_str(">"),
            Cmp::GreaterEq => f.write_str(">="),
        }
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Op::Read => f.write_str("read"),
            Op::Plus => f.write_str("+"),
            Op::Neg => f.write_str("-"),
            Op::Cmp(cmp) => cmp.fmt(f),
            Op::And => f.write_str("and"),
            Op::Or => f.write_str("or"),
            Op::Not => f.write_str("not"),
        }
    }
}
