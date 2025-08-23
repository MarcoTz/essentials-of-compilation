use super::{errors::Error, parser::parse_sexpr, Symbol};
use std::{fmt, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
pub enum SExp<T>
where
    T: Symbol,
{
    Symbol(T),
    Expr(Vec<SExp<T>>),
}

impl<T: Symbol> FromStr for SExp<T> {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_sexpr(s)
    }
}

impl<T: Symbol + fmt::Display> fmt::Display for SExp<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SExp::Symbol(c) => write!(f, "({c})"),
            SExp::Expr(exps) => {
                let inner_str = exps
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>()
                    .join(" ");
                write!(f, "({inner_str})")
            }
        }
    }
}
