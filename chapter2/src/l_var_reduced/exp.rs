use super::{Atm, Stmt};
use crate::{BinOp, UnaryOp, Var};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Exp {
    Atm(Atm),
    InputInt,
    UnaryOp { op: UnaryOp, exp: Atm },
    BinOp { exp1: Atm, op: BinOp, exp2: Atm },
}

impl fmt::Display for Exp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Exp::Atm(at) => at.fmt(f),
            Exp::InputInt => f.write_str("input_int"),
            Exp::UnaryOp { op, exp } => write!(f, "{op}{exp}"),
            Exp::BinOp { exp1, op, exp2 } => write!(f, "{exp1}{op}{exp2}"),
        }
    }
}

impl From<Exp> for Stmt {
    fn from(exp: Exp) -> Stmt {
        Stmt::Exp(exp)
    }
}

impl Exp {
    pub fn occurs(&self, var: &Var) -> bool {
        match self {
            Exp::Atm(atm) => atm.occurs(var),
            Exp::InputInt => false,
            Exp::UnaryOp { op: _, exp } => exp.occurs(var),
            Exp::BinOp { exp1, op: _, exp2 } => exp1.occurs(var) || exp2.occurs(var),
        }
    }
}

#[cfg(test)]
mod exp_tests {
    use super::{BinOp, Exp, UnaryOp};

    #[test]
    fn display_atm() {
        let result = format!("{}", Exp::Atm(1.into()));
        let expected = "1";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_input() {
        let result = format!("{}", Exp::InputInt);
        let expected = "input_int";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_unary() {
        let result = format!(
            "{}",
            Exp::UnaryOp {
                op: UnaryOp::Neg,
                exp: 1.into()
            }
        );
        let expected = "-1";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_bin() {
        let result = format!(
            "{}",
            Exp::BinOp {
                op: BinOp::Add,
                exp1: 2.into(),
                exp2: 4.into()
            }
        );
        let expected = "2+4";
        assert_eq!(result, expected)
    }

    #[test]
    fn occurs_atom() {
        let result = Exp::Atm("x".to_owned().into()).occurs(&"y".to_owned());
        assert!(!result)
    }

    #[test]
    fn occurs_input() {
        let result = Exp::InputInt.occurs(&"x".to_owned());
        assert!(!result)
    }

    #[test]
    fn occurs_unary() {
        let result = Exp::UnaryOp {
            op: UnaryOp::Neg,
            exp: 1.into(),
        }
        .occurs(&"x".to_owned());
        assert!(!result)
    }

    #[test]
    fn occurs_binary() {
        let result = Exp::BinOp {
            op: BinOp::Add,
            exp1: 1.into(),
            exp2: 2.into(),
        }
        .occurs(&"x".to_owned());
        assert!(!result)
    }
}
