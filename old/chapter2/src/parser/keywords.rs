use super::errors::{Error, ParseRes};
use nom::{bytes::complete::tag, character::complete::alphanumeric1, Err};
use std::{fmt, str::FromStr};

#[derive(Debug)]
pub enum Keyword {
    Let,
    InputInt,
}

impl Keyword {
    pub fn all() -> Vec<Keyword> {
        vec![Keyword::Let, Keyword::InputInt]
    }
}

pub fn parse_keyword(input: &str, kw: Keyword) -> ParseRes<()> {
    let (rem, _) = tag(kw.to_string().as_str())(input)?;
    Ok((rem, ()))
}

pub fn parse_non_keyword(input: &str) -> ParseRes<String> {
    let (rem, var) = alphanumeric1(input)?;
    match var.parse::<Keyword>() {
        Ok(kw) => Err(Err::Error(Error::VarIsKeyword(kw))),
        Err(_) => Ok((rem, var.to_owned())),
    }
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Keyword::Let => f.write_str("let"),
            Keyword::InputInt => f.write_str("input_int"),
        }
    }
}

impl FromStr for Keyword {
    type Err = Error;
    fn from_str(s: &str) -> Result<Keyword, Self::Err> {
        match s.trim() {
            "let" => Ok(Keyword::Let),
            "input_int" => Ok(Keyword::InputInt),
            _ => Err(Error::NotAKeyword(s.to_owned())),
        }
    }
}

#[cfg(test)]
mod keyword_tests {
    use super::{parse_keyword, Keyword};

    #[test]
    fn parse_let() {
        parse_keyword("let", Keyword::Let).unwrap();
    }

    #[test]
    fn parse_input() {
        parse_keyword("input_int", Keyword::InputInt).unwrap();
    }

    #[test]
    fn let_into() {
        "let".parse::<Keyword>().unwrap();
    }

    #[test]
    fn input_into() {
        "input_int".parse::<Keyword>().unwrap();
    }
}
