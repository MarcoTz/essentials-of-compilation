use syntax::{BinaryOperation, Comparator, PRINT_CALL, READ_INT_CALL, UnaryOperation, lang_c, x86};

pub fn select_instructions(prog: lang_c::Program) -> x86::VarProgram {
    let mut x86_prog = x86::VarProgram::new();
    for block in prog.blocks {
        x86_prog.add_block(&block.label, select_tail(block.tail));
    }
    x86_prog
}

fn select_tail(tail: lang_c::Tail) -> Vec<x86::Instruction<x86::VarArg>> {
    let mut instrs = vec![];
    for stmt in tail.stmts {
        instrs.extend(select_stmt(stmt));
    }
    instrs.extend(select_return(tail.cont));
    instrs
}

fn select_stmt(stmt: lang_c::Statement) -> Vec<x86::Instruction<x86::VarArg>> {
    match stmt {
        lang_c::Statement::Assign { var, bound } => select_exp(bound, x86::VarArg::Var(var)),
        lang_c::Statement::Print(atm) => {
            let arg_loc = select_atm(atm);
            let mov = x86::Instruction::MovQ {
                src: arg_loc,
                dest: x86::Reg::Rdi.into(),
            };
            let print = x86::Instruction::CallQ {
                label: PRINT_CALL.to_owned(),
            };
            vec![mov, print]
        }
    }
}

fn select_return(cont: lang_c::Continuation) -> Vec<x86::Instruction<x86::VarArg>> {
    match cont {
        lang_c::Continuation::Return(atm) => {
            let arg_dest = select_atm(atm);
            vec![
                x86::Instruction::MovQ {
                    src: arg_dest,
                    dest: x86::Reg::Rax.into(),
                },
                x86::Instruction::RetQ,
            ]
        }
        lang_c::Continuation::Goto(label) => vec![x86::Instruction::Jump { label }],
        lang_c::Continuation::If {
            left,
            cmp,
            right,
            then_label,
            else_label,
        } => {
            let left_dest = select_atm(left);
            let right_dest = select_atm(right);
            let cmp_instr = x86::Instruction::CmpQ {
                left: left_dest,
                right: right_dest,
            };
            let cmp = select_cmp(cmp);
            let jump_true = x86::Instruction::JumpCC {
                cc: cmp,
                label: then_label,
            };
            let jump_false = x86::Instruction::Jump { label: else_label };
            vec![cmp_instr, jump_true, jump_false]
        }
    }
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
                UnaryOperation::Not => vec![
                    x86::Instruction::MovQ {
                        src: arg_loc,
                        dest: dest.clone(),
                    },
                    x86::Instruction::NotQ { arg: dest },
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
                BinaryOperation::And => vec![
                    x86::Instruction::MovQ {
                        src: fst_loc,
                        dest: dest.clone(),
                    },
                    x86::Instruction::AndQ { src: snd_loc, dest },
                ],
                BinaryOperation::Or => vec![
                    x86::Instruction::MovQ {
                        src: fst_loc,
                        dest: dest.clone(),
                    },
                    x86::Instruction::OrQ { src: snd_loc, dest },
                ],
            }
        }
        lang_c::Expression::Cmp { left, cmp, right } => {
            let left_dest = select_atm(left);
            let right_dest = select_atm(right);
            let cc = select_cmp(cmp);
            vec![
                x86::Instruction::CmpQ {
                    left: left_dest,
                    right: right_dest,
                },
                x86::Instruction::SetCC { cc, dest },
            ]
        }
    }
}

fn select_atm(atm: lang_c::Atom) -> x86::VarArg {
    match atm {
        lang_c::Atom::Integer(i) => x86::Arg::Immediate(i).into(),
        lang_c::Atom::Variable(v) => x86::VarArg::Var(v),
        lang_c::Atom::Bool(b) => {
            if b {
                x86::Arg::Immediate(1).into()
            } else {
                x86::Arg::Immediate(0).into()
            }
        }
        lang_c::Atom::Unit => x86::Arg::Immediate(0).into(),
    }
}

fn select_cmp(cmp: Comparator) -> x86::Cc {
    match cmp {
        Comparator::Eq => x86::Cc::E,
        Comparator::Lt => x86::Cc::L,
        Comparator::Leq => x86::Cc::Le,
        Comparator::Gt => x86::Cc::G,
        Comparator::Geq => x86::Cc::Ge,
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
                stmts: vec![lang_c::Statement::assign(
                    "x0",
                    lang_c::Expression::BinOp {
                        fst: lang_c::Atom::Integer(10),
                        op: BinaryOperation::Add,
                        snd: lang_c::Atom::Integer(32),
                    },
                )],
                cont: lang_c::Continuation::Return(lang_c::Atom::Variable("x0".to_owned())),
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
                stmts: vec![
                    lang_c::Statement::assign(
                        "x0",
                        lang_c::Expression::UnaryOp {
                            arg: lang_c::Atom::Integer(10),
                            op: UnaryOperation::Neg,
                        },
                    ),
                    lang_c::Statement::assign(
                        "x1",
                        lang_c::Expression::BinOp {
                            fst: lang_c::Atom::Integer(52),
                            op: BinaryOperation::Add,
                            snd: lang_c::Atom::Variable("x0".to_owned()),
                        },
                    ),
                ],
                cont: lang_c::Continuation::Return(lang_c::Atom::Variable("x1".to_owned())),
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
