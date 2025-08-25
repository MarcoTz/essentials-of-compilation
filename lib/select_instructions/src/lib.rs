use syntax::{BinaryOperation, PRINT_CALL, READ_INT_CALL, UnaryOperation, lang_c, x86};

pub fn select_instructions(prog: lang_c::Program) -> x86::VarProgram {
    let mut x86_prog = x86::VarProgram::new();
    for (label, tail) in prog.blocks {
        x86_prog.add_block(&label, select_tail(tail));
    }
    x86_prog
}

fn select_tail(tail: lang_c::Tail) -> Vec<x86::Instruction<x86::VarArg>> {
    let mut instrs = vec![];
    for stmt in tail.stmts {
        instrs.extend(select_stmt(stmt));
    }
    instrs.extend(select_return(tail.ret));
    instrs
}

fn select_stmt(stmt: lang_c::Statement) -> Vec<x86::Instruction<x86::VarArg>> {
    match stmt {
        lang_c::Statement::Assign { var, bound } => select_exp(bound, x86::VarArg::Var(var)),
        lang_c::Statement::Print(exp) => {
            let mut exp_instrs = select_exp(exp, x86::Reg::Rdi.into());
            exp_instrs.push(x86::Instruction::CallQ {
                label: PRINT_CALL.to_owned(),
            });
            exp_instrs
        }
    }
}

fn select_return(exp: lang_c::Expression) -> Vec<x86::Instruction<x86::VarArg>> {
    select_exp(exp, x86::Reg::Rax.into())
}

fn select_exp(exp: lang_c::Expression, dest: x86::VarArg) -> Vec<x86::Instruction<x86::VarArg>> {
    match exp {
        lang_c::Expression::Atm(atm) => vec![x86::Instruction::MovQ {
            src: select_atm(atm),
            dest,
        }],
        lang_c::Expression::ReadInt => vec![
            x86::Instruction::CallQ {
                label: READ_INT_CALL.to_owned(),
            },
            x86::Instruction::MovQ {
                src: x86::Reg::Rax.into(),
                dest,
            },
        ],
        lang_c::Expression::UnaryOp { arg, op } => {
            let arg_loc = select_atm(arg);
            match op {
                UnaryOperation::Neg => vec![
                    x86::Instruction::MovQ {
                        src: arg_loc,
                        dest: dest.clone(),
                    },
                    x86::Instruction::NegQ { arg: dest },
                ],
            }
        }
        lang_c::Expression::BinOp { fst, op, snd } => {
            let fst_loc = select_atm(fst);
            let snd_loc = select_atm(snd);
            match op {
                BinaryOperation::Add => vec![
                    x86::Instruction::MovQ {
                        src: fst_loc,
                        dest: dest.clone(),
                    },
                    x86::Instruction::AddQ { src: snd_loc, dest },
                ],
                BinaryOperation::Sub => vec![
                    x86::Instruction::MovQ {
                        src: fst_loc,
                        dest: dest.clone(),
                    },
                    x86::Instruction::SubQ { src: snd_loc, dest },
                ],
            }
        }
        lang_c::Expression::Unit => vec![],
    }
}

fn select_atm(atm: lang_c::Atom) -> x86::VarArg {
    match atm {
        lang_c::Atom::Integer(i) => x86::Arg::Immediate(i).into(),
        lang_c::Atom::Variable(v) => x86::VarArg::Var(v),
    }
}

#[cfg(test)]
mod select_instructions_tests {
    use super::select_instructions;
    use syntax::{BinaryOperation, UnaryOperation, lang_c, x86};

    #[test]
    fn select_sum() {
        let mut prog = lang_c::Program::new();
        prog.add_block(
            "start",
            lang_c::Tail {
                stmts: vec![],
                ret: lang_c::Expression::BinOp {
                    fst: lang_c::Atom::Integer(10),
                    op: BinaryOperation::Add,
                    snd: lang_c::Atom::Integer(32),
                },
            },
        );
        let result = select_instructions(prog);
        let mut expected = x86::VarProgram::new();
        expected.add_block(
            "start",
            vec![
                x86::Instruction::MovQ {
                    src: x86::Arg::Immediate(10).into(),
                    dest: x86::Reg::Rax.into(),
                },
                x86::Instruction::AddQ {
                    src: x86::Arg::Immediate(32).into(),
                    dest: x86::Reg::Rax.into(),
                },
            ],
        );
        assert_eq!(result, expected)
    }

    #[test]
    fn select_neg_sum() {
        let mut prog = lang_c::Program::new();
        prog.add_block(
            "start",
            lang_c::Tail {
                stmts: vec![lang_c::Statement::Assign {
                    var: "x0".to_owned(),
                    bound: lang_c::Expression::UnaryOp {
                        arg: lang_c::Atom::Integer(10),
                        op: UnaryOperation::Neg,
                    },
                }],
                ret: lang_c::Expression::BinOp {
                    fst: lang_c::Atom::Integer(52),
                    op: BinaryOperation::Add,
                    snd: lang_c::Atom::Variable("x0".to_owned()),
                },
            },
        );
        let result = select_instructions(prog);
        let mut expected = x86::VarProgram::new();
        expected.add_block(
            "start",
            vec![
                x86::Instruction::MovQ {
                    src: x86::Arg::Immediate(10).into(),
                    dest: x86::VarArg::Var("x0".to_owned()).into(),
                },
                x86::Instruction::NegQ {
                    arg: x86::VarArg::Var("x0".to_owned()).into(),
                },
                x86::Instruction::MovQ {
                    src: x86::Arg::Immediate(52).into(),
                    dest: x86::Reg::Rax.into(),
                },
                x86::Instruction::AddQ {
                    src: x86::VarArg::Var("x0".to_owned()).into(),
                    dest: x86::Reg::Rax.into(),
                },
            ],
        );
        assert_eq!(result, expected)
    }
}
