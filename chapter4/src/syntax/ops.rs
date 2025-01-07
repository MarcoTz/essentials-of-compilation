use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Cmp {
    Equal,
    Less,
    LessEq,
    Greater,
    GreaterEq,
}

impl Cmp {
    pub fn apply(&self, fst: i64, snd: i64) -> bool {
        match self {
            Cmp::Equal => fst == snd,
            Cmp::Less => fst < snd,
            Cmp::LessEq => fst <= snd,
            Cmp::Greater => fst > snd,
            Cmp::GreaterEq => fst >= snd,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Op {
    Read,
    Plus,
    Sub,
    Neg,
    Cmp(Cmp),
    And,
    Or,
    Not,
}

impl Op {
    pub fn arity(&self) -> usize {
        match self {
            Op::Read => 0,
            Op::Plus => 2,
            Op::Neg => 1,
            Op::Sub => 2,
            Op::Cmp(_) => 2,
            Op::And => 2,
            Op::Or => 2,
            Op::Not => 1,
        }
    }
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
            Op::Sub => f.write_str("-"),
            Op::Cmp(cmp) => cmp.fmt(f),
            Op::And => f.write_str("and"),
            Op::Or => f.write_str("or"),
            Op::Not => f.write_str("not"),
        }
    }
}
