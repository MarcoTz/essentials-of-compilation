use super::{Atm, Exp};
use crate::Var;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stmt {
    Print(Atm),
    Exp(Exp),
    Assign { name: Var, exp: Exp },
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Stmt::Print(atm) => write!(f, "print({})", atm),
            Stmt::Exp(e) => e.fmt(f),
            Stmt::Assign { name, exp } => write!(f, "{name} = {exp}"),
        }
    }
}

impl Stmt {
    pub fn occurs(&self, var: &Var) -> bool {
        match self {
            Stmt::Print(atm) => atm.occurs(var),
            Stmt::Exp(e) => e.occurs(var),
            Stmt::Assign { name, exp } => name == var || exp.occurs(var),
        }
    }
}

#[cfg(test)]
mod stmt_tests {
    use super::{Exp, Stmt};

    #[test]
    fn display_print() {
        let result = format!("{}", Stmt::Print("x".to_owned().into()));
        let expected = "print(x)";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_exp() {
        let result = format!("{}", Stmt::Exp(Exp::Atm(1.into())));
        let expected = "1";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_assign() {
        let result = format!(
            "{}",
            Stmt::Assign {
                name: "x".to_owned(),
                exp: Exp::Atm(1.into())
            }
        );
        let expected = "x = 1";
        assert_eq!(result, expected)
    }

    #[test]
    fn occurs_print() {
        let result = Stmt::Print(1.into()).occurs(&"x".to_owned());
        assert!(!result)
    }

    #[test]
    fn occurs_exp() {
        let result = Stmt::Exp(Exp::Atm("x".to_owned().into())).occurs(&"x".to_owned());
        assert!(result)
    }

    #[test]
    fn occurs_assign() {
        let result = Stmt::Assign {
            name: "x".to_owned(),
            exp: Exp::Atm(1.into()),
        }
        .occurs(&"x".to_owned());
        assert!(result)
    }
}
