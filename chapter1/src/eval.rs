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
            println!("{}", arg);
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
