use std::collections::HashMap;
use syntax::{
    BinaryOperation, UnaryOperation,
    lang::{Expression, Program, Type},
};

mod errors;
pub use errors::Error;

pub fn typecheck(prog: &Program) -> Result<(), Error> {
    let mut var_types = HashMap::new();
    for exp in prog.exps.iter() {
        let _ = check_exp(exp, &mut var_types)?;
    }
    Ok(())
}

fn check_exp(exp: &Expression, var_types: &mut HashMap<String, Type>) -> Result<Type, Error> {
    match exp {
        Expression::Literal(_) => Ok(Type::Integer),
        Expression::Bool(_) => Ok(Type::Bool),
        Expression::Variable(v) => var_types.get(v).cloned().ok_or(Error::FreeVar(v.clone())),
        Expression::ReadInt => Ok(Type::Integer),
        Expression::Print(exp) => {
            let _ = check_exp(exp, var_types)?;
            Ok(Type::Unit)
        }
        Expression::LetIn { var, bound } => {
            let bound_ty = check_exp(bound, var_types)?;
            var_types.insert(var.clone(), bound_ty);
            Ok(Type::Unit)
        }
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
        Expression::If {
            cond_exp,
            then_exp,
            else_exp,
        } => {
            let cond_ty = check_exp(cond_exp, var_types)?;
            if cond_ty != Type::Bool {
                return Err(Error::mismatch(cond_ty, Type::Bool));
            }
            let mut then_vars = var_types.clone();
            let then_ty = check_exp(then_exp, &mut then_vars)?;
            let else_ty = check_exp(else_exp, var_types)?;
            if then_ty != else_ty {
                return Err(Error::mismatch(then_ty, else_ty));
            }
            Ok(then_ty)
        }
    }
}
