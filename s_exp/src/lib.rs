use std::str::FromStr;

pub mod errors;
pub mod l_int;
pub mod parser;
pub mod s_exp;

use errors::Error;
use s_exp::SExp;

pub trait Symbol: FromStr<Err = Error> {}

#[derive(Debug, PartialEq, Eq)]
pub struct StrSym {
    val: String,
}

impl StrSym {
    pub fn new(s: &str) -> StrSym {
        StrSym { val: s.to_owned() }
    }
}

impl Symbol for StrSym {}

impl FromStr for StrSym {
    type Err = Error;
    fn from_str(s: &str) -> Result<StrSym, Self::Err> {
        Ok(StrSym::new(s))
    }
}

impl From<&str> for StrSym {
    fn from(s: &str) -> StrSym {
        StrSym::new(s)
    }
}

pub trait Grammar {
    type Symbol: Symbol;
    type Target;

    fn to_target(&self, exp: SExp<Self::Symbol>) -> Result<Self::Target, Error>;
}
