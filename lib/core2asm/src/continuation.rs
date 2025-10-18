use super::SelectInstructions;

impl SelectInstructions for core::Continuation {
    type Target = Vec<asm::Instruction<asm::VarArg>>;
    type Arg = ();

    fn select_instructions(self, _: Self::Arg) -> Self::Target {
        match self {
            core::Continuation::Return(atm) => {
                let arg_dest = atm.select_instructions(());
                vec![
                    asm::Instruction::MovQ {
                        src: arg_dest,
                        dest: asm::Reg::Rax.into(),
                    },
                    asm::Instruction::Jump {
                        label: "conclusion".to_owned(),
                    },
                ]
            }
            core::Continuation::Goto(label) => vec![asm::Instruction::Jump { label }],
            core::Continuation::If {
                cond,
                then_label,
                else_label,
            } => {
                let cond_dest = cond.select_instructions(());
                let cmp = asm::Instruction::CmpQ {
                    left: cond_dest,
                    right: 1.into(),
                };
                let jump_true = asm::Instruction::JumpCC {
                    cc: asm::Cc::E,
                    label: then_label,
                };
                let jump_false = asm::Instruction::Jump { label: else_label };
                vec![cmp, jump_true, jump_false]
            }
        }
    }
}
