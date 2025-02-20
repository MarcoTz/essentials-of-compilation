use super::{errors::Error, s_exp::SExp, Grammar, Symbol};
use chapter1::syntax::{BinOp, Exp, Program, UnaryOp};
use std::{fmt, str::FromStr};

pub enum LIntSymbol {
    Num(i64),
    Read,
    Minus,
    Plus,
}

pub struct LInt;

impl Grammar for LInt {
    type Symbol = LIntSymbol;
    type Target = Program;

    fn to_target(&self, exp: SExp<Self::Symbol>) -> Result<Self::Target, Error> {
        todo!()
    }
}

impl FromStr for LIntSymbol {
    type Err = Error;
    fn from_str(s: &str) -> Result<LIntSymbol, Self::Err> {
        match s.to_lowercase().trim() {
            "read" => Ok(LIntSymbol::Read),
            "-" => Ok(LIntSymbol::Minus),
            "+" => Ok(LIntSymbol::Plus),
            s => s
                .parse::<i64>()
                .map(|i| LIntSymbol::Num(i))
                .map_err(|_| Error::UnexpectedSymbol(s.to_owned())),
        }
    }
}

impl fmt::Display for LIntSymbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LIntSymbol::Read => f.write_str("read"),
            LIntSymbol::Plus => f.write_str("+"),
            LIntSymbol::Minus => f.write_str("-"),
            LIntSymbol::Num(n) => write!(f, "{n}"),
        }
    }
}

impl Symbol for LIntSymbol {}
