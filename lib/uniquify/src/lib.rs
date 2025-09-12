use std::collections::HashMap;
use syntax::{
    fresh_var,
    lang::{Expression, Program},
};

pub fn uniquify(prog: Program) -> Program {
    let mut exps = vec![];
    let mut subst = HashMap::new();
    for exp in prog.exps {
        exps.push(uniquify_exp(exp, &mut subst));
    }
    Program::new(exps)
}

fn uniquify_exp(exp: Expression, substitutions: &mut HashMap<String, String>) -> Expression {
    match exp {
        Expression::Literal(_) => exp,
        Expression::Bool(_) => exp,
        Expression::Variable(ref v) => {
            if let Some(v1) = substitutions.get(v) {
                Expression::Variable(v1.clone())
            } else {
                exp
            }
        }
        Expression::ReadInt => exp,
        Expression::Print(exp) => {
            let exp_unique = uniquify_exp(*exp, substitutions);
            Expression::Print(Box::new(exp_unique))
        }
        Expression::LetIn { var, bound } => {
            let bound_unique = uniquify_exp(*bound, substitutions);
            let fresh = fresh_var(&substitutions.values().cloned().collect());
            substitutions.insert(var, fresh.clone());
            Expression::let_in(&fresh, bound_unique)
        }
        Expression::UnOp { arg, op } => Expression::un(uniquify_exp(*arg, substitutions), op),
        Expression::BinOp { fst, op, snd } => {
            let fst_unique = uniquify_exp(*fst, substitutions);
            let snd_unique = uniquify_exp(*snd, substitutions);
            Expression::bin(fst_unique, op, snd_unique)
        }
        Expression::Cmp { left, cmp, right } => {
            let left_unique = uniquify_exp(*left, substitutions);
            let right_unique = uniquify_exp(*right, substitutions);
            Expression::cmp(left_unique, cmp, right_unique)
        }
        Expression::If {
            cond_exp,
            then_block,
            else_block,
        } => {
            let cond_unique = uniquify_exp(*cond_exp, substitutions);
            let mut then_unique = vec![];
            for then_exp in then_block {
                then_unique.push(uniquify_exp(then_exp, substitutions));
            }
            let mut else_unique = vec![];
            for else_exp in else_block {
                else_unique.push(uniquify_exp(else_exp, substitutions));
            }
            Expression::if_exp(cond_unique, then_unique, else_unique)
        }
    }
}

#[cfg(test)]
mod uniquify_tests {
    use super::uniquify;
    use syntax::{
        BinaryOperation,
        lang::{Expression, Program},
    };

    #[test]
    fn uniqufy_let_let() {
        let result = uniquify(Program::new(vec![
            Expression::let_in("x", Expression::lit(32)),
            Expression::let_in("x", Expression::lit(10)),
            Expression::bin(
                Expression::var("x"),
                BinaryOperation::Add,
                Expression::var("x"),
            ),
        ]));
        let expected = Program::new(vec![
            Expression::let_in("x0", Expression::lit(32)),
            Expression::let_in("x1", Expression::lit(10)),
            Expression::bin(
                Expression::var("x1"),
                BinaryOperation::Add,
                Expression::var("x1"),
            ),
        ]);
        assert_eq!(result, expected)
    }

    #[test]
    fn uniquify_shadow() {
        let result = uniquify(Program::new(vec![
            Expression::let_in("x", Expression::lit(4)),
            Expression::let_in(
                "x",
                Expression::bin(
                    Expression::var("x"),
                    BinaryOperation::Add,
                    Expression::lit(1),
                ),
            ),
            Expression::bin(
                Expression::var("x"),
                BinaryOperation::Add,
                Expression::lit(2),
            ),
        ]));
        let expected = Program::new(vec![
            Expression::let_in("x0", Expression::lit(4)),
            Expression::let_in(
                "x1",
                Expression::bin(
                    Expression::var("x0"),
                    BinaryOperation::Add,
                    Expression::lit(1),
                ),
            ),
            Expression::bin(
                Expression::var("x1"),
                BinaryOperation::Add,
                Expression::lit(2),
            ),
        ]);
        assert_eq!(result, expected)
    }
}
