use super::{
    errors::Error,
    syntax::{Exp, Module, Stmt},
};
use crate::{BinOp, UnaryOp};
use std::{collections::HashMap, io::stdin};

fn get_input() -> i64 {
    let mut inp: String = String::default();
    stdin().read_line(&mut inp).unwrap_or_default();
    match inp.trim_end().parse::<i64>() {
        Ok(i) => i,
        Err(_) => get_input(),
    }
}

pub type Env = HashMap<String, i64>;
#[derive(Debug, PartialEq)]
pub enum EvalRes {
    Num { i: i64, env: Env },
    Unit(Env),
}

pub fn interp_exp(exp: Exp, env: &Env) -> Result<i64, Error> {
    match exp {
        Exp::Name(name) => env.get(&name).cloned().ok_or(Error::VarNotFound { name }),
        Exp::Constant(i) => Ok(i),
        Exp::InputInt => Ok(get_input()),
        Exp::UnaryOp { op, exp } => {
            let res = interp_exp(*exp, env)?;
            match op {
                UnaryOp::Neg => Ok(-res),
            }
        }
        Exp::BinOp { exp1, op, exp2 } => {
            let res1 = interp_exp(*exp1, env)?;
            let res2 = interp_exp(*exp2, env)?;
            match op {
                BinOp::Add => Ok(res1 + res2),
                BinOp::Sub => Ok(res1 - res2),
            }
        }
    }
}

pub fn interp_stmt(stmt: Stmt, env: &mut Env) -> Result<EvalRes, Error> {
    match stmt {
        Stmt::Assign { name, exp } => {
            let res = interp_exp(exp, env)?;
            env.insert(name, res);
            Ok(EvalRes::Unit(env.clone()))
        }
        Stmt::Print(exp) => {
            let res = interp_exp(exp, env)?;
            println!("{}", res);
            Ok(EvalRes::Unit(env.clone()))
        }
        Stmt::Exp(exp) => {
            let res = interp_exp(exp, env)?;
            Ok(EvalRes::Num {
                i: res,
                env: env.clone(),
            })
        }
    }
}

pub fn interp_lvar(m: Module) -> Result<Vec<EvalRes>, Error> {
    let mut results = vec![];
    let mut env = HashMap::new();
    for stmt in m.stmts.into_iter() {
        let res = interp_stmt(stmt, &mut env)?;
        results.push(res)
    }
    Ok(results)
}

#[cfg(test)]
mod eval_tests {
    use super::{interp_exp, interp_stmt, BinOp, EvalRes, Exp, Stmt, UnaryOp};
    use std::collections::HashMap;

    #[test]
    fn eval_name_ok() {
        let result = interp_exp(
            Exp::Name("x".to_owned()),
            &HashMap::from([("x".to_owned(), 1)]),
        )
        .unwrap();
        let expected = 1;
        assert_eq!(result, expected)
    }

    #[test]
    fn eval_name_err() {
        let result = interp_exp(Exp::Name("x".to_owned()), &HashMap::new());
        assert!(result.is_err())
    }

    #[test]
    fn eval_const() {
        let result = interp_exp(1.into(), &HashMap::new()).unwrap();
        let expected = 1;
        assert_eq!(result, expected)
    }

    #[test]
    fn eval_unary() {
        let result = interp_exp(
            Exp::UnaryOp {
                op: UnaryOp::Neg,
                exp: Box::new(2.into()),
            },
            &HashMap::new(),
        )
        .unwrap();
        let expected = -2;
        assert_eq!(result, expected)
    }

    #[test]
    fn eval_binop() {
        let result = interp_exp(
            Exp::BinOp {
                op: BinOp::Add,
                exp1: Box::new(2.into()),
                exp2: Box::new(4.into()),
            },
            &HashMap::new(),
        )
        .unwrap();
        let expected = 6;
        assert_eq!(result, expected)
    }

    #[test]
    fn eval_print() {
        let result = interp_stmt(Stmt::Print(2.into()), &mut HashMap::new()).unwrap();
        let expected = EvalRes::Unit(HashMap::new());
        assert_eq!(result, expected)
    }

    #[test]
    fn eval_assign() {
        let result = interp_stmt(
            Stmt::Assign {
                name: "x".to_owned(),
                exp: 2.into(),
            },
            &mut HashMap::new(),
        )
        .unwrap();
        let expected = EvalRes::Unit(HashMap::from([("x".to_owned(), 2)]));
        assert_eq!(result, expected)
    }

    #[test]
    fn eval_exp() {
        let result = interp_stmt(Stmt::Exp(4.into()), &mut HashMap::new()).unwrap();
        let expected = EvalRes::Num {
            i: 4,
            env: HashMap::new(),
        };
        assert_eq!(result, expected)
    }
}
