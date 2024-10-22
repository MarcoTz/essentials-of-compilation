use super::errors::Error;
use std::{fmt, str::FromStr};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Op {
    Plus,
    Minus,
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Op::Plus => f.write_str("+"),
            Op::Minus => f.write_str("-"),
        }
    }
}

impl FromStr for Op {
    type Err = Error;
    fn from_str(s: &str) -> Result<Op, Self::Err> {
        match s.trim() {
            "+" => Ok(Op::Plus),
            "-" => Ok(Op::Minus),
            _ => Err(Error::NotAnOp {
                input: s.to_owned(),
            }),
        }
    }
}
