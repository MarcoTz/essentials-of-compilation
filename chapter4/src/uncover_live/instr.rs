use super::UncoverLive;
use crate::{
    x86_var::{Arg, Instr},
    Var,
};
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Default)]
pub struct InstrVars {
    pub read: HashSet<Var>,
    pub written: HashSet<Var>,
}

impl From<InstrVars> for HashSet<Var> {
    fn from(vars: InstrVars) -> HashSet<Var> {
        vars.read.union(&vars.written).cloned().collect()
    }
}

impl InstrVars {
    fn from_arg_write(arg: &Arg) -> InstrVars {
        match arg.uncover() {
            None => InstrVars::default(),
            Some(v) => InstrVars {
                read: HashSet::new(),
                written: HashSet::from([v]),
            },
        }
    }

    fn from_read(args: Vec<&Arg>) -> InstrVars {
        InstrVars {
            read: args.iter().filter_map(|arg| arg.uncover()).collect(),
            written: HashSet::new(),
        }
    }
}

impl UncoverLive for Instr<Arg> {
    type Target = InstrVars;
    fn uncover(&self) -> Self::Target {
        match self {
            Instr::AddQ(a1, a2) => InstrVars::from_read(vec![a1, a2]),
            Instr::SubQ(a1, a2) => InstrVars::from_read(vec![a1, a2]),
            Instr::NegQ(a1) => InstrVars::from_read(vec![a1]),
            Instr::MovQ(a1, a2) => {
                let mut vars = InstrVars::from_read(vec![a1]);
                match a2.uncover() {
                    None => (),
                    Some(v) => {
                        vars.written.insert(v);
                    }
                };
                vars
            }
            Instr::CallQ(_, _) => InstrVars::default(),
            Instr::PushQ(arg) => InstrVars::from_read(vec![arg]),
            Instr::PopQ(arg) => InstrVars::from_arg_write(arg),
            Instr::RetQ => InstrVars::default(),
            Instr::Jump(_) => InstrVars::default(),
        }
    }
}

#[cfg(test)]
mod instrs {
    use super::{Arg, Instr, InstrVars, UncoverLive};
    use std::collections::HashSet;

    #[test]
    fn uncover_arith() {
        let result = Instr::AddQ(Arg::Var("a".to_owned()), Arg::Var("b".to_owned())).uncover();
        let expected = InstrVars {
            read: HashSet::from(["a".to_owned(), "b".to_owned()]),
            written: HashSet::new(),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn uncover_move() {
        let result = Instr::MovQ(Arg::Var("a".to_owned()), Arg::Var("b".to_owned())).uncover();
        let expected = InstrVars {
            read: HashSet::from(["a".to_owned()]),
            written: HashSet::from(["b".to_owned()]),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn uncover_push() {
        let result = Instr::PushQ(Arg::Var("a".to_owned())).uncover();
        let expected = InstrVars {
            read: HashSet::from(["a".to_owned()]),
            written: HashSet::new(),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn uncover_pop() {
        let result = Instr::PopQ(Arg::Var("a".to_owned())).uncover();
        let expected = InstrVars {
            read: HashSet::new(),
            written: HashSet::from(["a".to_owned()]),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn uncover_other() {
        let result = Instr::Jump("exit".to_owned()).uncover();
        let expected = InstrVars::default();
        assert_eq!(result, expected)
    }
}
