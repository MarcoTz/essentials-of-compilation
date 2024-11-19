use super::syntax::{BinOp, Exp, Program, UnaryOp};
use std::{collections::HashMap, io::stdin};

pub mod errors;
use errors::Error;

fn get_input() -> i64 {
    let mut inp: String = String::default();
    stdin().read_line(&mut inp).unwrap_or_default();
    match inp.trim_end().parse::<i64>() {
        Ok(i) => i,
        Err(_) => get_input(),
    }
}

pub type Env = HashMap<String, i64>;

pub fn interp_exp(exp: Exp, env: &mut Env) -> Result<i64, Error> {
    match exp {
        Exp::Name(name) => env.get(&name).cloned().ok_or(Error::VarNotFound { name }),
        Exp::Assign {
            name,
            bound_term,
            in_term,
        } => {
            let bound_evaled = interp_exp(*bound_term, env)?;
            env.insert(name, bound_evaled);
            interp_exp(*in_term, env)
        }
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

pub fn interp_lvar(m: Program) -> Result<i64, Error> {
    let mut env = HashMap::new();
    interp_exp(m.exp, &mut env)
}

#[cfg(test)]
mod eval_tests {
    use super::{interp_exp, BinOp, Exp, UnaryOp};
    use std::collections::HashMap;

    #[test]
    fn eval_name_ok() {
        let result = interp_exp(
            Exp::Name("x".to_owned()),
            &mut HashMap::from([("x".to_owned(), 1)]),
        )
        .unwrap();
        let expected = 1;
        assert_eq!(result, expected)
    }

    #[test]
    fn eval_name_err() {
        let result = interp_exp(Exp::Name("x".to_owned()), &mut HashMap::new());
        assert!(result.is_err())
    }

    #[test]
    fn eval_const() {
        let result = interp_exp(1.into(), &mut HashMap::new()).unwrap();
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
            &mut HashMap::new(),
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
            &mut HashMap::new(),
        )
        .unwrap();
        let expected = 6;
        assert_eq!(result, expected)
    }

    #[test]
    fn eval_assign() {
        let result = interp_exp(
            Exp::Assign {
                name: "x".to_owned(),
                bound_term: Box::new(2.into()),
                in_term: Box::new("x".to_owned().into()),
            },
            &mut HashMap::new(),
        )
        .unwrap();
        let expected = 2;
        assert_eq!(result, expected)
    }
}
