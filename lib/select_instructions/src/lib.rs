use syntax::{BinaryOperation, UnaryOperation, lang_c, x86};

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
        lang_c::Expression::InputInt => vec![
            x86::Instruction::CallQ {
                label: "input_int".to_owned(),
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
                    x86::Instruction::NegQ {
                        arg: arg_loc.clone(),
                    },
                    x86::Instruction::MovQ { src: arg_loc, dest },
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
    }
}

fn select_atm(atm: lang_c::Atom) -> x86::VarArg {
    match atm {
        lang_c::Atom::Integer(i) => x86::Arg::Immediate(i).into(),
        lang_c::Atom::Variable(v) => x86::VarArg::Var(v),
    }
}
