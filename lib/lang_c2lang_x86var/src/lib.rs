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
    type Target = lang_x86::Cc;
    type Arg = ();

    fn select_instructions(self, _: Self::Arg) -> lang_x86::Cc {
        match self {
            Comparator::Eq => lang_x86::Cc::E,
            Comparator::Lt => lang_x86::Cc::L,
            Comparator::Leq => lang_x86::Cc::Le,
            Comparator::Gt => lang_x86::Cc::G,
            Comparator::Geq => lang_x86::Cc::Ge,
        }
    }
}

#[cfg(test)]
mod select_instructions_tests {
    use super::SelectInstructions;
    use definitions::{BinaryOperation, Comparator, UnaryOperation};

    #[test]
    fn select_sum() {
        let mut prog = lang_c::Program::new();
        prog.add_block(
            "start",
            lang_c::Tail {
                stmts: vec![lang_c::Statement::assign(
                    "x0",
                    lang_c::Expression::bin(10.into(), BinaryOperation::Add, 32.into()),
                )],
                cont: lang_c::Continuation::Return(lang_c::Atom::Variable("x0".to_owned())),
            },
        );
        let result = prog.select_instructions(());
        let mut expected = lang_x86::VarProgram::new();
        expected.add_block(
            "start",
            vec![
                lang_x86::Instruction::MovQ {
                    src: 10.into(),
                    dest: "x0".into(),
                },
                lang_x86::Instruction::AddQ {
                    src: 32.into(),
                    dest: "x0".into(),
                },
                lang_x86::Instruction::MovQ {
                    src: "x0".into(),
                    dest: lang_x86::Reg::Rax.into(),
                },
                lang_x86::Instruction::Jump {
                    label: "conclusion".to_owned(),
                },
            ],
        );
        assert_eq!(result, expected)
    }

    #[test]
    fn select_neg_sum() {
        let mut prog = lang_c::Program::new();
        prog.add_block(
            "start",
            lang_c::Tail {
                stmts: vec![
                    lang_c::Statement::assign(
                        "x0",
                        lang_c::Expression::UnaryOp {
                            arg: lang_c::Atom::Integer(10),
                            op: UnaryOperation::Neg,
                        },
                    ),
                    lang_c::Statement::assign(
                        "x1",
                        lang_c::Expression::BinOp {
                            fst: 52.into(),
                            op: BinaryOperation::Add,
                            snd: "x0".into(),
                        },
                    ),
                ],
                cont: lang_c::Continuation::Return(lang_c::Atom::Variable("x1".to_owned())),
            },
        );
        let result = prog.select_instructions(());
        let mut expected = lang_x86::VarProgram::new();
        expected.add_block(
            "start",
            vec![
                lang_x86::Instruction::MovQ {
                    src: 10.into(),
                    dest: "x0".into(),
                },
                lang_x86::Instruction::NegQ { arg: "x0".into() },
                lang_x86::Instruction::MovQ {
                    src: 52.into(),
                    dest: "x1".into(),
                },
                lang_x86::Instruction::AddQ {
                    src: "x0".into(),
                    dest: "x1".into(),
                },
                lang_x86::Instruction::MovQ {
                    src: "x1".into(),
                    dest: lang_x86::Reg::Rax.into(),
                },
                lang_x86::Instruction::Jump {
                    label: "conclusion".to_owned(),
                },
            ],
        );
        assert_eq!(result, expected)
    }

    #[test]
    fn select_if() {
        let mut prog = lang_c::Program::new();
        prog.add_block(
            "start",
            lang_c::Tail {
                stmts: vec![
                    lang_c::Statement::assign("x0", lang_c::Expression::ReadInt),
                    lang_c::Statement::assign(
                        "x1",
                        lang_c::Expression::cmp("x0".into(), Comparator::Eq, 1.into()),
                    ),
                ],
                cont: lang_c::Continuation::If {
                    cond: "x1".into(),
                    then_label: "block_0".to_owned(),
                    else_label: "block_1".to_owned(),
                },
            },
        );
        prog.add_block(
            "block_0",
            lang_c::Tail {
                stmts: vec![],
                cont: lang_c::Continuation::Return(42.into()),
            },
        );
        prog.add_block(
            "block_1",
            lang_c::Tail {
                stmts: vec![],
                cont: lang_c::Continuation::Return(0.into()),
            },
        );
        let result = prog.select_instructions(());
        let mut expected = lang_x86::VarProgram::new();
        expected.add_block(
            "start",
            vec![
                lang_x86::Instruction::CallQ {
                    label: "read_int".to_owned(),
                },
                lang_x86::Instruction::MovQ {
                    src: lang_x86::Reg::Rax.into(),
                    dest: "x0".into(),
                },
                lang_x86::Instruction::CmpQ {
                    left: "x0".into(),
                    right: 1.into(),
                },
                lang_x86::Instruction::SetCC {
                    cc: lang_x86::Cc::E,
                    dest: lang_x86::Arg::ByteReg(lang_x86::ByteReg::Al).into(),
                },
                lang_x86::Instruction::MovZBQ {
                    src: lang_x86::Arg::ByteReg(lang_x86::ByteReg::Al).into(),
                    dest: "x1".into(),
                },
                lang_x86::Instruction::CmpQ {
                    left: "x1".into(),
                    right: 1.into(),
                },
                lang_x86::Instruction::JumpCC {
                    cc: lang_x86::Cc::E,
                    label: "block_0".to_owned(),
                },
                lang_x86::Instruction::Jump {
                    label: "block_1".to_owned(),
                },
            ],
        );
        expected.add_block(
            "block_0",
            vec![
                lang_x86::Instruction::MovQ {
                    src: 42.into(),
                    dest: lang_x86::Reg::Rax.into(),
                },
                lang_x86::Instruction::Jump {
                    label: "conclusion".to_owned(),
                },
            ],
        );
        expected.add_block(
            "block_1",
            vec![
                lang_x86::Instruction::MovQ {
                    src: 0.into(),
                    dest: lang_x86::Reg::Rax.into(),
                },
                lang_x86::Instruction::Jump {
                    label: "conclusion".to_owned(),
                },
            ],
        );
        assert_eq!(result, expected)
    }
}
