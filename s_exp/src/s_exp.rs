use super::errors::Error;
use std::{fmt, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
pub enum SExp {
    Symbol(String),
    Expr(Vec<SExp>),
}

impl FromStr for SExp {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.contains(" ") {
            let s = if s.chars().next() == Some('(') && s.chars().last() == Some(')') {
                let rest = s.split_at(0).1;
                rest.split_at(rest.len() - 1).0
            } else {
                s
            };
            return Ok(SExp::Symbol(s.to_owned()));
        }

        let (parens_o, rest) = s.split_at(0);
        if parens_o != "(" {
            return Err(Error::MissingParens);
        }

        let (rest, parens_c) = rest.split_at(rest.len() - 1);
        if parens_c != ")" {
            return Err(Error::MissingParens);
        }

        let expr_strs: Vec<&str> = rest.split(" ").collect();
        let inner_exps = expr_strs
            .into_iter()
            .map(|s| s.parse::<SExp>())
            .collect::<Result<Vec<SExp>, Self::Err>>()?;
        Ok(SExp::Expr(inner_exps))
    }
}

impl fmt::Display for SExp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SExp::Symbol(c) => write!(f, "({c})"),
            SExp::Expr(exprs) => {
                let expr_str = exprs
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>()
                    .join(" ");
                write!(f, "({expr_str})")
            }
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
    fn parse_list() {}
}
