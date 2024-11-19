use super::Var;
use std::fmt;

#[derive(Debug)]
pub enum Atm {
    Int(i64),
    Var(Var),
}

impl fmt::Display for Atm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Atm::Int(i) => write!(f, "{i}"),
            Atm::Var(v) => write!(f, "{v}"),
        }
    }
}

impl From<i64> for Atm {
    fn from(i: i64) -> Atm {
        Atm::Int(i)
    }
}

impl From<Var> for Atm {
    fn from(var: Var) -> Atm {
        Atm::Var(var)
    }
}

#[cfg(test)]
mod atm_tests {
    use super::Atm;

    #[test]
    fn display_int() {
        let result = format!("{}", Atm::Int(1));
        let expected = "1";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_var() {
        let result = format!("{}", Atm::Var("x".to_owned()));
        let expected = "x";
        assert_eq!(result, expected)
    }
}
