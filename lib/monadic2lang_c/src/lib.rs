mod errors;
mod state;
pub use errors::Error;
use state::ExplicateState;

pub trait ExplicateControl {
    type Target;
    fn explicate_control(self, state: &mut ExplicateState) -> Result<Self::Target, Error>;
}

enum StmtOrCont {
    Stmt(lang_c::Statement),
    Cont(lang_c::Continuation),
}

impl From<lang_c::Statement> for StmtOrCont {
    fn from(stmt: lang_c::Statement) -> StmtOrCont {
        StmtOrCont::Stmt(stmt)
    }
}

impl From<lang_c::Continuation> for StmtOrCont {
    fn from(cont: lang_c::Continuation) -> StmtOrCont {
        StmtOrCont::Cont(cont)
    }
}

impl ExplicateControl for monadic::Program {
    type Target = lang_c::Program;
    fn explicate_control(self, state: &mut ExplicateState) -> Result<Self::Target, Error> {
        let mut new_prog = lang_c::Program::new();
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
                let tail = lang_c::Tail {
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
            lang_c::Tail {
                stmts: block_stmts,
                cont: lang_c::Continuation::Return(lang_c::Atom::Unit),
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

fn create_block(tail: lang_c::Tail, label: Option<&str>, state: &mut ExplicateState) -> String {
    state.add_block(tail, label)
}

fn explicate_statement(
    stmt: monadic::Statement,
    state: &mut ExplicateState,
) -> Result<StmtOrCont, Error> {
    match stmt {
        monadic::Statement::Return(atm) => {
            Ok(lang_c::Continuation::Return(explicate_atm(atm)).into())
        }
        monadic::Statement::Print(atm) => Ok(lang_c::Statement::Print(explicate_atm(atm)).into()),
        monadic::Statement::Assign { var, bound } => {
            let bound_exp = explicate_exp(bound);
            Ok(lang_c::Statement::assign(&var, bound_exp).into())
        }
        monadic::Statement::If {
            cond_exp,
            then_block,
            else_block,
        } => {
            let cond = explicate_atm(cond_exp);
            let then_label = explicate_tail(then_block, state, false)?;
            let else_label = explicate_tail(else_block, state, false)?;
            Ok(lang_c::Continuation::If {
                cond,
                then_label,
                else_label,
            }
            .into())
        }
    }
}

fn explicate_exp(exp: monadic::Expression) -> lang_c::Expression {
    match exp {
        monadic::Expression::Atm(atm) => lang_c::Expression::Atm(explicate_atm(atm)),
        monadic::Expression::ReadInt => lang_c::Expression::ReadInt,
        monadic::Expression::UnaryOp { arg, op } => {
            let arg_exp = explicate_atm(arg);
            lang_c::Expression::un(arg_exp, op)
        }
        monadic::Expression::BinaryOp { fst, op, snd } => {
            let fst_exp = explicate_atm(fst);
            let snd_exp = explicate_atm(snd);
            lang_c::Expression::bin(fst_exp, op, snd_exp)
        }
        monadic::Expression::Cmp { left, cmp, right } => {
            let left_exp = explicate_atm(left);
            let right_exp = explicate_atm(right);
            lang_c::Expression::cmp(left_exp, cmp, right_exp)
        }
    }
}

fn explicate_atm(atm: monadic::Atom) -> lang_c::Atom {
    match atm {
        monadic::Atom::Integer(i) => lang_c::Atom::Integer(i),
        monadic::Atom::Bool(b) => lang_c::Atom::Bool(b),
        monadic::Atom::Variable(v) => lang_c::Atom::Variable(v),
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
        let mut expected = lang_c::Program::new();
        expected.add_block(
            "start",
            lang_c::Tail {
                stmts: vec![
                    lang_c::Statement::assign("x", lang_c::Expression::Atm(0.into())),
                    lang_c::Statement::assign("y", lang_c::Expression::Atm(5.into())),
                    lang_c::Statement::assign(
                        "z",
                        lang_c::Expression::cmp("x".into(), Comparator::Lt, 1.into()),
                    ),
                ],
                cont: lang_c::Continuation::If {
                    cond: "z".into(),
                    then_label: "block_2".to_owned(),
                    else_label: "block_3".to_owned(),
                },
            },
        );
        expected.add_block(
            "block_2",
            lang_c::Tail {
                stmts: vec![lang_c::Statement::assign(
                    "w".into(),
                    lang_c::Expression::cmp("x".into(), Comparator::Eq, 0.into()),
                )],
                cont: lang_c::Continuation::If {
                    cond: "w".into(),
                    then_label: "block_0".to_owned(),
                    else_label: "block_1".to_owned(),
                },
            },
        );
        expected.add_block(
            "block_1",
            lang_c::Tail {
                stmts: vec![lang_c::Statement::Print("y".into())],
                cont: lang_c::Continuation::Return(lang_c::Atom::Unit),
            },
        );
        expected.add_block(
            "block_0",
            lang_c::Tail {
                stmts: vec![
                    lang_c::Statement::assign(
                        "z",
                        lang_c::Expression::bin("y".into(), BinaryOperation::Add, 2.into()),
                    ),
                    lang_c::Statement::Print("z".into()),
                ],
                cont: lang_c::Continuation::Return(lang_c::Atom::Unit),
            },
        );
        expected.add_block(
            "block_3",
            lang_c::Tail {
                stmts: vec![
                    lang_c::Statement::assign(
                        "z",
                        lang_c::Expression::bin("y".into(), BinaryOperation::Add, 10.into()),
                    ),
                    lang_c::Statement::Print("z".into()),
                ],
                cont: lang_c::Continuation::Return(lang_c::Atom::Unit),
            },
        );
        assert_eq!(result, expected)
    }
}
