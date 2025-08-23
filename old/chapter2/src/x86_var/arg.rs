use super::{Reg, Var};
use std::fmt;

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
pub enum Arg {
    Immediate(i64),
    Reg(Reg),
    Deref(Reg, i64),
    Var(Var),
}

impl Arg {
    pub fn as_var(&self) -> Option<Var> {
        if let Arg::Var(v) = self {
            Some(v.clone())
        } else {
            None
        }
    }
}

impl From<&str> for Arg {
    fn from(s: &str) -> Arg {
        Arg::Var(s.to_owned())
    }
}

impl fmt::Display for Arg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Arg::Immediate(i) => write!(f, "${i}"),
            Arg::Reg(reg) => write!(f, "%{reg}"),
            Arg::Deref(reg, i) => write!(f, "{i}(%{reg})"),
            Arg::Var(var) => write!(f, "_{var}"),
        }
    }
}

#[cfg(test)]
mod arg_tests {
    use super::{Arg, Reg};

    #[test]
    fn display_int() {
        let result = format!("{}", Arg::Immediate(1));
        let expected = "$1";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_reg() {
        let result = format!("{}", Arg::Reg(Reg::Rdx));
        let expected = "%rdx";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_deref() {
        let result = format!("{}", Arg::Deref(Reg::Rax, 1));
        let expected = "1(%rax)";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_var() {
        let result = format!("{}", Arg::Var("x".to_owned()));
        let expected = "_x";
        assert_eq!(result, expected)
    }
}
