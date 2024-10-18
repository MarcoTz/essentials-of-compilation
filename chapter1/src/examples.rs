use super::syntax::{BinOp, Exp, Module, Stmt, UnaryOp};

pub fn example_add() -> Module {
    vec![Exp::BinOp {
        exp1: Box::new(1.into()),
        op: BinOp::Add,
        exp2: Box::new(2.into()),
    }
    .into()]
}

pub fn example_zero() -> Module {
    vec![Exp::BinOp {
        exp1: Box::new(4.into()),
        op: BinOp::Add,
        exp2: Box::new(Exp::UnaryOp {
            exp: Box::new(4.into()),
            op: UnaryOp::Neg,
        }),
    }
    .into()]
}

pub fn example_print() -> Module {
    vec![Stmt::Print(Exp::BinOp {
        exp1: Box::new(3.into()),
        op: BinOp::Sub,
        exp2: Box::new(2.into()),
    })]
}

pub fn example_multiple() -> Module {
    vec![
        Exp::Constant(1).into(),
        Exp::Constant(2).into(),
        Exp::Constant(3).into(),
    ]
}

#[cfg(test)]
mod example_tests {
    use super::{example_add, example_multiple, example_print, example_zero};
    use crate::eval::interp_lint;

    #[test]
    fn eval_add() {
        let result = interp_lint(example_add());
        let expected = vec![3];
        assert_eq!(result, expected)
    }

    #[test]
    fn eval_zero() {
        let result = interp_lint(example_zero());
        let expected = vec![0];
        assert_eq!(result, expected)
    }

    #[test]
    fn eval_print() {
        let result = interp_lint(example_print());
        let expected = vec![];
        assert_eq!(result, expected)
    }

    #[test]
    fn eval_multiple() {
        let result = interp_lint(example_multiple());
        let expected = vec![1, 2, 3];
        assert_eq!(result, expected)
    }
}
