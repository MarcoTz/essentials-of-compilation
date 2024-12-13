use super::{Arg, Reg};
use crate::x86_var::Arg as VarArg;
use std::fmt;
pub type Label = String;

#[derive(Clone, Hash, Debug, PartialEq, Eq)]
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

impl<T: Clone> Instr<T> {
    pub fn get_args(&self) -> Vec<T> {
        match self {
            Instr::AddQ(a, b) => vec![a.clone(), b.clone()],
            Instr::SubQ(a, b) => vec![a.clone(), b.clone()],
            Instr::NegQ(a) => vec![a.clone()],
            Instr::MovQ(a, b) => vec![a.clone(), b.clone()],
            Instr::CallQ(_, _) => vec![],
            Instr::PushQ(a) => vec![a.clone()],
            Instr::PopQ(a) => vec![a.clone()],
            Instr::RetQ => vec![],
            Instr::Jump(_) => vec![],
        }
    }

    pub fn set_arg(self, new_arg: T, index: usize) -> Option<Self> {
        if index == 0 {
            self.set_arg1(new_arg)
        } else if index == 1 {
            self.set_arg2(new_arg)
        } else {
            None
        }
    }

    pub fn set_arg1(self, new_arg: T) -> Option<Self> {
        match self {
            Instr::AddQ(_, a) => Some(Instr::AddQ(new_arg, a)),
            Instr::SubQ(_, a) => Some(Instr::SubQ(new_arg, a)),
            Instr::NegQ(_) => Some(Instr::NegQ(new_arg)),
            Instr::MovQ(_, a) => Some(Instr::MovQ(new_arg, a)),
            Instr::CallQ(_, _) => None,
            Instr::PushQ(_) => Some(Instr::PushQ(new_arg)),
            Instr::PopQ(_) => Some(Instr::PopQ(new_arg)),
            Instr::RetQ => None,
            Instr::Jump(_) => None,
        }
    }

    pub fn set_arg2(self, new_arg: T) -> Option<Self> {
        match self {
            Instr::AddQ(a, _) => Some(Instr::AddQ(a, new_arg)),
            Instr::SubQ(a, _) => Some(Instr::SubQ(a, new_arg)),
            Instr::NegQ(_) => None,
            Instr::MovQ(a, _) => Some(Instr::MovQ(a, new_arg)),
            Instr::CallQ(_, _) => None,
            Instr::PushQ(_) => None,
            Instr::PopQ(_) => None,
            Instr::RetQ => None,
            Instr::Jump(_) => None,
        }
    }
}

impl Instr<Arg> {
    pub fn double_deref(&self) -> bool {
        matches!(
            self,
            Instr::AddQ(Arg::Deref(_, _), Arg::Deref(_, _))
                | Instr::SubQ(Arg::Deref(_, _), Arg::Deref(_, _))
                | Instr::MovQ(Arg::Deref(_, _), Arg::Deref(_, _))
        )
    }

    pub fn remove_double_deref(self) -> Vec<Instr<Arg>> {
        if self.double_deref() {
            vec![
                Instr::MovQ(self.get_args().remove(0), Arg::Reg(Reg::Rax)),
                self.set_arg1(Arg::Reg(Reg::Rax)).unwrap(),
            ]
        } else {
            vec![self]
        }
    }

    pub fn remove_max_immediate(self, max_immediate: i64) -> Vec<Instr<Arg>> {
        let args = self.get_args();
        match (args.first(), args.get(1)) {
            (Some(Arg::Immediate(i)), Some(Arg::Deref(_, _))) => {
                if *i > max_immediate {
                    vec![
                        Instr::MovQ(Arg::Immediate(*i), Arg::Reg(Reg::Rax)),
                        self.set_arg1(Arg::Reg(Reg::Rax)).unwrap(),
                    ]
                } else {
                    vec![self]
                }
            }
            (Some(Arg::Deref(_, _)), Some(Arg::Immediate(i))) => {
                if *i > max_immediate {
                    vec![
                        Instr::MovQ(Arg::Immediate(*i), Arg::Reg(Reg::Rax)),
                        self.set_arg2(Arg::Reg(Reg::Rax)).unwrap(),
                    ]
                } else {
                    vec![self]
                }
            }
            (_, _) => vec![self],
        }
    }
}

impl TryInto<Instr<Arg>> for Instr<VarArg> {
    type Error = String;
    fn try_into(self) -> Result<Instr<Arg>, Self::Error> {
        match self {
            Instr::AddQ(a1, a2) => Ok(Instr::AddQ(a1.try_into()?, a2.try_into()?)),
            Instr::SubQ(a1, a2) => Ok(Instr::SubQ(a1.try_into()?, a2.try_into()?)),
            Instr::NegQ(a) => Ok(Instr::NegQ(a.try_into()?)),
            Instr::MovQ(a1, a2) => Ok(Instr::MovQ(a1.try_into()?, a2.try_into()?)),
            Instr::CallQ(fun, i) => Ok(Instr::CallQ(fun, i)),
            Instr::PushQ(a) => Ok(Instr::PushQ(a.try_into()?)),
            Instr::PopQ(a) => Ok(Instr::PopQ(a.try_into()?)),
            Instr::RetQ => Ok(Instr::RetQ),
            Instr::Jump(lab) => Ok(Instr::Jump(lab)),
        }
    }
}

#[cfg(test)]
mod instr_tests {
    use super::Instr;
    use crate::x86_int::Arg;

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
        let result = format!("{}", Instr::CallQ::<Instr<Arg>>("hello".to_owned(), 4));
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
