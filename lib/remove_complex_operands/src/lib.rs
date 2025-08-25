use std::collections::HashSet;
use syntax::{fresh_var, lang, lang_mon};

pub fn remove_complex_operands(prog: lang::Program) -> lang_mon::Program {
    let mut used = prog.used_vars();
    let mut removed = vec![];
    for exp in prog.exps {
        let (exps, last) = rco_expr(exp, &mut used);
        removed.extend(exps);
        removed.push(last);
    }
    lang_mon::Program::new(removed)
}

fn rco_expr(
    exp: lang::Expression,
    used_vars: &mut HashSet<String>,
) -> (Vec<lang_mon::Expression>, lang_mon::Expression) {
    match exp {
        lang::Expression::Literal(i) => (vec![], lang_mon::Atom::Integer(i).into()),
        lang::Expression::Variable(v) => (vec![], lang_mon::Atom::Variable(v).into()),
        lang::Expression::ReadInt => (vec![], lang_mon::Expression::ReadInt),
        lang::Expression::Print(exp) => {
            let (exps, last) = rco_expr(*exp, used_vars);
            if let lang_mon::Expression::Atm(atm) = last {
                (exps, lang_mon::Expression::Print(atm.clone()))
            } else {
                let mut exps = exps;
                let fresh = fresh_var(used_vars);
                used_vars.insert(fresh.clone());
                let assignment = lang_mon::Expression::let_in(&fresh, last);
                exps.push(assignment);
                (
                    exps,
                    lang_mon::Expression::Print(lang_mon::Atom::Variable(fresh)),
                )
            }
        }
        lang::Expression::LetIn { var, bound } => {
            let (exps, last) = rco_expr(*bound, used_vars);
            (exps, lang_mon::Expression::let_in(&var, last))
        }
        lang::Expression::BinOp { fst, op, snd } => {
            let (fst_exps, fst_last) = rco_expr(*fst, used_vars);
            let (snd_exps, snd_last) = rco_expr(*snd, used_vars);
            let mut exps = vec![];
            exps.extend(fst_exps);
            let fst_atm = if let lang_mon::Expression::Atm(atm) = fst_last {
                atm
            } else {
                let fst_var = fresh_var(used_vars);
                used_vars.insert(fst_var.clone());
                exps.push(lang_mon::Expression::let_in(&fst_var, fst_last));
                lang_mon::Atom::Variable(fst_var)
            };
            exps.extend(snd_exps);
            let snd_atm = if let lang_mon::Expression::Atm(atm) = snd_last {
                atm
            } else {
                let snd_var = fresh_var(used_vars);
                used_vars.insert(snd_var.clone());
                exps.push(lang_mon::Expression::let_in(&snd_var, snd_last));
                lang_mon::Atom::Variable(snd_var)
            };
            (exps, lang_mon::Expression::bin(fst_atm, op, snd_atm))
        }
        lang::Expression::UnOp { arg, op } => {
            let (exps, last) = rco_expr(*arg, used_vars);
            if let lang_mon::Expression::Atm(atm) = last {
                (exps, lang_mon::Expression::un(atm, op))
            } else {
                let mut exps = exps;
                let arg_var = fresh_var(used_vars);
                used_vars.insert(arg_var.clone());
                exps.push(lang_mon::Expression::let_in(&arg_var, last));
                (
                    exps,
                    lang_mon::Expression::un(lang_mon::Atom::Variable(arg_var), op),
                )
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
        let result = remove_complex_operands(lang::Program::new(vec![
            lang::Expression::let_in(
                "x",
                lang::Expression::bin(
                    lang::Expression::lit(42),
                    BinaryOperation::Add,
                    lang::Expression::un(lang::Expression::lit(10), UnaryOperation::Neg),
                ),
            ),
            lang::Expression::bin(
                lang::Expression::var("x"),
                BinaryOperation::Add,
                lang::Expression::lit(10),
            ),
        ]));
        let expected = lang_mon::Program::new(vec![
            lang_mon::Expression::let_in(
                "x0",
                lang_mon::Expression::un(lang_mon::Atom::Integer(10), UnaryOperation::Neg),
            ),
            lang_mon::Expression::let_in(
                "x",
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
        ]);
        assert_eq!(result, expected)
    }
}
