use crate::{
    l_if::{
        errors::Error,
        syntax::{types::Type, Exp},
    },
    Var,
};
use std::collections::HashMap;

pub type Env = HashMap<Var, Type>;

pub fn check(exp: &Exp, env: &mut Env) -> Result<Type, Error> {
    match exp {
        Exp::Int(_) => Ok(Type::Int),
        Exp::Bool(_) => Ok(Type::Bool),
        Exp::Var(v) => env.get(v).cloned().ok_or(Error::FreeVar(v.clone())),
        Exp::Prim { op, args } => {
            let arg_tys = args
                .iter()
                .map(|arg| check(arg, &mut env.clone()))
                .collect::<Result<Vec<Type>, Error>>()?;
            let expected_args = op.arg_tys();
            if arg_tys.iter().all(|ty| *ty == expected_args) {
                Ok(op.ret_ty())
            } else {
                Err(Error::TypeMismatch {
                    found: arg_tys
                        .iter()
                        .filter(|ty| **ty != expected_args)
                        .next()
                        .unwrap()
                        .clone(),
                    expected: expected_args,
                })
            }
        }
        Exp::Let {
            var,
            bound_exp,
            in_exp,
        } => {
            let bound_ty = check(bound_exp, &mut env.clone())?;
            env.insert(var.clone(), bound_ty);
            check(in_exp, env)
        }
        Exp::If { ifc, thenc, elsec } => {
            let if_ty = check(ifc, &mut env.clone())?;
            if if_ty != Type::Bool {
                return Err(Error::TypeMismatch {
                    found: if_ty,
                    expected: Type::Bool,
                });
            };
            let then_ty = check(thenc, &mut env.clone())?;
            let else_ty = check(elsec, env)?;
            if then_ty == else_ty {
                Ok(then_ty)
            } else {
                Err(Error::TypeMismatch {
                    found: then_ty,
                    expected: else_ty,
                })
            }
        }
    }
}
