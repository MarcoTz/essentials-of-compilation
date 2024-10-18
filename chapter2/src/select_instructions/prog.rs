use super::SelectInstructions;
use crate::{l_var_reduced::Module, x86_var::Prog};
use std::collections::HashMap;

impl SelectInstructions for Module {
    type Target = Prog;
    fn select_instructions(self) -> Self::Target {
        let mut instrs = vec![];
        for stmt in self.into_iter() {
            instrs.extend(stmt.select_instructions());
        }
        Prog {
            instrs,
            labels: HashMap::from([("main".to_owned(), 0)]),
        }
    }
}

#[cfg(test)]
mod prog_tests {
    use super::{Prog, SelectInstructions};
    use crate::{
        l_var_reduced::{Exp, Stmt},
        x86_var::{Arg, Instr, Reg},
        BinOp,
    };
    use std::collections::HashMap;

    #[test]
    fn select_empty() {
        let result = vec![].select_instructions();
        let expected = Prog {
            instrs: vec![],
            labels: HashMap::from([("main".to_owned(), 0)]),
        };
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
        let expected = Prog {
            labels: HashMap::from([("main".to_owned(), 0)]),
            instrs: vec![
                Instr::MovQ(Arg::Immediate(4), Arg::Reg(Reg::Rax)),
                Instr::MovQ(Arg::Reg(Reg::Rax), Arg::Var("x".to_owned())),
                Instr::MovQ(Arg::Immediate(2), Arg::Reg(Reg::Rax)),
                Instr::MovQ(Arg::Reg(Reg::Rax), Arg::Var("y".to_owned())),
                Instr::MovQ(Arg::Var("x".to_owned()), Arg::Reg(Reg::Rax)),
                Instr::MovQ(Arg::Reg(Reg::Rax), Arg::Reg(Reg::Rbx)),
                Instr::MovQ(Arg::Var("y".to_owned()), Arg::Reg(Reg::Rax)),
                Instr::AddQ(Arg::Reg(Reg::Rbx), Arg::Reg(Reg::Rax)),
            ],
        };
        assert_eq!(result, expected)
    }
}
