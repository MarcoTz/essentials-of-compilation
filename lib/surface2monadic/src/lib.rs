use definitions::traits::fresh_var;
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
    exp: monadic::Expression,
    used_vars: &mut HashSet<String>,
    mutable: bool,
) -> (monadic::Statement, monadic::Atom) {
    let new_var = fresh_var(used_vars);
    let let_exp = if mutable {
        monadic::Statement::set(&new_var, exp)
    } else {
        monadic::Statement::assign(&new_var, exp)
    };
    used_vars.insert(new_var.clone());
    let atm = monadic::Atom::Variable(new_var);
    (let_exp, atm)
}

#[cfg(test)]
mod remove_complex_operands_tests {
    use super::RemoveComplexOperands;
    use definitions::{BinaryOperation, UnaryOperation};

    #[test]
    fn remove_sum() {
        let result = surface::Program::new(vec![
            surface::Statement::assign(
                "x",
                surface::Expression::bin(
                    surface::Expression::lit(42),
                    BinaryOperation::Add,
                    surface::Expression::un(surface::Expression::lit(10), UnaryOperation::Neg),
                ),
            ),
            surface::Statement::Return(surface::Expression::bin(
                surface::Expression::var("x"),
                BinaryOperation::Add,
                surface::Expression::lit(10),
            )),
        ])
        .remove_complex_operands(&mut Default::default());
        let expected = monadic::Program::new(vec![
            monadic::Statement::assign(
                "x0",
                monadic::Expression::un(monadic::Atom::Integer(10), UnaryOperation::Neg),
            ),
            monadic::Statement::assign(
                "x",
                monadic::Expression::bin(
                    monadic::Atom::Integer(42),
                    BinaryOperation::Add,
                    monadic::Atom::Variable("x0".to_owned()),
                ),
            ),
            monadic::Statement::assign(
                "x1",
                monadic::Expression::bin(
                    monadic::Atom::Variable("x".to_owned()),
                    BinaryOperation::Add,
                    monadic::Atom::Integer(10),
                ),
            ),
            monadic::Statement::Return(monadic::Atom::Variable("x1".to_owned())),
        ]);
        assert_eq!(result, expected)
    }
}
