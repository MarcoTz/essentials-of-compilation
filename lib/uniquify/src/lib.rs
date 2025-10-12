use std::collections::HashMap;
use syntax::{
    lang::{Block, Expression, Program, Statement},
    traits::{SubstVar, fresh_var},
};

pub fn uniquify(prog: Program) -> Program {
    let mut subst = HashMap::new();
    let new_main = uniquify_block(prog.main, &mut subst);
    new_main.into()
}

fn uniquify_block(block: Block, substitutions: &mut HashMap<String, String>) -> Block {
    let mut new_stmts = vec![];
    for stmt in block.stmts {
        let new_stmt = uniquify_stmt(stmt, substitutions);
        new_stmts.push(new_stmt);
    }
    Block::new(new_stmts)
}

fn uniquify_stmt(stmt: Statement, substitutions: &mut HashMap<String, String>) -> Statement {
    match stmt {
        Statement::Return(exp) => Statement::Return(uniquify_exp(exp, substitutions)),
        Statement::Print(exp) => Statement::Print(uniquify_exp(exp, substitutions)),
        Statement::Assignment { var, bound } => {
            let new_bound = uniquify_exp(bound, substitutions);
            let new_var = fresh_var(&substitutions.values().cloned().collect());
            substitutions.insert(var, new_var.clone());
            Statement::assign(&new_var, new_bound)
        }
        Statement::If {
            cond_exp,
            then_block,
            else_block,
        } => {
            let new_cond = uniquify_exp(cond_exp, substitutions);
            let new_then = uniquify_block(then_block, substitutions);
            let new_else = uniquify_block(else_block, substitutions);
            Statement::cond(new_cond, new_then, new_else)
        }
    }
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
    }
}

#[cfg(test)]
mod uniquify_tests {
    use super::uniquify;
    use syntax::{
        BinaryOperation,
        lang::{Expression, Program, Statement},
    };

    #[test]
    fn uniqufy_let_let() {
        let result = uniquify(Program::new(vec![
            Statement::assign("x", Expression::lit(32)),
            Statement::assign("x", Expression::lit(10)),
            Statement::Return(Expression::bin(
                Expression::var("x"),
                BinaryOperation::Add,
                Expression::var("x"),
            )),
        ]));
        let expected = Program::new(vec![
            Statement::assign("x0", Expression::lit(32)),
            Statement::assign("x1", Expression::lit(10)),
            Statement::Return(Expression::bin(
                Expression::var("x1"),
                BinaryOperation::Add,
                Expression::var("x1"),
            )),
        ]);
        assert_eq!(result, expected)
    }

    #[test]
    fn uniquify_shadow() {
        let result = uniquify(Program::new(vec![
            Statement::assign("x", Expression::lit(4)),
            Statement::assign(
                "x",
                Expression::bin(
                    Expression::var("x"),
                    BinaryOperation::Add,
                    Expression::lit(1),
                ),
            ),
            Statement::Return(Expression::bin(
                Expression::var("x"),
                BinaryOperation::Add,
                Expression::lit(2),
            )),
        ]));
        let expected = Program::new(vec![
            Statement::assign("x0", Expression::lit(4)),
            Statement::assign(
                "x1",
                Expression::bin(
                    Expression::var("x0"),
                    BinaryOperation::Add,
                    Expression::lit(1),
                ),
            ),
            Statement::Return(Expression::bin(
                Expression::var("x1"),
                BinaryOperation::Add,
                Expression::lit(2),
            )),
        ]);
        assert_eq!(result, expected)
    }
}
