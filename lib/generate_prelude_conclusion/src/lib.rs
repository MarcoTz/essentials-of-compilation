use std::env::consts;
use syntax::x86::{Arg, Instruction, Program, Reg};

pub fn generate_prelude_conclusion(prog: Program) -> Program {
    let prelude = generate_prelude(&prog)
        .into_iter()
        .map(|instr| patch_label_instr(instr))
        .collect();
    let conclusion = generate_conclusion(&prog)
        .into_iter()
        .map(|instr| patch_label_instr(instr))
        .collect();
    let mut finalized = Program::new(prog.stack_space, prog.used_callee);
    finalized.add_block(&patch_label("main"), prelude);
    finalized.add_block(&patch_label("conclusion"), conclusion);
    for (label, block) in prog.blocks {
        let mut generated_block: Vec<Instruction<Arg>> = block
            .into_iter()
            .map(|instr| patch_label_instr(instr))
            .collect();
        if label == "start" {
            generated_block.push(Instruction::Jump {
                label: patch_label("conclusion"),
            })
        }

        finalized.add_block(&patch_label(&label), generated_block);
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
        label: patch_label("start"),
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
    conc.push(Instruction::RetQ);
    conc
}

fn patch_label(label: &str) -> String {
    if consts::OS == "macos" {
        format!("_{label}")
    } else {
        label.to_owned()
    }
}

fn patch_label_instr(instr: Instruction<Arg>) -> Instruction<Arg> {
    match instr {
        Instruction::AddQ { src, dest } => Instruction::AddQ { src, dest },
        Instruction::SubQ { src, dest } => Instruction::SubQ { src, dest },
        Instruction::NegQ { arg } => Instruction::NegQ { arg },
        Instruction::MovQ { src, dest } => Instruction::MovQ { src, dest },
        Instruction::PushQ { arg } => Instruction::PushQ { arg },
        Instruction::PopQ { arg } => Instruction::PopQ { arg },
        Instruction::CallQ { label } => Instruction::CallQ {
            label: patch_label(&label),
        },
        Instruction::RetQ => Instruction::RetQ,
        Instruction::Jump { label } => Instruction::Jump {
            label: patch_label(&label),
        },
    }
}
