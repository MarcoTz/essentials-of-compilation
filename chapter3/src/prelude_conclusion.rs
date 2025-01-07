use crate::x86_int::{Arg, Instr, Program, Reg};

pub fn generate_prelude_conclusion(prog: Program) -> Program {
    let mut new_blocks = prog.blocks;

    // If on MacOS, each label needs to start with _
    if std::env::consts::OS == "macos" {
        new_blocks = new_blocks
            .into_iter()
            .map(|(label, instrs)| (format!("_{label}"), instrs))
            .collect();
    }

    let main_instrs = vec![
        Instr::PushQ(Arg::Reg(Reg::Rbp)),
        Instr::MovQ(Arg::Reg(Reg::Rsp), Arg::Reg(Reg::Rbp)),
        Instr::SubQ(Arg::Immediate(16), Arg::Reg(Reg::Rsp)),
        Instr::Jump("start".to_owned()),
    ];
    new_blocks.insert("main".to_owned(), main_instrs);

    let conclusion_instrs = vec![
        Instr::AddQ(Arg::Immediate(16), Arg::Reg(Reg::Rsp)),
        Instr::PopQ(Arg::Reg(Reg::Rbp)),
        Instr::RetQ,
    ];
    new_blocks.insert("conclusion".to_owned(), conclusion_instrs);

    // main needs to be a global label
    let mut global_labels = prog.global_labels;
    global_labels.insert("main".to_owned());

    Program {
        blocks: new_blocks,
        stack_space: prog.stack_space,
        global_labels,
        callee_saved: prog.callee_saved,
    }
}
