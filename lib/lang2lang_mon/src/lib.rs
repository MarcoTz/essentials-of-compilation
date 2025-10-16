use definitions::traits::fresh_var;
use lang_mon;
use std::collections::HashSet;

mod block;
mod expression;
mod program;
mod statement;

pub trait RemoveComplexOperands {
    type Target;
    fn remove_complex_operands(self, used_vars: &mut HashSet<String>) -> Self::Target;
}

fn exp_to_atm(
    exp: lang_mon::Expression,
    used_vars: &mut HashSet<String>,
) -> (lang_mon::Statement, lang_mon::Atom) {
    let new_var = fresh_var(used_vars);
    let let_exp = lang_mon::Statement::assign(&new_var, exp);
    used_vars.insert(new_var.clone());
    let atm = lang_mon::Atom::Variable(new_var);
    (let_exp, atm)
}

#[cfg(test)]
mod remove_complex_operands_tests {
    use super::RemoveComplexOperands;
    use definitions::{BinaryOperation, UnaryOperation};
    use lang;
    use lang_mon;

    #[test]
    fn remove_sum() {
        let result = lang::Program::new(vec![
            lang::Statement::assign(
                "x",
                lang::Expression::bin(
                    lang::Expression::lit(42),
                    BinaryOperation::Add,
                    lang::Expression::un(lang::Expression::lit(10), UnaryOperation::Neg),
                ),
            ),
            lang::Statement::Return(lang::Expression::bin(
                lang::Expression::var("x"),
                BinaryOperation::Add,
                lang::Expression::lit(10),
            )),
        ])
        .remove_complex_operands(&mut Default::default());
        let expected = lang_mon::Program::new(vec![
            lang_mon::Statement::assign(
                "x0",
                lang_mon::Expression::un(lang_mon::Atom::Integer(10), UnaryOperation::Neg),
            ),
            lang_mon::Statement::assign(
                "x",
                lang_mon::Expression::bin(
                    lang_mon::Atom::Integer(42),
                    BinaryOperation::Add,
                    lang_mon::Atom::Variable("x0".to_owned()),
                ),
            ),
            lang_mon::Statement::assign(
                "x1",
                lang_mon::Expression::bin(
                    lang_mon::Atom::Variable("x".to_owned()),
                    BinaryOperation::Add,
                    lang_mon::Atom::Integer(10),
                ),
            ),
            lang_mon::Statement::Return(lang_mon::Atom::Variable("x1".to_owned())),
        ]);
        assert_eq!(result, expected)
    }
}
