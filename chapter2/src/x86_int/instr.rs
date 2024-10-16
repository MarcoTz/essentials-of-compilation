use std::fmt;
pub type Label = String;

#[derive(Debug, PartialEq, Eq)]
pub enum Instr<T> {
    AddQ(T, T),
    SubQ(T, T),
    NegQ(T),
    MovQ(T, T),
    CallQ(Label, i64),
    PushQ(T),
    PopQ(T),
    RetQ,
    Jump(Label),
}

impl<T: fmt::Display> fmt::Display for Instr<T> {
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
    use super::Instr;
    use crate::x86_int::Arg;

    #[test]
    fn display_addq() {
        let result = format!(
            "{}",
            Instr::AddQ(Arg::Intermediate(1), Arg::Intermediate(2))
        );
        let expected = "addq $1 $2";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_subq() {
        let result = format!(
            "{}",
            Instr::SubQ(Arg::Intermediate(5), Arg::Intermediate(3))
        );
        let expected = "subq $5 $3";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_negq() {
        let result = format!("{}", Instr::NegQ(Arg::Intermediate(3)));
        let expected = "negq $3";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_movq() {
        let result = format!(
            "{}",
            Instr::MovQ(Arg::Intermediate(4), Arg::Intermediate(3))
        );
        let expected = "movq $4 $3";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_callq() {
        let result = format!("{}", Instr::CallQ::<Instr<Arg>>("hello".to_owned(), 4));
        let expected = "callq hello";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_pushq() {
        let result = format!("{}", Instr::PushQ(Arg::Intermediate(3)));
        let expected = "pushq $3";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_popq() {
        let result = format!("{}", Instr::PopQ(Arg::Intermediate(2)));
        let expected = "popq $2";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_retq() {
        let result = format!("{}", Instr::RetQ::<Instr<Arg>>);
        let expected = "retq";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_jump() {
        let result = format!("{}", Instr::Jump::<Instr<Arg>>("exit".to_owned()));
        let expected = "jump exit";
        assert_eq!(result, expected)
    }
}
