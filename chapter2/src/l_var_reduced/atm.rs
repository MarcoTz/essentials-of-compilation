use super::{Exp, Stmt};
use crate::Var;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Atm {
    Constant(i64),
    Name(Var),
}

impl fmt::Display for Atm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Atm::Constant(i) => i.fmt(f),
            Atm::Name(var) => var.fmt(f),
        }
    }
}

impl From<i64> for Atm {
    fn from(i: i64) -> Atm {
        Atm::Constant(i)
    }
}

impl From<Var> for Atm {
    fn from(st: Var) -> Atm {
        Atm::Name(st)
    }
}
impl From<Atm> for Exp {
    fn from(at: Atm) -> Exp {
        Exp::Atm(at)
    }
}
impl From<Atm> for Stmt {
    fn from(at: Atm) -> Stmt {
        Stmt::Exp(at.into())
    }
}

impl Atm {
    pub fn occurs(&self, var: &Var) -> bool {
        match self {
            Atm::Constant(_) => false,
            Atm::Name(v) => var == v,
        }
    }
}

#[cfg(test)]
mod atm_tests {
    use super::Atm;

    #[test]
    fn display_const() {
        let result = format!("{}", Atm::Constant(1));
        let expected = "1";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_name() {
        let result = format!("{}", Atm::Name("x".to_owned()));
        let expected = "x";
        assert_eq!(result, expected)
    }

    #[test]
    fn occurs_const() {
        assert!(!Atm::Constant(1).occurs(&"x".to_owned()))
    }

    #[test]
    fn occurs_true() {
        assert!(Atm::Name("x".to_owned()).occurs(&"x".to_owned()))
    }

    #[test]
    fn occurs_false() {
        assert!(!Atm::Name("y".to_owned()).occurs(&"x".to_owned()))
    }
}
