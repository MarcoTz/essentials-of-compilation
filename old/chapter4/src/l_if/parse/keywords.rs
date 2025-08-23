use super::errors::{Error, ParseRes};
use nom::{bytes::complete::tag, character::complete::alphanumeric1, Err};
use std::{fmt, str::FromStr};
#[derive(Debug)]
pub enum Keyword {
    Read,
    Let,
    True,
    False,
    And,
    Or,
    Not,
    If,
}

pub fn parse_keyword(input: &str, kw: Keyword) -> ParseRes<()> {
    let (rem, _) = tag(kw.to_string().as_str())(input)?;
    Ok((rem, ()))
}

pub fn parse_non_keyword(input: &str) -> ParseRes<String> {
    let (rem, var) = alphanumeric1(input)?;
    match var.parse::<Keyword>() {
        Ok(kw) => Err(Err::Error(Error::UsedKeyword(kw))),
        Err(_) => Ok((rem, var.to_owned())),
    }
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Keyword::Read => f.write_str("read"),
            Keyword::Let => f.write_str("let"),
            Keyword::True => f.write_str("true"),
            Keyword::False => f.write_str("false"),
            Keyword::And => f.write_str("and"),
            Keyword::Or => f.write_str("or"),
            Keyword::Not => f.write_str("not"),
            Keyword::If => f.write_str("if"),
        }
    }
}

impl FromStr for Keyword {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().trim() {
            "read" => Ok(Keyword::Read),
            "let" => Ok(Keyword::Let),
            "true" => Ok(Keyword::True),
            "false" => Ok(Keyword::False),
            "and" => Ok(Keyword::And),
            "or" => Ok(Keyword::Or),
            "not" => Ok(Keyword::Not),
            "if" => Ok(Keyword::If),
            _ => Err(Error::UnknownKeyword(s.to_owned())),
        }
    }
}

#[cfg(test)]
mod keyword_tests {
    use super::{parse_keyword, parse_non_keyword, Keyword};

    #[test]
    fn parse_kw() {
        let result = parse_keyword("read", Keyword::Read);
        assert!(result.is_ok())
    }

    #[test]
    fn parse_nonkw() {
        let result = parse_non_keyword("notkeyword");
        assert!(result.is_ok())
    }

    #[test]
    fn parse_kw_fail() {
        let result = parse_keyword("notread", Keyword::Read);
        assert!(result.is_err())
    }

    #[test]
    fn parse_nonkw_fail() {
        let result = parse_non_keyword("read");
        assert!(result.is_err());
    }
}
