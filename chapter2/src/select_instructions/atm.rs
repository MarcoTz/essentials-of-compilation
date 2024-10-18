use super::SelectInstructions;
use crate::{
    l_var_reduced::Atm,
    x86_var::{Arg, Instr, Reg},
};

impl SelectInstructions for Atm {
    type Target = Instr<Arg>;
    fn select_instructions(self) -> Self::Target {
        match self {
            Atm::Constant(i) => Instr::MovQ(Arg::Immediate(i), Arg::Reg(Reg::Rax)),
            Atm::Name(name) => Instr::MovQ(Arg::Var(name), Arg::Reg(Reg::Rax)),
        }
    }
}

#[cfg(test)]
mod atm_tests {
    use super::{Arg, Atm, Instr, Reg, SelectInstructions};

    #[test]
    fn select_constant() {
        let result = Atm::Constant(1).select_instructions();
        let expected = Instr::MovQ(Arg::Immediate(1), Arg::Reg(Reg::Rax));
        assert_eq!(result, expected)
    }

    #[test]
    fn select_name() {
        let result = Atm::Name("x".to_owned()).select_instructions();
        let expected = Instr::MovQ(Arg::Var("x".to_owned()), Arg::Reg(Reg::Rax));
        assert_eq!(result, expected)
    }
}
