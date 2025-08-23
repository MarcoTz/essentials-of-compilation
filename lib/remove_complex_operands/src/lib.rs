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
        lang::Expression::InputInt => lang_mon::Expression::InputInt,
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
            let rco_snd = rco_expr(*snd, used_vars);
            todo!()
        }
        lang::Expression::UnOp { arg, op } => todo!(),
    }
}

fn to_atm(
    exp: lang_mon::Expression,
    used_vars: &mut HashSet<String>,
    cont: lang_mon::Expression,
) -> (lang_mon::Atom, Option<lang_mon::Expression>) {
    match exp {
        lang_mon::Expression::Atm(atm) => (atm, None),
        lang_mon::Expression::LetIn {
            var,
            bound_exp,
            in_exp,
        } => todo!(),
        _ => {
            let var = fresh_var(used_vars);
            let exp = lang_mon::Expression::let_in(&var, exp, cont);
            (lang_mon::Atom::Variable(var), Some(exp))
        }
    }
}
