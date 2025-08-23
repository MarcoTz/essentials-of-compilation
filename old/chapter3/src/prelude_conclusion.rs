use crate::x86_int::{Arg, Instr, Label, Program, Reg};
use std::collections::{HashMap, HashSet};

pub fn generate_prelude_conclusion(prog: Program) -> Program {
    let mut new_blocks = prog.blocks;

    let mut stack_diff = prog.stack_space - 8 + 8 * prog.callee_saved.len();
    stack_diff += stack_diff % 16;
    stack_diff -= 8 * prog.callee_saved.len();

    let main_instrs = generate_main(prog.callee_saved.clone(), stack_diff as i64);
    new_blocks.insert("main".to_owned(), main_instrs);

    let conclusion_instrs = generate_conclusion(prog.callee_saved.clone(), stack_diff as i64);
    new_blocks.insert("conclusion".to_owned(), conclusion_instrs);

    // main needs to be a global label
    let mut global_labels = prog.global_labels;
    global_labels.insert("main".to_owned());

    fix_macos(&mut new_blocks, &mut global_labels);

    Program {
        blocks: new_blocks,
        stack_space: prog.stack_space,
        global_labels,
        callee_saved: prog.callee_saved,
    }
}

fn generate_main(callee_saved: HashSet<Reg>, stack_diff: i64) -> Vec<Instr> {
    let mut main_instrs = vec![
        Instr::PushQ(Arg::Reg(Reg::Rbp)),
        Instr::MovQ(Arg::Reg(Reg::Rsp), Arg::Reg(Reg::Rbp)),
    ];
    for reg in callee_saved {
        main_instrs.push(Instr::PushQ(Arg::Reg(reg.clone())));
    }
    main_instrs.push(Instr::SubQ(Arg::Immediate(stack_diff), Arg::Reg(Reg::Rsp)));
    main_instrs.push(Instr::Jump("start".to_owned()));
    main_instrs
}

fn generate_conclusion(callee_saved: HashSet<Reg>, stack_diff: i64) -> Vec<Instr> {
    let mut conclusion_instrs = vec![Instr::AddQ(Arg::Immediate(stack_diff), Arg::Reg(Reg::Rsp))];
    for reg in callee_saved {
        conclusion_instrs.push(Instr::PopQ(Arg::Reg(reg.clone())));
    }
    conclusion_instrs.push(Instr::PopQ(Arg::Reg(Reg::Rbp)));
    conclusion_instrs.push(Instr::RetQ);
    conclusion_instrs
}

fn fix_macos(blocks: &mut HashMap<Label, Vec<Instr>>, global_labels: &mut HashSet<Label>) {
    // If on MacOS, each label needs to start with _
    if std::env::consts::OS != "macos" {
        return;
    }
    let keys: Vec<String> = blocks.keys().cloned().collect();
    for key in keys {
        let instrs = blocks.remove(&key).unwrap();
        blocks.insert(format!("_{key}"), instrs);
    }

    let labels: Vec<Label> = global_labels.iter().cloned().collect();
    for label in labels {
        global_labels.remove(&label);
        global_labels.insert(format!("_{label}"));
    }
}

#[cfg(test)]
mod prelude_conclusion_tests {
    use super::{generate_prelude_conclusion, Program};
    use crate::x86_int::{Arg, Instr, Reg};
    use std::collections::{HashMap, HashSet};

    #[test]
    fn gen_prelude_conc() {
        let result = generate_prelude_conclusion(Program {
            blocks: HashMap::from([(
                "start".to_owned(),
                vec![
                    Instr::MovQ(Arg::Immediate(1), Arg::Reg(Reg::Rbx)),
                    Instr::MovQ(Arg::Immediate(42), Arg::Reg(Reg::Rcx)),
                    Instr::AddQ(Arg::Immediate(7), Arg::Reg(Reg::Rbx)),
                    Instr::MovQ(Arg::Deref(Reg::Rbp, -8), Arg::Reg(Reg::Rax)),
                    Instr::MovQ(Arg::Reg(Reg::Rbx), Arg::Deref(Reg::Rbp, -16)),
                    Instr::AddQ(Arg::Reg(Reg::Rcx), Arg::Reg(Reg::Rbx)),
                    Instr::MovQ(Arg::Deref(Reg::Rbp, -16), Arg::Reg(Reg::Rcx)),
                    Instr::NegQ(Arg::Reg(Reg::Rcx)),
                    Instr::MovQ(Arg::Reg(Reg::Rbx), Arg::Reg(Reg::Rax)),
                    Instr::AddQ(Arg::Reg(Reg::Rcx), Arg::Reg(Reg::Rax)),
                    Instr::Jump("conclusion".to_owned()),
                ],
            )]),
            stack_space: 16,
            global_labels: HashSet::new(),
            callee_saved: HashSet::from([Reg::Rbx]),
        });
        let expected = Program {
            blocks: HashMap::from([
                (
                    "start".to_owned(),
                    vec![
                        Instr::MovQ(Arg::Immediate(1), Arg::Reg(Reg::Rbx)),
                        Instr::MovQ(Arg::Immediate(42), Arg::Reg(Reg::Rcx)),
                        Instr::AddQ(Arg::Immediate(7), Arg::Reg(Reg::Rbx)),
                        Instr::MovQ(Arg::Deref(Reg::Rbp, -8), Arg::Reg(Reg::Rax)),
                        Instr::MovQ(Arg::Reg(Reg::Rbx), Arg::Deref(Reg::Rbp, -16)),
                        Instr::AddQ(Arg::Reg(Reg::Rcx), Arg::Reg(Reg::Rbx)),
                        Instr::MovQ(Arg::Deref(Reg::Rbp, -16), Arg::Reg(Reg::Rcx)),
                        Instr::NegQ(Arg::Reg(Reg::Rcx)),
                        Instr::MovQ(Arg::Reg(Reg::Rbx), Arg::Reg(Reg::Rax)),
                        Instr::AddQ(Arg::Reg(Reg::Rcx), Arg::Reg(Reg::Rax)),
                        Instr::Jump("conclusion".to_owned()),
                    ],
                ),
                (
                    "main".to_owned(),
                    vec![
                        Instr::PushQ(Arg::Reg(Reg::Rbp)),
                        Instr::MovQ(Arg::Reg(Reg::Rsp), Arg::Reg(Reg::Rbp)),
                        Instr::PushQ(Arg::Reg(Reg::Rbx)),
                        Instr::SubQ(Arg::Immediate(8), Arg::Reg(Reg::Rsp)),
                        Instr::Jump("start".to_owned()),
                    ],
                ),
                (
                    "conclusion".to_owned(),
                    vec![
                        Instr::AddQ(Arg::Immediate(8), Arg::Reg(Reg::Rsp)),
                        Instr::PopQ(Arg::Reg(Reg::Rbx)),
                        Instr::PopQ(Arg::Reg(Reg::Rbp)),
                        Instr::RetQ,
                    ],
                ),
            ]),
            stack_space: 16,
            global_labels: HashSet::from(["main".to_owned()]),
            callee_saved: HashSet::from([Reg::Rbx]),
        };
        assert_eq!(result, expected)
    }
}

/*
  Program {
    blocks: {
      "start": [MovQ(Immediate(1), Reg(Rbx)), MovQ(Immediate(42), Reg(Rcx)), AddQ(Immediate(7), Reg(Rbx)), MovQ(Deref(Rbp, -8), Reg(Rax)), MovQ(Reg(Rbx), Deref(Rbp, -16)), AddQ(Reg(Rcx), Reg(Rbx)), MovQ(Deref(Rbp, -16), Reg(Rcx)), NegQ(Reg(Rcx)), MovQ(Reg(Rbx), Reg(Rax)), AddQ(Reg(Rcx), Reg(Rax)), Jump("conclusion")],
      "conclusion": [AddQ(Immediate(24), Reg(Rsp)), PopQ(Reg(Rbx)), PopQ(Reg(Rbp)), RetQ],
      "main": [PushQ(Reg(Rbp)), MovQ(Reg(Rsp), Reg(Rbp)), PushQ(Reg(Rbx)), SubQ(Immediate(24), Reg(Rsp)), Jump("start")]
    },
    stack_space: 16,
    global_labels: {"main"},
    callee_saved: {Rbx}
  }

  Program {
    blocks: {
      "conclusion": [AddQ(Immediate(8), Reg(Rsp)), PopQ(Reg(Rbx)), PopQ(Reg(Rbp)), RetQ],
      "main": [PushQ(Reg(Rbp)), MovQ(Reg(Rsp), Reg(Rbp)), PushQ(Reg(Rbx)), SubQ(Immediate(8), Reg(Rsp)), Jump("start")],
      "start": [MovQ(Immediate(1), Reg(Rbx)), MovQ(Immediate(42), Reg(Rcx)), AddQ(Immediate(7), Reg(Rbx)), MovQ(Deref(Rbp, -8), Reg(Rax)), MovQ(Reg(Rbx), Deref(Rbp, -16)), AddQ(Reg(Rcx), Reg(Rbx)), MovQ(Deref(Rbp, -16), Reg(Rcx)), NegQ(Reg(Rcx)), MovQ(Reg(Rbx), Reg(Rax)), AddQ(Reg(Rcx), Reg(Rax)), Jump("conclusion")]
    },
    stack_space: 16,
    global_labels: {"main"},
    callee_saved: {Rbx}
  }
*/
