use super::syntax::{BinOp, Exp, Module, Stmt, UnaryOp};
use std::io::stdin;

fn get_input() -> i32 {
    let mut inp: String = String::default();
    stdin().read_line(&mut inp).unwrap_or_default();
    match inp.trim_end().parse::<i32>() {
        Ok(i) => i,
        Err(_) => get_input(),
    }
}
pub fn interp_exp(e: Exp) -> i32 {
    match e {
        Exp::Constant(i) => i,
        Exp::InputInt => get_input(),
        Exp::UnaryOp { op, exp } => match op {
            UnaryOp::Neg => {
                let res = interp_exp(*exp);
                -res
            }
        },
        Exp::BinOp { op, exp1, exp2 } => {
            let i1 = interp_exp(*exp1);
            let i2 = interp_exp(*exp2);
            match op {
                BinOp::Sub => i1 - i2,
                BinOp::Add => i1 + i2,
            }
        }
    }
}

pub fn interp_stmt(st: Stmt) -> Option<i32> {
    match st {
        Stmt::Print(arg) => {
            let arg_res = interp_exp(arg);
            println!("{}", arg_res);
            None
        }
        Stmt::Exp(e) => Some(interp_exp(e)),
    }
}

pub fn interp_lint(m: Module) -> Vec<i32> {
    let mut results = vec![];
    for stmt in m.into_iter() {
        let res = interp_stmt(stmt);
        if let Some(i) = res {
            results.push(i)
        }
    }
    results
}

#[cfg(test)]
mod eval_tests {
    use super::{interp_exp, interp_lint, interp_stmt, BinOp, Exp, Stmt, UnaryOp};

    #[test]
    fn eval_const() {
        let result = interp_exp(1.into());
        let expected = 1;
        assert_eq!(result, expected)
    }
    #[test]
    fn eval_unary() {
        let result = interp_exp(Exp::UnaryOp {
            exp: Box::new(1.into()),
            op: UnaryOp::Neg,
        });
        let expected = -1;
        assert_eq!(result, expected)
    }

    #[test]
    fn eval_bin() {
        let result = interp_exp(Exp::BinOp {
            exp1: Box::new(2.into()),
            op: BinOp::Add,
            exp2: Box::new(3.into()),
        });
        let expected = 5;
        assert_eq!(result, expected)
    }

    #[test]
    fn eval_print() {
        let result = interp_stmt(Stmt::Print(1.into()));
        let expected = None;
        assert_eq!(result, expected)
    }

    #[test]
    fn eval_exp() {
        let result = interp_stmt(Stmt::Exp(2.into()));
        let expected = Some(2);
        assert_eq!(result, expected)
    }

    #[test]
    fn eval_mod() {
        let result = interp_lint(vec![
            Stmt::Print(1.into()),
            Stmt::Exp(2.into()),
            Stmt::Exp(3.into()),
        ]);
        let expected = vec![2, 3];
        assert_eq!(result, expected)
    }
}
