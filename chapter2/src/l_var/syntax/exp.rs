use super::{BinOp, UnaryOp, Var};
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum Exp {
    Name(Var),
    Assign {
        name: Var,
        bound_term: Box<Exp>,
        in_term: Box<Exp>,
    },
    Constant(i64),
    InputInt,
    UnaryOp {
        op: UnaryOp,
        exp: Box<Exp>,
    },
    BinOp {
        exp1: Box<Exp>,
        op: BinOp,
        exp2: Box<Exp>,
    },
}

impl From<i64> for Exp {
    fn from(i: i64) -> Exp {
        Exp::Constant(i)
    }
}

impl From<String> for Exp {
    fn from(st: String) -> Exp {
        Exp::Name(st)
    }
}

impl fmt::Display for Exp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Exp::Name(name) => f.write_str(name),
            Exp::Assign {
                name,
                bound_term,
                in_term,
            } => write!(f, "(let [{name} {bound_term}] {in_term})"),
            Exp::Constant(i) => write!(f, "{i}"),
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
    fn display_name() {
        let result = format!("{}", Exp::Name("x".to_owned()));
        let expected = "x";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_const() {
        let result = format!("{}", Exp::Constant(1));
        let expected = "1";
        assert_eq!(result, expected)
    }

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
                exp: Box::new(1.into())
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
                exp1: Box::new(6.into()),
                exp2: Box::new(2.into())
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
                bound_term: Box::new(2.into()),
                in_term: Box::new("x".to_owned().into())
            }
        );
        let expected = "(let [x 2] x)";
        assert_eq!(result, expected)
    }
}
