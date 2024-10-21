use super::exp::Exp;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum Stmt {
    Print(Exp),
    Exp(Exp),
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Stmt::Print(exp) => write!(f, "print({exp})"),
            Stmt::Exp(e) => e.fmt(f),
        }
    }
}

#[cfg(test)]
mod stmt_tests {
    use super::{Exp, Stmt};

    fn example_print() -> Stmt {
        Stmt::Print(Exp::Constant(1))
    }

    fn example_exp() -> Stmt {
        Stmt::Exp(Exp::Constant(1))
    }

    #[test]
    fn display_print() {
        let result = format!("{}", example_print());
        let expected = "print(1)";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_exp() {
        let result = format!("{}", example_exp());
        let expected = "1";
        assert_eq!(result, expected)
    }
}
