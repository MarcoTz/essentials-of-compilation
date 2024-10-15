use super::{BinOp, UnaryOp};
use std::fmt;

pub enum Exp {
    Constant(i32),
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

impl fmt::Display for Exp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Exp::Constant(i) => write!(f, "{}", i),
            Exp::InputInt => f.write_str("input_int"),
            Exp::UnaryOp { op, exp } => write!(f, "{}{}", op, exp),
            Exp::BinOp { exp1, op, exp2 } => write!(f, "{}{}{}", exp1, op, exp2),
        }
    }
}

impl Exp {
    pub fn leaf(&self) -> bool {
        matches!(self, Exp::Constant(_) | Exp::InputInt)
    }
}

#[cfg(test)]
mod exp_tests {
    use super::{BinOp, Exp, UnaryOp};

    fn example_const() -> Exp {
        Exp::Constant(1)
    }

    fn example_input() -> Exp {
        Exp::InputInt
    }

    fn example_unary() -> Exp {
        Exp::UnaryOp {
            op: UnaryOp::Neg,
            exp: Box::new(example_const()),
        }
    }

    fn example_bin() -> Exp {
        Exp::BinOp {
            op: BinOp::Add,
            exp1: Box::new(example_const()),
            exp2: Box::new(example_const()),
        }
    }

    #[test]
    fn display_const() {
        let result = format!("{}", example_const());
        let expected = "1";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_input() {
        let result = format!("{}", example_input());
        let expected = "input_int";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_unary() {
        let result = format!("{}", example_unary());
        let expected = "-1";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_bin() {
        let result = format!("{}", example_bin());
        let expected = "1+1";
        assert_eq!(result, expected)
    }

    #[test]
    fn leaf_constant() {
        assert!(example_const().leaf())
    }

    #[test]
    fn leaf_input() {
        assert!(example_input().leaf())
    }

    #[test]
    fn leaf_unary() {
        assert!(!example_unary().leaf())
    }

    #[test]
    fn leaf_bin() {
        assert!(!example_bin().leaf())
    }
}
