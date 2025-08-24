use std::collections::HashSet;
use syntax::{fresh_var, lang, lang_mon};

pub fn remove_complex_operands(prog: lang::Program) -> lang_mon::Program {
    let mut used = prog.used_vars();
    lang_mon::Program::new(rco_expr(prog.exp, &mut used))
}

fn rco_expr(exp: lang::Expression, used_vars: &mut HashSet<String>) -> lang_mon::Expression {
    match exp {
        lang::Expression::Literal(i) => lang_mon::Atom::Integer(i).into(),
        lang::Expression::Variable(v) => lang_mon::Atom::Variable(v).into(),
        lang::Expression::ReadInt => lang_mon::Expression::ReadInt,
        lang::Expression::LetIn {
            var,
            bound_exp,
            in_exp,
        } => {
            let rco_bound = rco_expr(*bound_exp, used_vars);
            let rco_in = rco_expr(*in_exp, used_vars);
            lang_mon::Expression::let_in(&var, rco_bound, rco_in)
        }
        lang::Expression::BinOp { fst, op, snd } => {
            let rco_fst = rco_expr(*fst, used_vars);
            let mut arg_vars = vec![];
            let fst_atom = if let lang_mon::Expression::Atm(atm) = rco_fst {
                atm
            } else {
                let fst_var = fresh_var(used_vars);
                arg_vars.push((fst_var.clone(), rco_fst));
                lang_mon::Atom::Variable(fst_var)
            };
            let rco_snd = rco_expr(*snd, used_vars);
            let snd_atom = if let lang_mon::Expression::Atm(atm) = rco_snd {
                atm
            } else {
                let snd_var = fresh_var(used_vars);
                arg_vars.push((snd_var.clone(), rco_snd));
                lang_mon::Atom::Variable(snd_var)
            };
            let bin_op = lang_mon::Expression::bin(fst_atom, op, snd_atom);
            let mut exp = bin_op;
            for (var, var_exp) in arg_vars {
                exp = lang_mon::Expression::let_in(&var, var_exp, exp);
            }
            exp
        }
        lang::Expression::UnOp { arg, op } => {
            let rco_arg = rco_expr(*arg, used_vars);
            let (arg_atom, assgn) = if let lang_mon::Expression::Atm(atm) = rco_arg {
                (atm, None)
            } else {
                let arg_var = fresh_var(used_vars);
                (
                    lang_mon::Atom::Variable(arg_var.clone()),
                    Some((arg_var, rco_arg)),
                )
            };
            let un_op = lang_mon::Expression::un(arg_atom, op);
            if let Some((var, var_exp)) = assgn {
                lang_mon::Expression::let_in(&var, var_exp, un_op)
            } else {
                un_op
            }
        }
    }
}

#[cfg(test)]
mod remove_complex_operands_tests {
    use super::remove_complex_operands;
    use syntax::{BinaryOperation, UnaryOperation, lang, lang_mon};

    #[test]
    fn remove_sum() {
        let result = remove_complex_operands(lang::Program::new(lang::Expression::let_in(
            "x",
            lang::Expression::bin(
                lang::Expression::lit(42),
                BinaryOperation::Add,
                lang::Expression::un(lang::Expression::lit(10), UnaryOperation::Neg),
            ),
            lang::Expression::bin(
                lang::Expression::var("x"),
                BinaryOperation::Add,
                lang::Expression::lit(10),
            ),
        )));
        let expected = lang_mon::Program::new(lang_mon::Expression::let_in(
            "x",
            lang_mon::Expression::let_in(
                "x0",
                lang_mon::Expression::un(lang_mon::Atom::Integer(10), UnaryOperation::Neg),
                lang_mon::Expression::bin(
                    lang_mon::Atom::Integer(42),
                    BinaryOperation::Add,
                    lang_mon::Atom::Variable("x0".to_owned()),
                ),
            ),
            lang_mon::Expression::bin(
                lang_mon::Atom::Variable("x".to_owned()),
                BinaryOperation::Add,
                lang_mon::Atom::Integer(10),
            ),
        ));
        assert_eq!(result, expected)
    }
}
