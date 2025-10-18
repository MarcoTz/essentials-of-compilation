use super::SelectInstructions;
use definitions::{BinaryOperation, READ_INT_CALL, UnaryOperation};

impl SelectInstructions for core::Expression {
    type Target = Vec<asm::Instruction<asm::VarArg>>;
    type Arg = asm::VarArg;
    fn select_instructions(self, dest: asm::VarArg) -> Self::Target {
        match self {
            core::Expression::Atm(atm) => vec![asm::Instruction::MovQ {
                src: atm.select_instructions(()),
                dest,
            }],
            core::Expression::ReadInt => vec![
                asm::Instruction::CallQ {
                    label: READ_INT_CALL.to_owned(),
                },
                asm::Instruction::MovQ {
                    src: asm::Reg::Rax.into(),
                    dest,
                },
            ],
            core::Expression::UnaryOp { arg, op } => {
                let arg_loc = arg.select_instructions(());
                match op {
                    UnaryOperation::Neg => vec![
                        asm::Instruction::MovQ {
                            src: arg_loc,
                            dest: dest.clone(),
                        },
                        asm::Instruction::NegQ { arg: dest },
                    ],
                    UnaryOperation::Not => vec![
                        asm::Instruction::MovQ {
                            src: arg_loc,
                            dest: dest.clone(),
                        },
                        asm::Instruction::XorQ {
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
                        asm::Instruction::MovQ {
                            src: fst_loc,
                            dest: dest.clone(),
                        },
                        asm::Instruction::AddQ { src: snd_loc, dest },
                    ],
                    BinaryOperation::Sub => vec![
                        asm::Instruction::MovQ {
                            src: fst_loc,
                            dest: dest.clone(),
                        },
                        asm::Instruction::SubQ { src: snd_loc, dest },
                    ],
                    BinaryOperation::And => vec![
                        asm::Instruction::MovQ {
                            src: fst_loc,
                            dest: dest.clone(),
                        },
                        asm::Instruction::AndQ { src: snd_loc, dest },
                    ],
                    BinaryOperation::Or => vec![
                        asm::Instruction::MovQ {
                            src: fst_loc,
                            dest: dest.clone(),
                        },
                        asm::Instruction::OrQ { src: snd_loc, dest },
                    ],
                }
            }
            core::Expression::Cmp { left, cmp, right } => {
                let left_dest = left.select_instructions(());
                let right_dest = right.select_instructions(());
                let cc = cmp.select_instructions(());
                vec![
                    asm::Instruction::CmpQ {
                        left: left_dest,
                        right: right_dest,
                    },
                    asm::Instruction::SetCC {
                        cc,
                        dest: asm::ByteReg::Al.into(),
                    },
                    asm::Instruction::MovZBQ {
                        src: asm::ByteReg::Al.into(),
                        dest,
                    },
                ]
            }
        }
    }
}
