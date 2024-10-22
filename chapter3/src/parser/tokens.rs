use super::{digits::Digit, errors::Error, ops::Op};
use std::{fmt, str::FromStr};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token {
    Digit(Digit),
    Op(Op),
    InputInt,
    Print,
    BrackO,
    BrackC,
    Sep,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Digit(d) => d.fmt(f),
            Token::Op(op) => op.fmt(f),
            Token::InputInt => f.write_str("input_int"),
            Token::Print => f.write_str("print"),
            Token::BrackO => f.write_str("("),
            Token::BrackC => f.write_str(")"),
            Token::Sep => f.write_str(" "),
        }
    }
}

impl FromStr for Token {
    type Err = Error;
    fn from_str(s: &str) -> Result<Token, Self::Err> {
        match s.trim() {
            "input_int" => Ok(Token::InputInt),
            "print" => Ok(Token::Print),
            "(" => Ok(Token::BrackO),
            ")" => Ok(Token::BrackC),
            _ => {
                if let Ok(d) = s.parse::<Digit>() {
                    Ok(Token::Digit(d))
                } else if let Ok(op) = s.parse::<Op>() {
                    Ok(Token::Op(op))
                } else {
                    Err(Error::NotAToken {
                        input: s.to_owned(),
                    })
                }
            }
        }
    }
}
