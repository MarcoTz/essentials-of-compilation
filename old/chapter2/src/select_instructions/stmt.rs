use super::SelectInstructions;
use crate::{c_var, x86_var};

impl SelectInstructions for c_var::Stmt {
    type Target = Vec<x86_var::Instr>;
    fn select_instructions(self) -> Self::Target {
        let c_var::Stmt::Assign { var, exp } = self;
        let var_arg = x86_var::Arg::Var(var);
        match exp {
            c_var::Exp::Atm(at) => vec![x86_var::Instr::MovQ(at.select_instructions(), var_arg)],
            c_var::Exp::Read => {
                vec![
                    x86_var::Instr::CallQ("read_int".to_owned(), 0),
                    x86_var::Instr::MovQ(x86_var::Arg::Reg(x86_var::Reg::Rax), var_arg),
                ]
            }
            c_var::Exp::UnaryOp { exp, op } => {
                let op_instr = match op {
                    c_var::UnaryOp::Neg => x86_var::Instr::NegQ,
                };
                vec![
                    x86_var::Instr::MovQ(exp.select_instructions(), var_arg.clone()),
                    op_instr(var_arg),
                ]
            }
            c_var::Exp::BinOp { exp1, op, exp2 } => {
                let arg1 = exp1.select_instructions();
                let arg2 = exp2.select_instructions();
                let op_instr = match op {
                    c_var::BinOp::Add => x86_var::Instr::AddQ,
                    c_var::BinOp::Sub => x86_var::Instr::SubQ,
                };
                if arg1 == var_arg {
                    vec![op_instr(arg2, var_arg)]
                } else if arg2 == var_arg {
                    vec![op_instr(arg1, var_arg)]
                } else {
                    vec![
                        x86_var::Instr::MovQ(arg1, var_arg.clone()),
                        op_instr(arg2, var_arg),
                    ]
                }
            }
        }
    }
}
