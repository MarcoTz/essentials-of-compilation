use std::{collections::HashSet, mem::swap};
mod errors;
mod state;
pub use errors::Error;
use state::ExplicateState;

pub struct BlockBuilder {
    stmts: Vec<core::Statement>,
}

impl BlockBuilder {
    pub fn new() -> BlockBuilder {
        BlockBuilder { stmts: vec![] }
    }

    pub fn to_block(self, lb: &str, cont: core::Continuation) -> core::Block {
        core::Block {
            label: lb.to_owned(),
            tail: core::Tail {
                stmts: self.stmts,
                cont,
            },
        }
    }
}

pub struct BlockAccum {
    blocks: Vec<core::Block>,
    current: BlockBuilder,
    current_label: String,
    used_labels: HashSet<String>,
}

impl BlockAccum {
    pub fn new() -> BlockAccum {
        BlockAccum {
            blocks: vec![],
            current: BlockBuilder::new(),
            current_label: "start".to_owned(),
            used_labels: HashSet::from(["start".to_owned()]),
        }
    }

    pub fn push_block(&mut self, block: core::Block) {
        self.used_labels.insert(block.label.clone());
        self.blocks.push(block)
    }

    pub fn push_stmt(&mut self, stmt: core::Statement) {
        self.current.stmts.push(stmt)
    }

    pub fn next_block(&mut self, cont: core::Continuation) {
        let mut old = BlockBuilder::new();
        swap(&mut self.current, &mut old);
        println!("adding block with label {}", self.current_label);
        let block = old.to_block(&self.current_label, cont);
        self.used_labels.insert(self.current_label.to_owned());
        self.blocks.push(block);
    }

    pub fn fresh_label(&mut self) -> String {
        let mut num = 0;
        let mut next = format!("block_{}", num);
        while self.used_labels.contains(&next) {
            num += 1;
            next = format!("block_{}", num);
        }
        self.used_labels.insert(next.clone());
        next
    }

    pub fn to_prog(self) -> core::Program {
        let mut prog = core::Program::new();
        for block in self.blocks {
            prog.add_block(&block.label, block.tail);
        }
        prog
    }
}

impl Default for BlockAccum {
    fn default() -> BlockAccum {
        BlockAccum::new()
    }
}

pub fn explicate_control(prog: monadic::Program) -> Result<core::Program, Error> {
    let mut accum = BlockAccum::new();
    prog.main.explicate_control(&mut accum)?;
    Ok(accum.to_prog())
}

pub trait ExplicateControl {
    type Target;
    fn explicate_control(self, accum: &mut BlockAccum) -> Result<Self::Target, Error>;
}

impl ExplicateControl for monadic::Block {
    type Target = ();
    fn explicate_control(self, accum: &mut BlockAccum) -> Result<Self::Target, Error> {
        for stmt in self.stmts {
            stmt.explicate_control(accum)?;
        }
        if !accum.current.stmts.is_empty() {
            println!("finishing block from explicate block");
            accum.next_block(core::Continuation::Return(core::Atom::Unit));
        }
        Ok(())
    }
}

impl ExplicateControl for monadic::Statement {
    type Target = ();
    fn explicate_control(self, state: &mut BlockAccum) -> Result<(), Error> {
        match self {
            monadic::Statement::Return(atm) => {
                let cont = core::Continuation::Return(atm.explicate_control(state)?);
                println!("finishing block from return statement");
                state.next_block(cont);
                Ok(())
            }
            monadic::Statement::Print(atm) => {
                let stmt = core::Statement::Print(atm.explicate_control(state)?);
                state.push_stmt(stmt);
                Ok(())
            }
            monadic::Statement::Assign { var, bound } => {
                let bound_exp = bound.explicate_control(state)?;
                state.push_stmt(core::Statement::assign(&var, bound_exp));
                Ok(())
            }
            monadic::Statement::If {
                cond_exp,
                then_block,
                else_block,
            } => {
                let cond = cond_exp.explicate_control(state)?;
                let then_label = state.fresh_label();
                let else_label = state.fresh_label();
                let cont = core::Continuation::If {
                    cond,
                    then_label: then_label.clone(),
                    else_label: else_label.clone(),
                };
                println!("finishing block from if statement");
                state.next_block(cont);
                println!("explicating then block with label {then_label}");
                state.current_label = then_label;
                then_block.explicate_control(state)?;
                println!("explicating else block with label {else_label}");
                state.current_label = else_label;
                else_block.explicate_control(state)?;
                Ok(())
            }
        }
    }
}

impl ExplicateControl for monadic::Expression {
    type Target = core::Expression;
    fn explicate_control(self, accum: &mut BlockAccum) -> Result<Self::Target, Error> {
        match self {
            monadic::Expression::Atm(atm) => {
                Ok(core::Expression::Atm(atm.explicate_control(accum)?))
            }
            monadic::Expression::ReadInt => Ok(core::Expression::ReadInt),
            monadic::Expression::UnaryOp { arg, op } => {
                let arg_exp = arg.explicate_control(accum)?;
                Ok(core::Expression::un(arg_exp, op))
            }
            monadic::Expression::BinaryOp { fst, op, snd } => {
                let fst_exp = fst.explicate_control(accum)?;
                let snd_exp = snd.explicate_control(accum)?;
                Ok(core::Expression::bin(fst_exp, op, snd_exp))
            }
            monadic::Expression::Cmp { left, cmp, right } => {
                let left_exp = left.explicate_control(accum)?;
                let right_exp = right.explicate_control(accum)?;
                Ok(core::Expression::cmp(left_exp, cmp, right_exp))
            }
        }
    }
}

impl ExplicateControl for monadic::Atom {
    type Target = core::Atom;

    fn explicate_control(self, _: &mut BlockAccum) -> Result<Self::Target, Error> {
        match self {
            monadic::Atom::Integer(i) => Ok(core::Atom::Integer(i)),
            monadic::Atom::Bool(b) => Ok(core::Atom::Bool(b)),
            monadic::Atom::Variable(v) => Ok(core::Atom::Variable(v)),
        }
    }
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
