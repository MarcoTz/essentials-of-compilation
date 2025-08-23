use super::{Exp, Stmt};
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum Tail {
    Return(Exp),
    Seq(Stmt, Box<Tail>),
}

impl fmt::Display for Tail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Tail::Return(exp) => write!(f, "return {exp};"),
            Tail::Seq(stmt, tl) => write!(f, "{stmt};\n{tl}"),
        }
    }
}

#[cfg(test)]
mod tail_test {
    use super::{Exp, Stmt, Tail};
    #[test]
    fn display_return() {
        let result = format!("{}", Tail::Return(Exp::Read));
        let expected = "return read;";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_seq() {
        let result = format!(
            "{}",
            Tail::Seq(
                Stmt::Assign {
                    var: "x".to_owned(),
                    exp: Exp::Read
                },
                Box::new(Tail::Return(Exp::Read))
            )
        );
        let expected = "x = read;\nreturn read;";
        assert_eq!(result, expected)
    }
}
