use super::SelectInstructions;
use crate::{l_var_reduced::Module, x86_var::Prog};

impl SelectInstructions for Module {
    type Target = Prog;
    fn select_instructions(self) -> Self::Target {
        let mut prog = vec![];
        for stmt in self.into_iter() {
            prog.extend(stmt.select_instructions());
        }
        prog
    }
}

#[cfg(test)]
mod prog_tests {
    use super::SelectInstructions;
    use crate::{
        l_var_reduced::{Exp, Stmt},
        x86_var::{Arg, Instr, Reg},
        BinOp,
    };

    #[test]
    fn select_empty() {
        let result = vec![].select_instructions();
        let expected = vec![];
        assert_eq!(result, expected)
    }

    #[test]
    fn select_prog() {
        let result = vec![
            Stmt::Assign {
                name: "x".to_owned(),
                exp: Exp::Atm(4.into()),
            },
            Stmt::Assign {
                name: "y".to_owned(),
                exp: Exp::Atm(2.into()),
            },
            Stmt::Exp(Exp::BinOp {
                exp1: "x".to_owned().into(),
                exp2: "y".to_owned().into(),
                op: BinOp::Add,
            }),
        ]
        .select_instructions();
        let expected = vec![
            Instr::MovQ(Arg::Intermediate(4), Arg::Reg(Reg::Rax)),
            Instr::MovQ(Arg::Reg(Reg::Rax), Arg::Var("x".to_owned())),
            Instr::MovQ(Arg::Intermediate(2), Arg::Reg(Reg::Rax)),
            Instr::MovQ(Arg::Reg(Reg::Rax), Arg::Var("y".to_owned())),
            Instr::MovQ(Arg::Var("x".to_owned()), Arg::Reg(Reg::Rax)),
            Instr::MovQ(Arg::Reg(Reg::Rax), Arg::Reg(Reg::Rbx)),
            Instr::MovQ(Arg::Var("y".to_owned()), Arg::Reg(Reg::Rax)),
            Instr::AddQ(Arg::Reg(Reg::Rbx), Arg::Reg(Reg::Rax)),
        ];
        assert_eq!(result, expected)
    }
}
