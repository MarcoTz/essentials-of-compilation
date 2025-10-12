use std::collections::HashMap;
use syntax::{
    BinaryOperation, UnaryOperation,
    lang::{Block, Expression, Program, Statement, Type},
};

mod errors;
pub use errors::Error;

pub fn typecheck(prog: &Program) -> Result<(), Error> {
    let mut var_types = HashMap::new();
    check_block(&prog.main, &mut var_types)?;
    Ok(())
}

fn check_block(
    block: &Block,
    var_types: &mut HashMap<String, Type>,
) -> Result<Option<Type>, Error> {
    let mut ret_ty = None;
    for stmt in block.stmts.iter() {
        ret_ty = check_stmt(stmt, var_types)?;
    }
    Ok(ret_ty)
}

fn check_stmt(
    stmt: &Statement,
    var_types: &mut HashMap<String, Type>,
) -> Result<Option<Type>, Error> {
    match stmt {
        Statement::Return(exp) => {
            let exp_ty = check_exp(exp, var_types)?;
            Ok(Some(exp_ty))
        }
        Statement::Print(exp) => {
            let exp_ty = check_exp(exp, var_types)?;
            if exp_ty == Type::Integer {
                Ok(None)
            } else {
                Err(Error::mismatch(exp_ty, Type::Integer))
            }
        }
        Statement::LetBinding { var, bound } => {
            let bound_ty = check_exp(bound, var_types)?;
            var_types.insert(var.clone(), bound_ty);
            Ok(None)
        }
        Statement::If {
            cond_exp,
            then_block,
            else_block,
        } => {
            let cond_ty = check_exp(cond_exp, var_types)?;
            if cond_ty != Type::Bool {
                return Err(Error::mismatch(cond_ty, Type::Bool));
            }
            let then_ty = check_block(then_block, &mut var_types.clone())?;
            let else_ty = check_block(else_block, var_types)?;
            match (then_ty, else_ty) {
                (Some(ty1), Some(ty2)) if ty1 == ty2 => Ok(Some(ty1)),
                (None, None) => Ok(None),
                (ty1, ty2) => Err(Error::mismatch(
                    ty1.unwrap_or(Type::Unit),
                    ty2.unwrap_or(Type::Unit),
                )),
            }
        }
    }
}

fn check_exp(exp: &Expression, var_types: &mut HashMap<String, Type>) -> Result<Type, Error> {
    match exp {
        Expression::Literal(_) => Ok(Type::Integer),
        Expression::Bool(_) => Ok(Type::Bool),
        Expression::Variable(v) => var_types.get(v).cloned().ok_or(Error::FreeVar(v.clone())),
        Expression::ReadInt => Ok(Type::Integer),
        Expression::BinOp { fst, op, snd } => {
            let fst_ty = check_exp(fst, var_types)?;
            let snd_ty = check_exp(snd, var_types)?;
            if fst_ty != snd_ty {
                return Err(Error::mismatch(fst_ty, snd_ty));
            }
            match op {
                BinaryOperation::Add | BinaryOperation::Sub => {
                    if fst_ty == Type::Integer {
                        Ok(Type::Integer)
                    } else {
                        Err(Error::mismatch(fst_ty, Type::Integer))
                    }
                }
                BinaryOperation::And | BinaryOperation::Or => {
                    if fst_ty == Type::Bool {
                        Ok(Type::Bool)
                    } else {
                        Err(Error::mismatch(fst_ty, Type::Bool))
                    }
                }
            }
        }
        Expression::UnOp { arg, op } => {
            let arg_ty = check_exp(arg, var_types)?;
            match op {
                UnaryOperation::Neg => {
                    if arg_ty == Type::Integer {
                        Ok(Type::Integer)
                    } else {
                        Err(Error::mismatch(arg_ty, Type::Integer))
                    }
                }
                UnaryOperation::Not => {
                    if arg_ty == Type::Bool {
                        Ok(Type::Bool)
                    } else {
                        Err(Error::mismatch(arg_ty, Type::Bool))
                    }
                }
            }
        }
        Expression::Cmp { left, right, .. } => {
            let left_ty = check_exp(left, var_types)?;
            let right_ty = check_exp(right, var_types)?;
            if left_ty != right_ty {
                return Err(Error::mismatch(left_ty, right_ty));
            }
            if left_ty != Type::Integer {
                return Err(Error::mismatch(left_ty, Type::Integer));
            }
            Ok(Type::Bool)
        }
    }
}
