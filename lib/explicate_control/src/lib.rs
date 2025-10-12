use std::collections::HashSet;
use syntax::{BinaryOperation, Comparator, UnaryOperation, UsedVars, fresh_var, lang_c, lang_mon};

mod errors;
pub use errors::Error;

struct ExplicateState {
    prog: lang_c::Program,
    used_vars: HashSet<String>,
    current_label: String,
    current_stmts: Vec<lang_c::Statement>,
}

impl ExplicateState {
    fn new_label(&self) -> String {
        let mut num = 0;
        let mut label = format!("block{num}");
        let labels = self.prog.get_labels();
        while labels.contains(label) {
            num += 1;
            label = format!("block{num}");
        }
        label
    }
}

//start label: start
// each expression atm,readint ,unary,binary, cmp does nothing -> can be discarded
// if they are the last they get a return
// each expression let gives an assign statement
// each expression print gives a print statement
// both of these can appear in the middle
// each expression if gives a tail and two new blocks
// if can appear at any point which changes the current block

pub fn explicate_control(prog: lang_mon::Program) -> Result<lang_c::Program, Error> {
    let mut used_vars = prog.used_vars();
    let mut new_prog = lang_c::Program::new();
    let mut state = ExplicateState {
        prog: new_prog,
        used_vars,
        current_label: "start".to_owned(),
        current_stmts: vec![],
    };
    exps_to_block(prog, &mut state);
    todo!()
}

fn exps_to_block(exps: Vec<lang_mon::Expression>, state: &mut ExplicateState) -> Result<(), Error> {
    let mut stmts = vec![];
    for (ind, exp) in exps.into_iter().enumerate() {
        match exp {
            lang_mon::Expression::Print(exp) => {
                stmts.push(lang_c::Statement::Print(mon_to_c_atm(exp)));
                if ind == exps.len() - 1 {
                    state.prog.add_block(
                        &state.current_label,
                        lang_c::Tail {
                            stmts,
                            ret: lang_c::TailEnd::Return(lang_c::Expression::Unit),
                        },
                    );
                    return Ok(());
                }
            }
            lang_mon::Expression::LetIn { var, bound } => {
                let old_label = state.current_label.clone();
                let bind_label = state.new_label();
                state.current_label = bind_label;
                todo!()
            }
            lang_mon::Expression::If {
                cond_exp,
                then_block,
                else_block,
            } => {
                let old_label = state.current_label.clone();
                let then_label = state.new_label();
                state.current_label = then_label;
                exps_to_block(then_block, state)?;
                let else_label = state.new_label();
                state.current_label = else_label;
                exps_to_block(else_block, state)?;
                let next_label = state.new_label();
                state.current_label = next_label;
                exps_to_block(exps, state)?;
                update_return(then_label, next_label, state)?;
                update_return(else_label, next_label, state)?;
                let new_tail = exp_to_cmp(cond_exp, then_label, else_label, state)?;
                state.prog.add_block(&old_label, new_tail);
                return Ok(());
            }
            _ => continue,
        }
    }
    Ok(())
}
fn update_return(
    to_update: String,
    new_return: String,
    state: &mut ExplicateState,
) -> Result<(), Error> {
    let block = state
        .prog
        .get_block_mut(&to_update)
        .ok_or(Error::BlockNotFound(to_update))?;
    match block.tail.ret {
        lang_c::TailEnd::Goto(lb) => {
            if new_return == lb {
                Ok(())
            } else {
                update_return(lb, new_return, state)
            }
        }
        lang_c::TailEnd::Return(_) => {
            block.tail.ret = lang_c::TailEnd::Goto(new_return);
            Ok(())
        }
        lang_c::TailEnd::If {
            then_label,
            else_label,
            ..
        } => {
            update_return(then_label, new_return.clone(), state)?;
            update_return(else_label, new_return, state)?;
            Ok(())
        }
    }
}

fn exp_to_cmp(
    exp: lang_mon::Expression,
    then_label: String,
    else_label: String,
    state: &mut ExplicateState,
) -> Result<lang_c::Tail, Error> {
    match exp {
        lang_mon::Expression::Atm(lang_mon::Atom::Bool(b)) => {
            if b {
                Ok(lang_c::Tail {
                    stmts,
                    ret: lang_c::TailEnd::Goto(then_label),
                })
            } else {
                Ok(lang_c::Tail {
                    stmts,
                    ret: lang_c::TailEnd::Goto(else_label),
                })
            }
        }
        lang_mon::Expression::UnaryOp {
            arg,
            op: UnaryOperation::Not,
        } => {
            let new_var = fresh_var(&used_vars);
            let new_stmt =
                lang_c::Statement::assign(&new_var, lang_c::Expression::un(op, mon_to_c_atm(arg)));
            stmts.push(new_stmt);
            used_vars.insert(new_var.clone());
            Ok(lang_c::Tail {
                stmts,
                ret: lang_c::TailEnd::If {
                    left: lang_c::Atom::Variable(new_var),
                    cmp: Comparator::Eq,
                    right: lang_c::Atom::Bool(true),
                    then_label,
                    else_label,
                },
            })
        }
        lang_mon::Expression::BinaryOp { fst, op, snd }
            if op == BinaryOperation::And || op == BinaryOperation::Or_ =>
        {
            let new_var = fresh_var(&used_vars);
            let new_stmt = lang_c::Statement::assign(
                &new_var,
                lang_c::Expression::bin(mon_to_c_atm(fst), op, mon_to_c_atm(snd)),
            );
            stmts.push(new_stmt);
            used_vars.insert(new_var.clone());
            Ok(lang_c::Tail {
                stmts,
                ret: lang_c::TailEnd::If {
                    left: lang_c::Atom::Variable(new_var),
                    cmp: Comparator::Eq,
                    right: lang_c::Atom::Bool(true),
                    then_label,
                    else_label,
                },
            })
        }
        lang_c::Expression::Cmp { left, cmp, right } => Ok(lang_c::Tail {
            stmts,
            ret: lang_c::TailEnd::If {
                left,
                cmp,
                right,
                then_label,
                else_label,
            },
        }),
        _ => Err(Error::BadCmp(exp)),
    }
}

fn exp_to_assign(
    exp: lang_mon::Expression,
    var: String,
    state: &mut ExplicateState,
) -> Result<Vec<lang_c::Statement>, Error> {
    match exp {
        lang_mon::Expression::Atm(atm) => Ok(vec![lang_c::Statement::assign(
            &var,
            mon_to_c_atm(atm).into(),
        )]),
        lang_mon::Expression::ReadInt => Ok(vec![lang_c::Statement::assign(
            &var,
            lang_c::Expression::ReadInt,
        )]),
        lang_mon::Expression::Print(atm) => Ok(vec![
            lang_c::Statement::Print(mon_to_c_atm(atm)),
            lang_c::Statement::assign(vec, lang_c::Expression::Unit),
        ]),
        lang_mon::Expression::UnaryOp { arg, op } => Ok(vec![lang_c::Statement::assign(
            &var,
            lang_c::Expression::un(mon_to_c_atm(arg), op),
        )]),
        lang_mon::Expression::BinaryOp { fst, op, snd } => Ok(vec![lang_c::Statement::assign(
            &var,
            lang_c::Expression::bin(mon_to_c_atm(fst), op, mon_to_c_atm(snd)),
        )]),
        lang_mon::Expression::Cmp { left, cmp, right } => Ok(vec![lang_c::Statement::assign(
            &var,
            lang_c::Expression::cmp(mon_to_c_atm(left), cmp, mon_to_c_atm(right)),
        )]),
        lang_mon::Expression::LetIn { var: v, bound } => {
            let mut bind_stmts = exp_to_assign(v, bound, state)?;
            bound_stmts.push(lang_c::Statement::assign(&var, lang_c::Expression::Unit));
            Ok(bound_stmts)
        }
        lang_mon::Expression::If {
            cond_exp,
            then_block,
            else_block,
        } => {
            let old_label = state.current_label;
            let then_label = state.new_label();
            state.current_label = then_label;
            exps_to_block(then_block, state)?;
            let else_label = state.new_label();
            state.current_label = else_label;
            exps_to_block(else_block, state)?;
            let assign_label = state.new_label();
        }
    }
}

pub fn mon_to_c_atm(atm: lang_mon::Atom) -> lang_c::Atom {
    match atm {
        lang_mon::Atom::Integer(i) => lang_c::Atom::Integer(i),
        lang_mon::Atom::Variable(v) => lang_c::Atom::Variable(v),
        lang_mon::Atom::Bool(b) => lang_c::Atom::Bool(b),
    }
}

/*
pub fn explicate_exp(exp: lang_mon::Expression,prog:&mut lang_c::Program,current_label:String){
    match exp {
        lang_mon::Expression::Atm(atm) => mon_to_c_atm(atm).into(),
        lang_mon::Expression::ReadInt => lang_c::Expression::ReadInt.into(),
        lang_mon::Expression::Print(exp) => lang_c::Tail {
            stmts: vec![lang_c::Statement::Print(mon_to_c_atm(exp).into())],
            ret: lang_c::Expression::Unit.into(),
        },
        lang_mon::Expression::UnaryOp { arg, op } => {
            lang_c::Expression::un(mon_to_c_atm(arg), op).into()
        }
        lang_mon::Expression::BinaryOp { fst, op, snd } => {
            lang_c::Expression::bin(mon_to_c_atm(fst), op, mon_to_c_atm(snd)).into()
        }
        // let x = if ... { ... 3; } else { ... 5; }
        // ....
        // if ... { ... let x = 3; goto ....} else { .. let x = 5; goto .... }
        lang_mon::Expression::LetIn { var, bound } => {
            let bound_explicated = explicate_exp(*bound);
            match bound_explicated.ret {
                lang_c::TailEnd::Return(exp) => {
                    let assign = lang_c::Statement::assign(&var, exp);
                    let mut new_statements = bound_explicated.stmts;
                    new_statements.push(assign);
                    lang_c::Tail {
                        stmts: new_statements,
                        ret: lang_c::Atom::Variable(var).into(),
                    }
                }
                lang_c::TailEnd::Goto(label) => todo!(),
                lang_c::TailEnd::If {
                    left,
                    cmp,
                    right,
                    then_label,
                    else_label,
                } => todo!(),
            }
        }
        lang_mon::Expression::Cmp { left, cmp, right } => lang_c::Expression::Cmp {
            left: mon_to_c_atm(left),
            cmp,
            right: mon_to_c_atm(right),
        }
        .into(),
        lang_mon::Expression::If {
            cond_exp,
            then_block,
            else_block,
        } => todo!(),
    }
}

}*/

#[cfg(test)]
mod explicate_control_tests {
    use super::explicate_control;
    use syntax::{BinaryOperation, lang_c, lang_mon};

    #[test]
    fn explicate_let() {
        let result = explicate_control(lang_mon::Program::new(vec![
            lang_mon::Expression::let_in("x0", lang_mon::Atom::Integer(20).into()),
            lang_mon::Expression::let_in("x1", lang_mon::Atom::Integer(22).into()),
            lang_mon::Expression::let_in(
                "y",
                lang_mon::Expression::bin(
                    lang_mon::Atom::Variable("x0".to_owned()).into(),
                    BinaryOperation::Add,
                    lang_mon::Atom::Variable("x1".to_owned()).into(),
                ),
            ),
            lang_mon::Atom::Variable("y".to_owned()).into(),
        ]));
        let mut expected = lang_c::Program::new();
        expected.add_block(
            "start",
            lang_c::Tail {
                stmts: vec![
                    lang_c::Statement::Assign {
                        var: "x0".to_owned(),
                        bound: lang_c::Atom::Integer(20).into(),
                    },
                    lang_c::Statement::Assign {
                        var: "x1".to_owned(),
                        bound: lang_c::Atom::Integer(22).into(),
                    },
                    lang_c::Statement::Assign {
                        var: "y".to_owned(),
                        bound: lang_c::Expression::bin(
                            lang_c::Atom::Variable("x0".to_owned()),
                            BinaryOperation::Add,
                            lang_c::Atom::Variable("x1".to_owned()),
                        ),
                    },
                ],
                ret: lang_c::Atom::Variable("y".to_owned()).into(),
            },
        );
        assert_eq!(result, expected)
    }
}
