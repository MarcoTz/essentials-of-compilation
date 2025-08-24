use syntax::x86::{Arg, Instruction, Program, Reg};

pub fn generate_prelude_conclusion(prog: Program) -> Program {
    let prelude = generate_prelude(&prog);
    let conclusion = generate_conclusion(&prog);
    let mut finalized = Program::new(prog.stack_space, prog.used_callee);
    finalized.add_block(&"main", prelude);
    finalized.add_block(&"conclusion", conclusion);
    for (label, block) in prog.blocks {
        let mut generated_block = block;
        if label == "start" {
            generated_block.push(Instruction::Jump {
                label: "conclusion".to_owned(),
            })
        }

        finalized.add_block(&label, generated_block);
    }
    finalized
}

fn used_space(prog: &Program) -> i64 {
    let mut used_space = prog.stack_space + 8 * prog.used_callee.len() as u64;
    used_space = used_space + (used_space % 16);
    used_space -= 8 * prog.used_callee.len() as u64;
    used_space as i64
}

fn generate_prelude(prog: &Program) -> Vec<Instruction<Arg>> {
    let mut prelude = vec![
        Instruction::PushQ {
            arg: Reg::Rbp.into(),
        },
        Instruction::MovQ {
            src: Reg::Rsp.into(),
            dest: Reg::Rbp.into(),
        },
    ];
    for callee_saved in prog.used_callee.iter() {
        prelude.push(Instruction::PushQ {
            arg: callee_saved.clone().into(),
        });
    }
    let used_space = used_space(&prog);
    prelude.push(Instruction::SubQ {
        src: Arg::Immediate(used_space as i64),
        dest: Reg::Rbp.into(),
    });
    prelude.push(Instruction::Jump {
        label: "start".to_owned(),
    });
    prelude
}

fn generate_conclusion(prog: &Program) -> Vec<Instruction<Arg>> {
    let used_space = used_space(prog);
    let mut conc = vec![Instruction::AddQ {
        src: Arg::Immediate(used_space),
        dest: Reg::Rbp.into(),
    }];

    for callee_saved in prog.used_callee.iter() {
        conc.push(Instruction::PopQ {
            arg: callee_saved.clone().into(),
        });
    }
    conc.push(Instruction::MovQ {
        src: Arg::Immediate(0),
        dest: Reg::Rax.into(),
    });
    conc.push(Instruction::RetQ);
    conc
}
