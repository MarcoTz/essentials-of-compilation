use definitions::Comparator;

mod atom;
mod continuation;
mod expression;
mod program;
mod statement;
mod tail;

pub trait SelectInstructions {
    type Target;
    type Arg;
    fn select_instructions(self, arg: Self::Arg) -> Self::Target;
}

impl SelectInstructions for Comparator {
    type Target = asm::Cc;
    type Arg = ();

    fn select_instructions(self, _: Self::Arg) -> asm::Cc {
        match self {
            Comparator::Eq => asm::Cc::E,
            Comparator::Lt => asm::Cc::L,
            Comparator::Leq => asm::Cc::Le,
            Comparator::Gt => asm::Cc::G,
            Comparator::Geq => asm::Cc::Ge,
        }
    }
}

#[cfg(test)]
mod select_instructions_tests {
    use super::SelectInstructions;
    use definitions::{BinaryOperation, Comparator, UnaryOperation};

    #[test]
    fn select_sum() {
        let mut prog = core::Program::new();
        prog.add_block(
            "start",
            core::Tail {
                stmts: vec![core::Statement::assign(
                    "x0",
                    core::Expression::bin(10.into(), BinaryOperation::Add, 32.into()),
                )],
                cont: core::Continuation::Return(core::Atom::Variable("x0".to_owned())),
            },
        );
        let result = prog.select_instructions(());
        let mut expected = asm::VarProgram::new();
        expected.add_block(
            "start",
            vec![
                asm::Instruction::MovQ {
                    src: 10.into(),
                    dest: "x0".into(),
                },
                asm::Instruction::AddQ {
                    src: 32.into(),
                    dest: "x0".into(),
                },
                asm::Instruction::MovQ {
                    src: "x0".into(),
                    dest: asm::Reg::Rax.into(),
                },
                asm::Instruction::Jump {
                    label: "conclusion".to_owned(),
                },
            ],
        );
        assert_eq!(result, expected)
    }

    #[test]
    fn select_neg_sum() {
        let mut prog = core::Program::new();
        prog.add_block(
            "start",
            core::Tail {
                stmts: vec![
                    core::Statement::assign(
                        "x0",
                        core::Expression::UnaryOp {
                            arg: core::Atom::Integer(10),
                            op: UnaryOperation::Neg,
                        },
                    ),
                    core::Statement::assign(
                        "x1",
                        core::Expression::BinOp {
                            fst: 52.into(),
                            op: BinaryOperation::Add,
                            snd: "x0".into(),
                        },
                    ),
                ],
                cont: core::Continuation::Return(core::Atom::Variable("x1".to_owned())),
            },
        );
        let result = prog.select_instructions(());
        let mut expected = asm::VarProgram::new();
        expected.add_block(
            "start",
            vec![
                asm::Instruction::MovQ {
                    src: 10.into(),
                    dest: "x0".into(),
                },
                asm::Instruction::NegQ { arg: "x0".into() },
                asm::Instruction::MovQ {
                    src: 52.into(),
                    dest: "x1".into(),
                },
                asm::Instruction::AddQ {
                    src: "x0".into(),
                    dest: "x1".into(),
                },
                asm::Instruction::MovQ {
                    src: "x1".into(),
                    dest: asm::Reg::Rax.into(),
                },
                asm::Instruction::Jump {
                    label: "conclusion".to_owned(),
                },
            ],
        );
        assert_eq!(result, expected)
    }

    #[test]
    fn select_if() {
        let mut prog = core::Program::new();
        prog.add_block(
            "start",
            core::Tail {
                stmts: vec![
                    core::Statement::assign("x0", core::Expression::ReadInt),
                    core::Statement::assign(
                        "x1",
                        core::Expression::cmp("x0".into(), Comparator::Eq, 1.into()),
                    ),
                ],
                cont: core::Continuation::If {
                    cond: "x1".into(),
                    then_label: "block_0".to_owned(),
                    else_label: "block_1".to_owned(),
                },
            },
        );
        prog.add_block(
            "block_0",
            core::Tail {
                stmts: vec![],
                cont: core::Continuation::Return(42.into()),
            },
        );
        prog.add_block(
            "block_1",
            core::Tail {
                stmts: vec![],
                cont: core::Continuation::Return(0.into()),
            },
        );
        let result = prog.select_instructions(());
        let mut expected = asm::VarProgram::new();
        expected.add_block(
            "start",
            vec![
                asm::Instruction::CallQ {
                    label: "read_int".to_owned(),
                },
                asm::Instruction::MovQ {
                    src: asm::Reg::Rax.into(),
                    dest: "x0".into(),
                },
                asm::Instruction::CmpQ {
                    left: "x0".into(),
                    right: 1.into(),
                },
                asm::Instruction::SetCC {
                    cc: asm::Cc::E,
                    dest: asm::Arg::ByteReg(asm::ByteReg::Al).into(),
                },
                asm::Instruction::MovZBQ {
                    src: asm::Arg::ByteReg(asm::ByteReg::Al).into(),
                    dest: "x1".into(),
                },
                asm::Instruction::CmpQ {
                    left: "x1".into(),
                    right: 1.into(),
                },
                asm::Instruction::JumpCC {
                    cc: asm::Cc::E,
                    label: "block_0".to_owned(),
                },
                asm::Instruction::Jump {
                    label: "block_1".to_owned(),
                },
            ],
        );
        expected.add_block(
            "block_0",
            vec![
                asm::Instruction::MovQ {
                    src: 42.into(),
                    dest: asm::Reg::Rax.into(),
                },
                asm::Instruction::Jump {
                    label: "conclusion".to_owned(),
                },
            ],
        );
        expected.add_block(
            "block_1",
            vec![
                asm::Instruction::MovQ {
                    src: 0.into(),
                    dest: asm::Reg::Rax.into(),
                },
                asm::Instruction::Jump {
                    label: "conclusion".to_owned(),
                },
            ],
        );
        assert_eq!(result, expected)
    }
}
