use super::{Exp, Var};
use std::fmt;

#[derive(Debug)]
pub enum Stmt {
    Assign { var: Var, exp: Exp },
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Stmt::Assign { var, exp } => write!(f, "{var} = {exp}"),
        }
    }
}

#[cfg(test)]
mod stmt_test {
    use super::{Exp, Stmt};

    #[test]
    fn display_assign() {
        let result = format!(
            "{}",
            Stmt::Assign {
                var: "x".to_owned(),
                exp: Exp::Read
            }
        );
        let expected = "x = read";
        assert_eq!(result, expected)
    }
}
