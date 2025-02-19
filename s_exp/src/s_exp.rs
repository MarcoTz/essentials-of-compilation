use super::errors::Error;
use std::{fmt, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
pub enum SExp {
    Symbol(String),
    Expr(String, Box<SExp>),
}

impl FromStr for SExp {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().next() != Some('(') {
            if s.contains(" ") || s.contains("(") {
                return Err(Error::MissingParens);
            } else {
                return Ok(SExp::Symbol(s.to_owned()));
            }
        }

        let (parens_o, rest) = s.split_at(1);
        if parens_o != "(" {
            return Err(Error::MissingParens);
        }

        let (rest, parens_c) = rest.split_at(rest.len() - 1);
        if parens_c != ")" {
            return Err(Error::MissingParens);
        }

        let next_split = match rest.find("(") {
            None => return Ok(SExp::Symbol(rest.to_owned())),
            Some(ind) => ind,
        };
        let (fst, rest) = rest.split_at(next_split);
        let rest_parsed = rest.parse::<SExp>()?;
        Ok(SExp::Expr(fst.trim().to_owned(), Box::new(rest_parsed)))
    }
}

impl fmt::Display for SExp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SExp::Symbol(c) => write!(f, "({c})"),
            SExp::Expr(fst, rst) => write!(f, "({fst} {rst})"),
        }
    }
}

#[cfg(test)]
mod s_exp_tests {
    use super::SExp;

    #[test]
    fn parse_sym() {
        let result = "Nil".parse::<SExp>().unwrap();
        let expected = SExp::Symbol("Nil".to_owned());
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_parens() {
        let result = "(Nil)".parse::<SExp>().unwrap();
        let expected = SExp::Symbol("Nil".to_owned());
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_list() {
        let result = "(1 (2 (Nil)))".parse::<SExp>().unwrap();
        let expected = SExp::Expr(
            "1".to_owned(),
            Box::new(SExp::Expr(
                "2".to_owned(),
                Box::new(SExp::Symbol("Nil".to_owned())),
            )),
        );
        assert_eq!(result, expected)
    }
}
