use super::{Arg, Label};
use std::fmt;

#[derive(Clone, Hash, Debug, PartialEq, Eq)]
pub enum Instr {
    AddQ(Arg, Arg),
    SubQ(Arg, Arg),
    NegQ(Arg),
    MovQ(Arg, Arg),
    CallQ(Label, usize),
    PushQ(Arg),
    PopQ(Arg),
    RetQ,
    Jump(Label),
}

impl fmt::Display for Instr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Instr::AddQ(a1, a2) => write!(f, "addq {a1} {a2}"),
            Instr::SubQ(a1, a2) => write!(f, "subq {a1} {a2}"),
            Instr::NegQ(a) => write!(f, "negq {a}"),
            Instr::MovQ(a1, a2) => write!(f, "movq {a1} {a2}"),
            Instr::CallQ(l, _) => write!(f, "callq {l}"),
            Instr::PushQ(a) => write!(f, "pushq {a}"),
            Instr::PopQ(a) => write!(f, "popq {a}"),
            Instr::RetQ => write!(f, "retq"),
            Instr::Jump(l) => write!(f, "jump {l}"),
        }
    }
}

#[cfg(test)]
mod instr_tests {
    use super::{Arg, Instr};

    #[test]
    fn display_addq() {
        let result = format!("{}", Instr::AddQ(Arg::Immediate(1), Arg::Immediate(2)));
        let expected = "addq $1 $2";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_subq() {
        let result = format!("{}", Instr::SubQ(Arg::Immediate(5), Arg::Immediate(3)));
        let expected = "subq $5 $3";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_negq() {
        let result = format!("{}", Instr::NegQ(Arg::Immediate(3)));
        let expected = "negq $3";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_movq() {
        let result = format!("{}", Instr::MovQ(Arg::Immediate(4), Arg::Immediate(3)));
        let expected = "movq $4 $3";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_callq() {
        let result = format!("{}", Instr::CallQ("hello".to_owned(), 4));
        let expected = "callq hello";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_pushq() {
        let result = format!("{}", Instr::PushQ(Arg::Immediate(3)));
        let expected = "pushq $3";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_popq() {
        let result = format!("{}", Instr::PopQ(Arg::Immediate(2)));
        let expected = "popq $2";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_retq() {
        let result = format!("{}", Instr::RetQ);
        let expected = "retq";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_jump() {
        let result = format!("{}", Instr::Jump("exit".to_owned()));
        let expected = "jump exit";
        assert_eq!(result, expected)
    }
}
