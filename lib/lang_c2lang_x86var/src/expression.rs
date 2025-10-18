use super::SelectInstructions;
use definitions::{BinaryOperation, READ_INT_CALL, UnaryOperation};

impl SelectInstructions for core::Expression {
    type Target = Vec<lang_x86::Instruction<lang_x86::VarArg>>;
    type Arg = lang_x86::VarArg;
    fn select_instructions(self, dest: lang_x86::VarArg) -> Self::Target {
        match self {
            core::Expression::Atm(atm) => vec![lang_x86::Instruction::MovQ {
                src: atm.select_instructions(()),
                dest,
            }],
            core::Expression::ReadInt => vec![
                lang_x86::Instruction::CallQ {
                    label: READ_INT_CALL.to_owned(),
                },
                lang_x86::Instruction::MovQ {
                    src: lang_x86::Reg::Rax.into(),
                    dest,
                },
            ],
            core::Expression::UnaryOp { arg, op } => {
                let arg_loc = arg.select_instructions(());
                match op {
                    UnaryOperation::Neg => vec![
                        lang_x86::Instruction::MovQ {
                            src: arg_loc,
                            dest: dest.clone(),
                        },
                        lang_x86::Instruction::NegQ { arg: dest },
                    ],
                    UnaryOperation::Not => vec![
                        lang_x86::Instruction::MovQ {
                            src: arg_loc,
                            dest: dest.clone(),
                        },
                        lang_x86::Instruction::XorQ {
                            src: 1.into(),
                            dest,
                        },
                    ],
                }
            }
            core::Expression::BinOp { fst, op, snd } => {
                let fst_loc = fst.select_instructions(());
                let snd_loc = snd.select_instructions(());
                match op {
                    BinaryOperation::Add => vec![
                        lang_x86::Instruction::MovQ {
                            src: fst_loc,
                            dest: dest.clone(),
                        },
                        lang_x86::Instruction::AddQ { src: snd_loc, dest },
                    ],
                    BinaryOperation::Sub => vec![
                        lang_x86::Instruction::MovQ {
                            src: fst_loc,
                            dest: dest.clone(),
                        },
                        lang_x86::Instruction::SubQ { src: snd_loc, dest },
                    ],
                    BinaryOperation::And => vec![
                        lang_x86::Instruction::MovQ {
                            src: fst_loc,
                            dest: dest.clone(),
                        },
                        lang_x86::Instruction::AndQ { src: snd_loc, dest },
                    ],
                    BinaryOperation::Or => vec![
                        lang_x86::Instruction::MovQ {
                            src: fst_loc,
                            dest: dest.clone(),
                        },
                        lang_x86::Instruction::OrQ { src: snd_loc, dest },
                    ],
                }
            }
            core::Expression::Cmp { left, cmp, right } => {
                let left_dest = left.select_instructions(());
                let right_dest = right.select_instructions(());
                let cc = cmp.select_instructions(());
                vec![
                    lang_x86::Instruction::CmpQ {
                        left: left_dest,
                        right: right_dest,
                    },
                    lang_x86::Instruction::SetCC {
                        cc,
                        dest: lang_x86::ByteReg::Al.into(),
                    },
                    lang_x86::Instruction::MovZBQ {
                        src: lang_x86::ByteReg::Al.into(),
                        dest,
                    },
                ]
            }
        }
    }
}
