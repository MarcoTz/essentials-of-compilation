use super::{UsedVars, Var};
use std::{collections::HashSet, fmt};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Atm {
    Int(i64),
    Var(Var),
}

impl UsedVars for Atm {
    fn used_vars(&self) -> HashSet<Var> {
        match self {
            Atm::Int(_) => HashSet::new(),
            Atm::Var(v) => HashSet::from([v.clone()]),
        }
    }
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
    fn from(v: Var) -> Atm {
        Atm::Var(v)
    }
}

#[cfg(test)]
mod atm_test {
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
