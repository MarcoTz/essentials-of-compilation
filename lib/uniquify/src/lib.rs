use std::collections::HashSet;
use syntax::{
    fresh_var,
    lang::{Expression, Program},
};

pub fn uniquify(prog: Program) -> Program {
    Program::new(uniquify_exp(prog.exp, &mut HashSet::new()))
}

fn uniquify_exp(exp: Expression, used_vars: &mut HashSet<String>) -> Expression {
    match exp {
        Expression::Literal(_) => exp,
        Expression::Variable(_) => exp,
        Expression::InputInt => exp,
        Expression::LetIn {
            var,
            bound_exp,
            in_exp,
        } => {
            let new_var = if used_vars.contains(&var) {
                fresh_var(used_vars)
            } else {
                used_vars.insert(var.clone());
                var.clone()
            };
            let bound_unique = uniquify_exp(bound_exp.subst_var(&var, &new_var), used_vars);
            let in_unique = uniquify_exp(in_exp.subst_var(&var, &new_var), used_vars);
            Expression::let_in(&new_var, bound_unique, in_unique)
        }
        Expression::UnOp { arg, op } => Expression::un(uniquify_exp(*arg, used_vars), op),
        Expression::BinOp { fst, op, snd } => {
            let fst_unique = uniquify_exp(*fst, used_vars);
            let snd_unique = uniquify_exp(*snd, used_vars);
            Expression::bin(fst_unique, op, snd_unique)
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
        let result = uniquify(Program::new(Expression::let_in(
            "x",
            Expression::lit(32),
            Expression::bin(
                Expression::let_in("x", Expression::lit(10), Expression::var("x")),
                BinaryOperation::Add,
                Expression::var("x"),
            ),
        )));
        let expected = Program::new(Expression::let_in(
            "x",
            Expression::lit(32),
            Expression::bin(
                Expression::let_in("x0", Expression::lit(10), Expression::var("x0")),
                BinaryOperation::Add,
                Expression::var("x"),
            ),
        ));
        assert_eq!(result, expected)
    }

    #[test]
    fn uniquify_shadow() {
        let result = uniquify(Program::new(Expression::let_in(
            "x",
            Expression::let_in(
                "x",
                Expression::lit(4),
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
        )));
        let expected = Program::new(Expression::let_in(
            "x",
            Expression::let_in(
                "x0",
                Expression::lit(4),
                Expression::bin(
                    Expression::var("x0"),
                    BinaryOperation::Add,
                    Expression::lit(1),
                ),
            ),
            Expression::bin(
                Expression::var("x"),
                BinaryOperation::Add,
                Expression::lit(2),
            ),
        ));
        assert_eq!(result, expected)
    }
}
