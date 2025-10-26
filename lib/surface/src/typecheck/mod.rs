use crate::{Block, Expression, Program, Statement, Type};
use definitions::{BinaryOperation, UnaryOperation};
use std::collections::HashMap;

mod errors;
pub use errors::Error;

/// trait used for type checking the language
/// implemented for all kinds of expressions (expression,statement,etc)
pub trait Typecheck {
    /// check self returning the type if successful
    /// var_types contains types of variables in scope
    fn check(&self, var_types: &mut HashMap<String, Type>) -> Result<Type, Error>;
}

impl Typecheck for Program {
    fn check(&self, var_types: &mut HashMap<String, Type>) -> Result<Type, Error> {
        self.main.check(var_types)?;
        Ok(Type::Unit)
    }
}

impl Typecheck for Block {
    fn check(&self, var_types: &mut HashMap<String, Type>) -> Result<Type, Error> {
        let mut ret_ty = Type::Unit;
        for stmt in self.stmts.iter() {
            ret_ty = stmt.check(var_types)?;
        }
        Ok(ret_ty)
    }
}

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

impl Typecheck for Expression {
    fn check(&self, var_types: &mut HashMap<String, Type>) -> Result<Type, Error> {
        match self {
            Expression::Literal(_) => Ok(Type::Integer),
            Expression::Bool(_) => Ok(Type::Bool),
            Expression::Variable(v) => var_types.get(v).cloned().ok_or(Error::FreeVar(v.clone())),
            Expression::ReadInt => Ok(Type::Integer),
            Expression::BinOp { fst, op, snd } => {
                let fst_ty = fst.check(var_types)?;
                let snd_ty = snd.check(var_types)?;
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
                let arg_ty = arg.check(var_types)?;
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
                let left_ty = left.check(var_types)?;
                let right_ty = right.check(var_types)?;
                if left_ty != right_ty {
                    return Err(Error::mismatch(left_ty, right_ty));
                }
                if left_ty != Type::Integer {
                    return Err(Error::mismatch(left_ty, Type::Integer));
                }
                Ok(Type::Bool)
            }
            Expression::Tuple { inner } => {
                let mut inner_tys = vec![];
                for exp in inner.iter() {
                    inner_tys.push(exp.check(var_types)?);
                }
                Ok(Type::Tuple(inner_tys))
            }
            Expression::TupleAccess { tup, index } => {
                let tup_ty = tup.check(var_types)?;
                let inner_tys = tup_ty.as_tuple()?;
                let indiced = inner_tys
                    .get(*index)
                    .ok_or(Error::out_of_bounds(*index, inner_tys.len()))?;
                Ok(indiced.clone())
            }
            Expression::Reference { inner } => {
                let inner_ty = inner.check(var_types)?;
                Ok(Type::Reference(Box::new(inner_ty)))
            }
            Expression::Dereference { inner } => {
                let ref_ty = inner.check(var_types)?;
                let ref_inner = ref_ty.as_ref()?;
                Ok(ref_inner)
            }
        }
    }
}
