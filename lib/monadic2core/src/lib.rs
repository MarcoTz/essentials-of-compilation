mod atom;
mod block;
mod builder;
mod errors;
mod expression;
mod statement;
use builder::BlockAccum;
pub use errors::Error;

pub fn explicate_control(prog: monadic::Program) -> Result<core::Program, Error> {
    let mut accum = BlockAccum::new();
    prog.main.explicate_control(&mut accum)?;
    Ok(accum.to_prog())
}

pub trait ExplicateControl {
    type Target;
    fn explicate_control(self, accum: &mut BlockAccum) -> Result<Self::Target, Error>;
}

#[cfg(test)]
mod explicate_tests {
    use super::{ExplicateControl, explicate_control};
    use definitions::{BinaryOperation, Comparator};

    #[test]
    fn explicate_if_nested() {
        let prog = monadic::Program::new(vec![
            monadic::Statement::assign("x", monadic::Expression::Atm(0.into())),
            monadic::Statement::assign("y", monadic::Expression::Atm(5.into())),
            monadic::Statement::assign(
                "z",
                monadic::Expression::cmp("x".into(), Comparator::Lt, 1.into()),
            ),
            monadic::Statement::cond(
                "z".into(),
                monadic::Block::new(vec![
                    monadic::Statement::assign(
                        "w",
                        monadic::Expression::cmp("x".into(), Comparator::Eq, 0.into()),
                    ),
                    monadic::Statement::cond(
                        "w".into(),
                        monadic::Block::new(vec![
                            monadic::Statement::assign(
                                "z",
                                monadic::Expression::bin(
                                    "y".into(),
                                    BinaryOperation::Add,
                                    2.into(),
                                ),
                            ),
                            monadic::Statement::Print("z".into()),
                        ]),
                        monadic::Block::new(vec![monadic::Statement::Print("y".into())]),
                    ),
                ]),
                monadic::Block::new(vec![
                    monadic::Statement::assign(
                        "z",
                        monadic::Expression::bin("y".into(), BinaryOperation::Add, 10.into()),
                    ),
                    monadic::Statement::Print("z".into()),
                ]),
            ),
        ]);
        let result = explicate_control(prog).unwrap();
        let mut expected = core::Program::new();
        expected.add_block(
            "start",
            core::Tail {
                stmts: vec![
                    core::Statement::assign("x", core::Expression::Atm(0.into())),
                    core::Statement::assign("y", core::Expression::Atm(5.into())),
                    core::Statement::assign(
                        "z",
                        core::Expression::cmp("x".into(), Comparator::Lt, 1.into()),
                    ),
                ],
                cont: core::Continuation::If {
                    cond: "z".into(),
                    then_label: "block_0".to_owned(),
                    else_label: "block_1".to_owned(),
                },
            },
        );
        expected.add_block(
            "block_0",
            core::Tail {
                stmts: vec![core::Statement::assign(
                    "w".into(),
                    core::Expression::cmp("x".into(), Comparator::Eq, 0.into()),
                )],
                cont: core::Continuation::If {
                    cond: "w".into(),
                    then_label: "block_2".to_owned(),
                    else_label: "block_3".to_owned(),
                },
            },
        );
        expected.add_block(
            "block_3",
            core::Tail {
                stmts: vec![core::Statement::Print("y".into())],
                cont: core::Continuation::Return(core::Atom::Unit),
            },
        );
        expected.add_block(
            "block_2",
            core::Tail {
                stmts: vec![
                    core::Statement::assign(
                        "z",
                        core::Expression::bin("y".into(), BinaryOperation::Add, 2.into()),
                    ),
                    core::Statement::Print("z".into()),
                ],
                cont: core::Continuation::Return(core::Atom::Unit),
            },
        );
        expected.add_block(
            "block_1",
            core::Tail {
                stmts: vec![
                    core::Statement::assign(
                        "z",
                        core::Expression::bin("y".into(), BinaryOperation::Add, 10.into()),
                    ),
                    core::Statement::Print("z".into()),
                ],
                cont: core::Continuation::Return(core::Atom::Unit),
            },
        );
        assert_eq!(result, expected)
    }
}
