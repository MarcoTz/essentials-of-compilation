use super::{Atm, BinOp, UnaryOp, Var};
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum Exp {
    Atm(Atm),
    Assign {
        name: Var,
        bound_term: Atm,
        in_term: Atm,
    },
    InputInt,
    UnaryOp {
        op: UnaryOp,
        exp: Atm,
    },
    BinOp {
        exp1: Atm,
        op: BinOp,
        exp2: Atm,
    },
}

impl fmt::Display for Exp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Exp::Atm(atm) => atm.fmt(f),
            Exp::Assign {
                name,
                bound_term,
                in_term,
            } => write!(f, "(let [{name} {bound_term}] {in_term})"),
            Exp::InputInt => f.write_str("read"),
            Exp::UnaryOp { op, exp } => write!(f, "({op} {exp})"),
            Exp::BinOp { exp1, op, exp2 } => write!(f, "({op} {exp1} {exp2})"),
        }
    }
}

#[cfg(test)]
mod exp_tests {
    use super::{BinOp, Exp, UnaryOp};

    #[test]
    fn display_input() {
        let result = format!("{}", Exp::InputInt);
        let expected = "read";
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
        let expected = "(- 1)";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_bin() {
        let result = format!(
            "{}",
            Exp::BinOp {
                op: BinOp::Add,
                exp1: 6.into(),
                exp2: 2.into()
            }
        );
        let expected = "(+ 6 2)";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_let() {
        let result = format!(
            "{}",
            Exp::Assign {
                name: "x".to_owned(),
                bound_term: 2.into(),
                in_term: "x".to_owned().into()
            }
        );
        let expected = "(let [x 2] x)";
        assert_eq!(result, expected)
    }
}
