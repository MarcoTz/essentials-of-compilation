use super::SelectInstructions;
use crate::{
    l_var_reduced::Exp,
    x86_var::{Arg, Instr, Reg},
    BinOp, UnaryOp,
};

impl SelectInstructions for Exp {
    type Target = Vec<Instr<Arg>>;
    fn select_instructions(self) -> Self::Target {
        match self {
            Exp::Atm(at) => vec![at.select_instructions()],
            Exp::InputInt => vec![Instr::CallQ("read_int".to_owned(), 0)],
            Exp::UnaryOp { op, exp } => {
                let exp_instr = exp.select_instructions();
                let mut new_instrs = vec![exp_instr];
                match op {
                    UnaryOp::Neg => {
                        new_instrs.push(Instr::NegQ(Arg::Reg(Reg::Rax)));
                        new_instrs
                    }
                }
            }
            Exp::BinOp { exp1, op, exp2 } => {
                let mut prog = vec![];
                let exp1_instr = exp1.select_instructions();
                prog.push(exp1_instr);
                prog.push(Instr::MovQ(Arg::Reg(Reg::Rax), Arg::Reg(Reg::Rbx)));
                let exp2_instr = exp2.select_instructions();
                prog.push(exp2_instr);
                match op {
                    BinOp::Add => prog.push(Instr::AddQ(Arg::Reg(Reg::Rbx), Arg::Reg(Reg::Rax))),
                    BinOp::Sub => prog.push(Instr::SubQ(Arg::Reg(Reg::Rbx), Arg::Reg(Reg::Rax))),
                };
                prog
            }
        }
    }
}

#[cfg(test)]
mod exp_tests {
    use super::{Arg, BinOp, Exp, Instr, Reg, SelectInstructions, UnaryOp};
    #[test]
    fn select_atom() {
        let result = Exp::Atm(1.into()).select_instructions();
        let expected = vec![Instr::MovQ(Arg::Intermediate(1), Arg::Reg(Reg::Rax))];
        assert_eq!(result, expected)
    }

    #[test]
    fn select_input() {
        let result = Exp::InputInt.select_instructions();
        let expected = vec![Instr::CallQ("read_int".to_owned(), 0)];
        assert_eq!(result, expected)
    }

    #[test]
    fn select_unary() {
        let result = Exp::UnaryOp {
            exp: 2.into(),
            op: UnaryOp::Neg,
        }
        .select_instructions();
        let expected = vec![
            Instr::MovQ(Arg::Intermediate(2), Arg::Reg(Reg::Rax)),
            Instr::NegQ(Arg::Reg(Reg::Rax)),
        ];
        assert_eq!(result, expected)
    }

    #[test]
    fn select_bin() {
        let result = Exp::BinOp {
            exp1: 1.into(),
            op: BinOp::Add,
            exp2: 2.into(),
        }
        .select_instructions();
        let expected = vec![
            Instr::MovQ(Arg::Intermediate(1), Arg::Reg(Reg::Rax)),
            Instr::MovQ(Arg::Reg(Reg::Rax), Arg::Reg(Reg::Rbx)),
            Instr::MovQ(Arg::Intermediate(2), Arg::Reg(Reg::Rax)),
            Instr::AddQ(Arg::Reg(Reg::Rbx), Arg::Reg(Reg::Rax)),
        ];
        assert_eq!(result, expected)
    }
}
