use crate::{Block, Expression, Program, Statement, Type};
use definitions::{BinaryOperation, UnaryOperation};
use std::collections::HashMap;

mod errors;
pub use errors::Error;

pub trait Typecheck {
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
                while_block.check(var_types)?;
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
        }
    }
}
