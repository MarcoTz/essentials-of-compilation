use super::SelectInstructions;
use crate::{c_var, x86_var};

impl SelectInstructions for c_var::Tail {
    type Target = Vec<x86_var::Instr>;
    fn select_instructions(self) -> Self::Target {
        let conc_instr = x86_var::Instr::Jump("conclusion".to_owned());
        match self {
            c_var::Tail::Return(e) => match e {
                c_var::Exp::Atm(a) => vec![
                    x86_var::Instr::MovQ(
                        a.select_instructions(),
                        x86_var::Arg::Reg(x86_var::Reg::Rax),
                    ),
                    conc_instr,
                ],
                c_var::Exp::Read => {
                    vec![x86_var::Instr::CallQ("read_int".to_owned(), 0), conc_instr]
                }
                c_var::Exp::UnaryOp { op, exp } => {
                    let instr_op = match op {
                        c_var::UnaryOp::Neg => x86_var::Instr::NegQ,
                    };
                    vec![
                        x86_var::Instr::MovQ(
                            exp.select_instructions(),
                            x86_var::Arg::Reg(x86_var::Reg::Rax),
                        ),
                        instr_op(x86_var::Arg::Reg(x86_var::Reg::Rax)),
                        conc_instr,
                    ]
                }
                c_var::Exp::BinOp { exp1, op, exp2 } => {
                    let instr_op = match op {
                        c_var::BinOp::Add => x86_var::Instr::AddQ,
                        c_var::BinOp::Sub => x86_var::Instr::SubQ,
                    };
                    vec![
                        x86_var::Instr::MovQ(
                            exp1.select_instructions(),
                            x86_var::Arg::Reg(x86_var::Reg::Rax),
                        ),
                        instr_op(
                            exp2.select_instructions(),
                            x86_var::Arg::Reg(x86_var::Reg::Rax),
                        ),
                        conc_instr,
                    ]
                }
            },
            c_var::Tail::Seq(stmt, tl) => {
                let mut instrs = stmt.select_instructions();
                instrs.extend(tl.select_instructions());
                instrs
            }
        }
    }
}
