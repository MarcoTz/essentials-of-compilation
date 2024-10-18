use super::{
    l_var::syntax::{Exp, Module, Stmt},
    BinOp,
};

pub fn example_empty() -> Module {
    Module { stmts: vec![] }
}

pub fn example_add() -> Module {
    Module {
        stmts: vec![
            Stmt::Assign {
                name: "x".to_owned(),
                exp: Exp::Constant(1.into()),
            },
            Stmt::Assign {
                name: "y".to_owned(),
                exp: Exp::Constant(2.into()),
            },
            Exp::BinOp {
                exp1: Box::new("x".to_owned().into()),
                op: BinOp::Add,
                exp2: Box::new("y".to_owned().into()),
            }
            .into(),
        ],
    }
}

pub fn example_print() -> Module {
    Module {
        stmts: vec![
            Stmt::Assign {
                name: "x".to_owned(),
                exp: Exp::Constant(5.into()),
            },
            Stmt::Print("x".to_owned().into()),
        ],
    }
}

#[cfg(test)]
mod example_tests {
    use super::{example_add, example_empty, example_print};
    use crate::{
        compile::compile,
        x86_int::{Arg, Instr, Prog, Reg},
    };
    use std::collections::HashMap;

    #[test]
    fn compile_empty() {
        let result = compile(example_empty());
        let expected = Prog {
            instrs: vec![],
            stack_space: 0,
            labels: HashMap::from([("main".to_owned(), 0)]),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn compile_add() {
        let result = compile(example_add());
        let expected = Prog {
            instrs: vec![
                Instr::MovQ(Arg::Immediate(1), Arg::Reg(Reg::Rax)),
                Instr::MovQ(Arg::Reg(Reg::Rax), Arg::Deref(Reg::Rbp, -8)),
                Instr::MovQ(Arg::Immediate(2), Arg::Reg(Reg::Rax)),
                Instr::MovQ(Arg::Reg(Reg::Rax), Arg::Deref(Reg::Rbp, -16)),
                Instr::MovQ(Arg::Deref(Reg::Rbp, -8), Arg::Reg(Reg::Rax)),
                Instr::MovQ(Arg::Reg(Reg::Rax), Arg::Reg(Reg::Rbx)),
                Instr::MovQ(Arg::Deref(Reg::Rbp, -16), Arg::Reg(Reg::Rax)),
                Instr::AddQ(Arg::Reg(Reg::Rbx), Arg::Reg(Reg::Rax)),
            ],
            stack_space: 16,
            labels: HashMap::from([("main".to_owned(), 0)]),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn compile_print() {
        let result = compile(example_print());
        let expected = Prog {
            instrs: vec![
                Instr::MovQ(Arg::Immediate(5), Arg::Reg(Reg::Rax)),
                Instr::MovQ(Arg::Reg(Reg::Rax), Arg::Deref(Reg::Rbp, -8)),
                Instr::MovQ(Arg::Deref(Reg::Rbp, -8), Arg::Reg(Reg::Rax)),
                Instr::MovQ(Arg::Reg(Reg::Rax), Arg::Deref(Reg::Rbp, -16)),
                Instr::MovQ(Arg::Deref(Reg::Rbp, -16), Arg::Reg(Reg::Rax)),
                Instr::MovQ(Arg::Reg(Reg::Rax), Arg::Reg(Reg::Rdi)),
                Instr::CallQ("print_int".to_owned(), 1),
            ],
            stack_space: 16,
            labels: HashMap::from([("main".to_owned(), 0)]),
        };
        assert_eq!(result, expected)
    }
}
