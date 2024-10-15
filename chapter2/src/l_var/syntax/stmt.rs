use super::Exp;
use crate::Var;
use std::fmt;

pub enum Stmt {
    Assign { name: Var, exp: Exp },
    Print(Exp),
    Exp(Exp),
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Stmt::Assign { name, exp } => write!(f, "{name} = {exp}"),
            Stmt::Print(e) => write!(f, "print({e})"),
            Stmt::Exp(e) => e.fmt(f),
        }
    }
}

#[cfg(test)]
mod stmt_tests {
    use super::Stmt;

    #[test]
    fn display_assign() {
        let result = format!(
            "{}",
            Stmt::Assign {
                name: "x".to_owned(),
                exp: 1.into()
            }
        );
        let expected = "x = 1";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_print() {
        let result = format!("{}", Stmt::Print(2.into()));
        let expected = "print(2)";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_exp() {
        let result = format!("{}", Stmt::Exp(3.into()));
        let expected = "3";
        assert_eq!(result, expected)
    }
}
