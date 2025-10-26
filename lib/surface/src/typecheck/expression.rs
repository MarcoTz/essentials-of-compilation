use super::{Error, Typecheck};
use crate::syntax::{Expression, Type};
use definitions::{BinaryOperation, UnaryOperation};
use std::collections::HashMap;

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
