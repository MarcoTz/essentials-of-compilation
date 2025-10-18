mod errors;
mod state;
pub use errors::Error;
use state::ExplicateState;

pub trait ExplicateControl {
    type Target;
    fn explicate_control(self, state: &mut ExplicateState) -> Result<Self::Target, Error>;
}

enum StmtOrCont {
    Stmt(core::Statement),
    Cont(core::Continuation),
}

impl From<core::Statement> for StmtOrCont {
    fn from(stmt: core::Statement) -> StmtOrCont {
        StmtOrCont::Stmt(stmt)
    }
}

impl From<core::Continuation> for StmtOrCont {
    fn from(cont: core::Continuation) -> StmtOrCont {
        StmtOrCont::Cont(cont)
    }
}

impl ExplicateControl for monadic::Program {
    type Target = core::Program;
    fn explicate_control(self, state: &mut ExplicateState) -> Result<Self::Target, Error> {
        let mut new_prog = core::Program::new();
        explicate_tail(self.main, state, true)?;
        state.move_blocks(&mut new_prog);
        Ok(new_prog)
    }
}

fn explicate_tail(
    block: monadic::Block,
    state: &mut ExplicateState,
    mut is_start: bool,
) -> Result<String, Error> {
    let mut block_stmts = vec![];
    let mut first_label = None;
    for stmt in block.stmts {
        match explicate_statement(stmt, state)? {
            StmtOrCont::Cont(c) => {
                let tail = core::Tail {
                    stmts: block_stmts,
                    cont: c,
                };
                let label = if is_start { Some("start") } else { None };
                let new_label = create_block(tail, label, state);
                if first_label.is_none() {
                    first_label = Some(new_label);
                }
                is_start = false;
                block_stmts = vec![];
            }
            StmtOrCont::Stmt(stmt) => block_stmts.push(stmt),
        }
    }
    if !block_stmts.is_empty() {
        let label = if is_start { Some("start") } else { None };
        let next_label = create_block(
            core::Tail {
                stmts: block_stmts,
                cont: core::Continuation::Return(core::Atom::Unit),
            },
            label,
            state,
        );
        if first_label.is_none() {
            first_label = Some(next_label)
        }
    }

    match first_label {
        None => Ok(state.fresh_label()),
        Some(lb) => Ok(lb),
    }
}

fn create_block(tail: core::Tail, label: Option<&str>, state: &mut ExplicateState) -> String {
    state.add_block(tail, label)
}

fn explicate_statement(
    stmt: monadic::Statement,
    state: &mut ExplicateState,
) -> Result<StmtOrCont, Error> {
    match stmt {
        monadic::Statement::Return(atm) => {
            Ok(core::Continuation::Return(explicate_atm(atm)).into())
        }
        monadic::Statement::Print(atm) => Ok(core::Statement::Print(explicate_atm(atm)).into()),
        monadic::Statement::Assign { var, bound } => {
            let bound_exp = explicate_exp(bound);
            Ok(core::Statement::assign(&var, bound_exp).into())
        }
        monadic::Statement::If {
            cond_exp,
            then_block,
            else_block,
        } => {
            let cond = explicate_atm(cond_exp);
            let then_label = explicate_tail(then_block, state, false)?;
            let else_label = explicate_tail(else_block, state, false)?;
            Ok(core::Continuation::If {
                cond,
                then_label,
                else_label,
            }
            .into())
        }
    }
}

fn explicate_exp(exp: monadic::Expression) -> core::Expression {
    match exp {
        monadic::Expression::Atm(atm) => core::Expression::Atm(explicate_atm(atm)),
        monadic::Expression::ReadInt => core::Expression::ReadInt,
        monadic::Expression::UnaryOp { arg, op } => {
            let arg_exp = explicate_atm(arg);
            core::Expression::un(arg_exp, op)
        }
        monadic::Expression::BinaryOp { fst, op, snd } => {
            let fst_exp = explicate_atm(fst);
            let snd_exp = explicate_atm(snd);
            core::Expression::bin(fst_exp, op, snd_exp)
        }
        monadic::Expression::Cmp { left, cmp, right } => {
            let left_exp = explicate_atm(left);
            let right_exp = explicate_atm(right);
            core::Expression::cmp(left_exp, cmp, right_exp)
        }
    }
}

fn explicate_atm(atm: monadic::Atom) -> core::Atom {
    match atm {
        monadic::Atom::Integer(i) => core::Atom::Integer(i),
        monadic::Atom::Bool(b) => core::Atom::Bool(b),
        monadic::Atom::Variable(v) => core::Atom::Variable(v),
    }
}

#[cfg(test)]
mod explicate_tests {
    use super::ExplicateControl;
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
        let result = prog.explicate_control(&mut Default::default()).unwrap();
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
                    then_label: "block_2".to_owned(),
                    else_label: "block_3".to_owned(),
                },
            },
        );
        expected.add_block(
            "block_2",
            core::Tail {
                stmts: vec![core::Statement::assign(
                    "w".into(),
                    core::Expression::cmp("x".into(), Comparator::Eq, 0.into()),
                )],
                cont: core::Continuation::If {
                    cond: "w".into(),
                    then_label: "block_0".to_owned(),
                    else_label: "block_1".to_owned(),
                },
            },
        );
        expected.add_block(
            "block_1",
            core::Tail {
                stmts: vec![core::Statement::Print("y".into())],
                cont: core::Continuation::Return(core::Atom::Unit),
            },
        );
        expected.add_block(
            "block_0",
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
            "block_3",
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
