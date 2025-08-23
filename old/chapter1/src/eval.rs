use super::syntax::{BinOp, Exp, Program, UnaryOp};
use std::io::stdin;

fn get_input() -> i64 {
    let mut inp: String = String::default();
    println!("Please enter a number:");
    stdin().read_line(&mut inp).unwrap_or_default();
    match inp.trim_end().parse::<i64>() {
        Ok(i) => i,
        Err(_) => get_input(),
    }
}
pub fn interp_exp(e: Exp) -> i64 {
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

pub fn interp_lint(prog: Program) -> i64 {
    interp_exp(prog.exp)
}

#[cfg(test)]
mod eval_tests {
    use super::{interp_exp, interp_lint, BinOp, Exp, Program, UnaryOp};

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
    fn eval_prog() {
        let result = interp_lint(Program {
            exp: Exp::BinOp {
                exp1: Box::new(1.into()),
                op: BinOp::Add,
                exp2: Box::new(2.into()),
            },
        });
        let expected = 3;
        assert_eq!(result, expected)
    }
}
