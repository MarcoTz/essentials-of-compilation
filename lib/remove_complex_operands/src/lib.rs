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
        lang::Expression::Bool(b) => (vec![], lang_mon::Atom::Bool(b).into()),
        lang::Expression::Variable(v) => (vec![], lang_mon::Atom::Variable(v).into()),
        lang::Expression::ReadInt => (vec![], lang_mon::Expression::ReadInt),
        lang::Expression::Print(exp) => {
            let (mut exps, last) = rco_expr(*exp, used_vars);
            if let lang_mon::Expression::Atm(atm) = last {
                (exps, lang_mon::Expression::Print(atm.clone()))
            } else {
                let (assignment, atm) = exp_to_atm(last, used_vars);
                exps.push(assignment);
                (exps, lang_mon::Expression::Print(atm))
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
                let (assignment, atm) = exp_to_atm(fst_last, used_vars);
                exps.push(assignment);
                atm
            };
            exps.extend(snd_exps);
            let snd_atm = if let lang_mon::Expression::Atm(atm) = snd_last {
                atm
            } else {
                let (assignment, atm) = exp_to_atm(snd_last, used_vars);
                exps.push(assignment);
                atm
            };
            (exps, lang_mon::Expression::bin(fst_atm, op, snd_atm))
        }
        lang::Expression::UnOp { arg, op } => {
            let (mut exps, last) = rco_expr(*arg, used_vars);
            if let lang_mon::Expression::Atm(atm) = last {
                (exps, lang_mon::Expression::un(atm, op))
            } else {
                let (assignment, atm) = exp_to_atm(last, used_vars);
                exps.push(assignment);
                (exps, lang_mon::Expression::un(atm, op))
            }
        }
        lang::Expression::Cmp { left, cmp, right } => {
            let (left_exps, left_last) = rco_expr(*left, used_vars);
            let (right_exps, right_last) = rco_expr(*right, used_vars);
            let mut exps = left_exps;
            let left_atm = if let lang_mon::Expression::Atm(atm) = left_last {
                atm
            } else {
                let (assignment, atm) = exp_to_atm(left_last, used_vars);
                exps.push(assignment);
                atm
            };
            exps.extend(right_exps);
            let right_atm = if let lang_mon::Expression::Atm(atm) = right_last {
                atm
            } else {
                let (assignment, atm) = exp_to_atm(right_last, used_vars);
                exps.push(assignment);
                atm
            };
            (exps, lang_mon::Expression::cmp(left_atm, cmp, right_atm))
        }
        lang::Expression::If {
            cond_exp,
            then_block,
            else_block,
        } => {
            let (cond_exps, cond_last) = rco_expr(*cond_exp, used_vars);
            let mut then_exps = vec![];
            for then_exp in then_block {
                let (exps, last) = rco_expr(then_exp, used_vars);
                then_exps.extend(exps);
                then_exps.push(last);
            }

            let mut else_exps = vec![];
            for else_exp in else_block {
                let (exps, last) = rco_expr(else_exp, used_vars);
                else_exps.extend(exps);
                else_exps.push(last);
            }
            (
                cond_exps,
                lang_mon::Expression::if_exp(cond_last, then_exps, else_exps),
            )
        }
    }
}

fn exp_to_atm(
    exp: lang_mon::Expression,
    used_vars: &mut HashSet<String>,
) -> (lang_mon::Expression, lang_mon::Atom) {
    let new_var = fresh_var(&used_vars);
    let let_exp = lang_mon::Expression::let_in(&new_var, exp);
    used_vars.insert(new_var.clone());
    let atm = lang_mon::Atom::Variable(new_var);
    (let_exp, atm)
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
