use super::{Atm, BinOp, Tail, UnaryOp};
use std::fmt;

#[derive(Debug)]
pub enum Exp {
    Atm(Atm),
    Read,
    UnaryOp { op: UnaryOp, exp: Atm },
    BinOp { exp1: Atm, op: BinOp, exp2: Atm },
}

impl From<Exp> for Tail {
    fn from(e: Exp) -> Tail {
        Tail::Return(e)
    }
}

impl fmt::Display for Exp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Exp::Atm(a) => a.fmt(f),
            Exp::Read => f.write_str("read"),
            Exp::UnaryOp { op, exp } => write!(f, "({op} {exp})"),
            Exp::BinOp { exp1, op, exp2 } => write!(f, "({op} {exp1} {exp2})"),
        }
    }
}

#[cfg(test)]
mod exp_test {
    use super::{BinOp, Exp, UnaryOp};

    #[test]
    fn display_atm() {
        let result = format!("{}", Exp::Atm(1.into()));
        let expected = "1";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_read() {
        let result = format!("{}", Exp::Read);
        let expected = "read";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_unary() {
        let result = format!(
            "{}",
            Exp::UnaryOp {
                exp: 1.into(),
                op: UnaryOp::Neg
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
                exp1: 1.into(),
                op: BinOp::Add,
                exp2: 2.into()
            }
        );
        let expected = "(+ 1 2)";
        assert_eq!(result, expected)
    }
}
