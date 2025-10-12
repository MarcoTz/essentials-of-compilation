use syntax::{Comparator, UnaryOperation, lang_c, lang_mon};

mod errors;
pub use errors::Error;

pub struct ExplicateState {
    blocks: Vec<lang_c::Block>,
}

impl ExplicateState {
    pub fn new() -> ExplicateState {
        ExplicateState { blocks: vec![] }
    }

    fn fresh_label(&self) -> String {
        let prefix = "block_";
        let mut num = 0;
        let mut label = format!("{prefix}{num}");
        while self
            .blocks
            .iter()
            .map(|block| &block.label)
            .collect::<Vec<_>>()
            .contains(&&label)
        {
            num += 1;
            label = format!("{prefix}{num}");
        }
        label
    }

    pub fn add_block(&mut self, tail: lang_c::Tail) -> String {
        let label = self.fresh_label();
        let block = lang_c::Block::new(&label, tail);
        self.blocks.push(block);
        label
    }

    fn move_blocks(self, prog: &mut lang_c::Program) {
        for block in self.blocks {
            prog.add_block(&block.label, block.tail);
        }
    }
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

pub fn explicate_control(prog: lang_mon::Program) -> Result<lang_c::Program, Error> {
    let mut new_prog = lang_c::Program::new();
    let mut state = ExplicateState::new();
    explicate_tail(prog.main, &mut state, true)?;
    state.move_blocks(&mut new_prog);
    Ok(new_prog)
}

fn explicate_tail(
    block: lang_mon::Block,
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
                let new_label = create_block(tail, is_start, state);
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
        let next_label = create_block(
            lang_c::Tail {
                stmts: block_stmts,
                cont: lang_c::Continuation::Return(lang_c::Atom::Unit),
            },
            is_start,
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

fn create_block(tail: lang_c::Tail, is_start: bool, state: &mut ExplicateState) -> String {
    if !is_start {
        state.add_block(tail)
    } else {
        state.blocks.push(lang_c::Block {
            label: "start".to_owned(),
            tail,
        });
        "start".to_owned()
    }
}

fn explicate_statement(
    stmt: lang_mon::Statement,
    state: &mut ExplicateState,
) -> Result<StmtOrCont, Error> {
    match stmt {
        lang_mon::Statement::Return(atm) => {
            Ok(lang_c::Continuation::Return(explicate_atm(atm)).into())
        }
        lang_mon::Statement::Print(atm) => Ok(lang_c::Statement::Print(explicate_atm(atm)).into()),
        lang_mon::Statement::Assign { var, bound } => {
            let bound_exp = explicate_exp(bound);
            Ok(lang_c::Statement::assign(&var, bound_exp).into())
        }
        lang_mon::Statement::If {
            cond_exp,
            then_block,
            else_block,
        } => {
            let cond_exp = explicate_exp(cond_exp);
            let (left, cmp, right) = explicate_cmp(cond_exp)?;
            let then_label = explicate_tail(then_block, state, false)?;
            let else_label = explicate_tail(else_block, state, false)?;
            Ok(lang_c::Continuation::If {
                left,
                cmp,
                right,
                then_label,
                else_label,
            }
            .into())
        }
    }
}

fn explicate_exp(exp: lang_mon::Expression) -> lang_c::Expression {
    match exp {
        lang_mon::Expression::Atm(atm) => lang_c::Expression::Atm(explicate_atm(atm)),
        lang_mon::Expression::ReadInt => lang_c::Expression::ReadInt,
        lang_mon::Expression::UnaryOp { arg, op } => {
            let arg_exp = explicate_atm(arg);
            lang_c::Expression::un(arg_exp, op)
        }
        lang_mon::Expression::BinaryOp { fst, op, snd } => {
            let fst_exp = explicate_atm(fst);
            let snd_exp = explicate_atm(snd);
            lang_c::Expression::bin(fst_exp, op, snd_exp)
        }
        lang_mon::Expression::Cmp { left, cmp, right } => {
            let left_exp = explicate_atm(left);
            let right_exp = explicate_atm(right);
            lang_c::Expression::cmp(left_exp, cmp, right_exp)
        }
    }
}

fn explicate_atm(atm: lang_mon::Atom) -> lang_c::Atom {
    match atm {
        lang_mon::Atom::Integer(i) => lang_c::Atom::Integer(i),
        lang_mon::Atom::Bool(b) => lang_c::Atom::Bool(b),
        lang_mon::Atom::Variable(v) => lang_c::Atom::Variable(v),
    }
}

fn explicate_cmp(
    exp: lang_c::Expression,
) -> Result<(lang_c::Atom, Comparator, lang_c::Atom), Error> {
    match exp {
        lang_c::Expression::Atm(atm) => Ok((atm, Comparator::Eq, lang_c::Atom::Bool(true))),
        lang_c::Expression::Cmp { left, cmp, right } => Ok((left, cmp, right)),
        lang_c::Expression::UnaryOp {
            arg,
            op: UnaryOperation::Not,
        } => Ok((arg, Comparator::Eq, lang_c::Atom::Bool(false))),
        _ => Err(Error::BadCmp(exp)),
    }
}
