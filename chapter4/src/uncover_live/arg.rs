use super::UncoverLive;
use chapter2::{x86_var::Arg, Var};

impl UncoverLive for Arg {
    type Target = Option<Var>;
    fn uncover(&self) -> Self::Target {
        match self {
            Arg::Var(v) => Some(v.to_owned()),
            _ => None,
        }
    }
}

#[cfg(test)]
mod args {
    use super::{Arg, UncoverLive};
    #[test]
    fn uncover_var() {
        let result = Arg::Immediate(1).uncover();
        let expected = None;
        assert_eq!(result, expected)
    }

    #[test]
    fn uncover_novar() {
        let result = Arg::Var("a".to_owned()).uncover();
        let expected = Some("a".to_owned());
        assert_eq!(result, expected)
    }
}
