use std::collections::HashSet;
use syntax::{
    lang, lang_mon,
    traits::{UsedVars, fresh_var},
};

pub fn remove_complex_operands(prog: lang::Program) -> lang_mon::Program {
    let mut used = prog.used_vars();
    let new_main = rco_block(prog.main, &mut used);
    new_main.into()
}

fn rco_block(block: lang::Block, used_vars: &mut HashSet<String>) -> lang_mon::Block {
    let mut removed = vec![];
    for stmt in block.stmts {
        removed.extend(rco_stmt(stmt, used_vars));
    }
    lang_mon::Block::new(removed)
}

fn rco_stmt(stmt: lang::Statement, used_vars: &mut HashSet<String>) -> Vec<lang_mon::Statement> {
    match stmt {
        lang::Statement::Return(exp) => {
            let (mut stmts, exp) = rco_expr(exp, used_vars);
            let (assign, atm) = exp_to_atm(exp, used_vars);
            stmts.push(assign);
            stmts.push(lang_mon::Statement::Return(atm));
            stmts
        }
        lang::Statement::Print(exp) => {
            let (mut stmts, exp) = rco_expr(exp, used_vars);
            let (assign, atm) = exp_to_atm(exp, used_vars);
            stmts.push(assign);
            stmts.push(lang_mon::Statement::Print(atm));
            stmts
        }
        lang::Statement::Assignment { var, bound } => {
            let (mut stmts, new_bind) = rco_expr(bound, used_vars);
            stmts.push(lang_mon::Statement::assign(&var, new_bind));
            stmts
        }
        lang::Statement::If {
            cond_exp,
            then_block,
            else_block,
        } => {
            let (mut stmts, new_cond) = rco_expr(cond_exp, used_vars);
            let (assign, cond_atm) = exp_to_atm(new_cond, used_vars);
            stmts.push(assign);
            let new_then = rco_block(then_block, used_vars);
            let new_else = rco_block(else_block, used_vars);
            stmts.push(lang_mon::Statement::cond(cond_atm, new_then, new_else));
            stmts
        }
    }
}

fn rco_expr(
    exp: lang::Expression,
    used_vars: &mut HashSet<String>,
) -> (Vec<lang_mon::Statement>, lang_mon::Expression) {
    match exp {
        lang::Expression::Literal(i) => (vec![], lang_mon::Atom::Integer(i).into()),
        lang::Expression::Bool(b) => (vec![], lang_mon::Atom::Bool(b).into()),
        lang::Expression::Variable(v) => (vec![], lang_mon::Atom::Variable(v).into()),
        lang::Expression::ReadInt => (vec![], lang_mon::Expression::ReadInt),

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
    }
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
    use super::remove_complex_operands;
    use syntax::{BinaryOperation, UnaryOperation, lang, lang_mon};

    #[test]
    fn remove_sum() {
        let result = remove_complex_operands(lang::Program::new(vec![
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
        ]));
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
