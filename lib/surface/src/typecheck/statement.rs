use super::{Error, Typecheck};
use crate::syntax::{Statement, Type};
use std::collections::HashMap;

impl Typecheck for Statement {
    fn check(&self, var_types: &mut HashMap<String, Type>) -> Result<Type, Error> {
        match self {
            Statement::Return(exp) => exp.check(var_types),
            Statement::Print(exp) => {
                let exp_ty = exp.check(var_types)?;
                if exp_ty == Type::Integer {
                    Ok(Type::Unit)
                } else {
                    Err(Error::mismatch(exp_ty, Type::Integer))
                }
            }
            Statement::Assignment { var, bound } => {
                let bound_ty = bound.check(var_types)?;
                var_types.insert(var.clone(), bound_ty);
                Ok(Type::Unit)
            }
            Statement::SetTuple { var, index, bound } => {
                let var_ty = var_types.get(var).ok_or(Error::FreeVar(var.clone()))?;
                let tuple_tys = var_ty.clone().as_tuple()?;
                let indiced_type = tuple_tys
                    .get(*index)
                    .ok_or(Error::out_of_bounds(*index, tuple_tys.len()))?;
                let bound_ty = bound.check(var_types)?;
                if *indiced_type != bound_ty {
                    Err(Error::mismatch(bound_ty, indiced_type.clone()))
                } else {
                    Ok(Type::Unit)
                }
            }
            Statement::Set { var, bound } => {
                let bound_ty = bound.check(var_types)?;
                match var_types.get(var) {
                    None => {
                        var_types.insert(var.clone(), bound_ty.clone());
                        Ok(bound_ty)
                    }
                    Some(ty) if *ty == bound_ty => Ok(bound_ty),
                    Some(ty) => Err(Error::mismatch(bound_ty, ty.clone())),
                }
            }
            Statement::If {
                cond_exp,
                then_block,
                else_block,
            } => {
                let cond_ty = cond_exp.check(var_types)?;
                if cond_ty != Type::Bool {
                    return Err(Error::mismatch(cond_ty, Type::Bool));
                }
                let then_ty = then_block.check(&mut var_types.clone())?;
                let else_ty = else_block.check(var_types)?;
                if then_ty == else_ty {
                    Ok(then_ty)
                } else {
                    Err(Error::mismatch(then_ty, else_ty))
                }
            }
            Statement::While {
                cond_exp,
                while_block,
            } => {
                let cond_ty = cond_exp.check(var_types)?;
                if cond_ty != Type::Bool {
                    return Err(Error::mismatch(cond_ty, Type::Bool));
                }
                let old_vars = var_types.clone();
                while_block.check(var_types)?;
                *var_types = old_vars;
                Ok(Type::Unit)
            }
        }
    }
}
