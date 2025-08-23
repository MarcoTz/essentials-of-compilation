use super::Reg;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Arg {
    Immediate(i64),
    Reg(Reg),
    Deref(Reg, i64),
}

impl fmt::Display for Arg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Arg::Immediate(i) => write!(f, "${i}"),
            Arg::Reg(reg) => write!(f, "%{reg}"),
            Arg::Deref(reg, i) => write!(f, "{i}(%{reg})"),
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
}
