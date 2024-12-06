use super::{Arg, Instr, Program, Reg};

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
    }
}

#[cfg(test)]
mod prelude_conclusion_tests {
    use super::{generate_prelude_conclusion, Arg, Instr, Program, Reg};
    use std::collections::{HashMap, HashSet};

    #[test]
    fn gen_prelude_conc() {
        let result = generate_prelude_conclusion(Program {
            blocks: HashMap::from([(
                "start".to_owned(),
                vec![Instr::AddQ(Arg::Immediate(10), Arg::Immediate(5))],
            )]),
            stack_space: 0,
            global_labels: HashSet::new(),
        });
        let labels = if std::env::consts::OS == "macos" {
            ["_main", "_start", "_conclusion"]
        } else {
            ["main", "start", "conclusion"]
        };
        let expected = Program {
            blocks: HashMap::from([
                (
                    labels[0].to_owned(),
                    vec![
                        Instr::PushQ(Arg::Reg(Reg::Rbp)),
                        Instr::MovQ(Arg::Reg(Reg::Rsp), Arg::Reg(Reg::Rbp)),
                        Instr::SubQ(Arg::Immediate(16), Arg::Reg(Reg::Rsp)),
                        Instr::Jump("start".to_owned()),
                    ],
                ),
                (
                    labels[1].to_owned(),
                    vec![Instr::AddQ(Arg::Immediate(10), Arg::Immediate(5))],
                ),
                (
                    labels[2].to_owned(),
                    vec![
                        Instr::AddQ(Arg::Immediate(16), Arg::Reg(Reg::Rsp)),
                        Instr::PopQ(Arg::Reg(Reg::Rbp)),
                        Instr::RetQ,
                    ],
                ),
            ]),
            stack_space: 0,
            global_labels: HashSet::from([labels[0].to_owned()]),
        };
        assert_eq!(result, expected)
    }
}
