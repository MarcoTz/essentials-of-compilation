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
        lang_c::Statement::Assign { var, bound } => {
            let (mut instrs, arg) = select_exp(bound);
            instrs.push(x86::Instruction::MovQ {
                src: arg,
                dest: x86::VarArg::Var(var),
            });
            instrs
        }
    }
}

fn select_return(exp: lang_c::Expression) -> Vec<x86::Instruction<x86::VarArg>> {
    let (mut stmts, arg) = select_exp(exp);
    stmts.push(x86::Instruction::MovQ {
        src: arg,
        dest: x86::Reg::Rax.into(),
    });
    stmts
}

fn select_exp(exp: lang_c::Expression) -> (Vec<x86::Instruction<x86::VarArg>>, x86::VarArg) {
    match exp {
        lang_c::Expression::Atm(atm) => (vec![], select_atm(atm)),
        lang_c::Expression::InputInt => (
            vec![x86::Instruction::CallQ {
                label: "input_int".to_owned(),
            }],
            x86::Reg::Rax.into(),
        ),
        lang_c::Expression::UnaryOp { arg, op } => {
            let arg_loc = select_atm(arg);
            match op {
                UnaryOperation::Neg => (
                    vec![x86::Instruction::NegQ {
                        arg: arg_loc.clone(),
                    }],
                    arg_loc,
                ),
            }
        }
        lang_c::Expression::BinOp { fst, op, snd } => {
            let fst_loc = select_atm(fst);
            let snd_loc = select_atm(snd);
            match op {
                BinaryOperation::Add => (
                    vec![x86::Instruction::AddQ {
                        src: fst_loc,
                        dest: snd_loc.clone(),
                    }],
                    snd_loc,
                ),
                BinaryOperation::Sub => (
                    vec![x86::Instruction::SubQ {
                        src: fst_loc,
                        dest: snd_loc.clone(),
                    }],
                    snd_loc,
                ),
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
