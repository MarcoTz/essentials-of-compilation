use crate::{Block, Expression, Program, Statement};
use definitions::traits::fresh_var;
use std::collections::HashMap;

pub trait Uniquify {
    fn uniquify(self, substitutions: &mut HashMap<String, String>) -> Self;
}

impl Uniquify for Program {
    fn uniquify(self, substitutions: &mut HashMap<String, String>) -> Program {
        let new_main = self.main.uniquify(substitutions);
        new_main.into()
    }
}

impl Uniquify for Block {
    fn uniquify(self, substitutions: &mut HashMap<String, String>) -> Block {
        let mut new_stmts = vec![];
        for stmt in self.stmts {
            let new_stmt = stmt.uniquify(substitutions);
            new_stmts.push(new_stmt);
        }
        Block::new(new_stmts)
    }
}

impl Uniquify for Statement {
    fn uniquify(self, substitutions: &mut HashMap<String, String>) -> Statement {
        match self {
            Statement::Return(exp) => Statement::Return(exp.uniquify(substitutions)),
            Statement::Print(exp) => Statement::Print(exp.uniquify(substitutions)),
            Statement::Assignment { var, bound } => {
                let new_bound = bound.uniquify(substitutions);
                let new_var = fresh_var(&substitutions.values().cloned().collect());
                substitutions.insert(var, new_var.clone());
                Statement::assign(&new_var, new_bound)
            }
            Statement::SetTuple { var, index, bound } => Statement::SetTuple {
                var,
                index,
                bound: bound.uniquify(substitutions),
            },
            Statement::Set { var, bound } => {
                let new_bound = bound.uniquify(substitutions);
                Statement::Set {
                    var,
                    bound: new_bound,
                }
            }
            Statement::If {
                cond_exp,
                then_block,
                else_block,
            } => {
                let new_cond = cond_exp.uniquify(substitutions);
                let new_then = then_block.uniquify(substitutions);
                let new_else = else_block.uniquify(substitutions);
                Statement::cond(new_cond, new_then, new_else)
            }
            Statement::While {
                cond_exp,
                while_block,
            } => {
                let new_cond = cond_exp.uniquify(substitutions);
                let new_while = while_block.uniquify(substitutions);
                Statement::While {
                    cond_exp: new_cond,
                    while_block: new_while,
                }
            }
        }
    }
}

impl Uniquify for Expression {
    fn uniquify(self, substitutions: &mut HashMap<String, String>) -> Expression {
        match self {
            Expression::Literal(_) => self,
            Expression::Bool(_) => self,
            Expression::Variable(ref v) => {
                if let Some(v1) = substitutions.get(v) {
                    Expression::Variable(v1.clone())
                } else {
                    self
                }
            }
            Expression::ReadInt => self,
            Expression::UnOp { arg, op } => Expression::un(arg.uniquify(substitutions), op),
            Expression::BinOp { fst, op, snd } => {
                let fst_unique = fst.uniquify(substitutions);
                let snd_unique = snd.uniquify(substitutions);
                Expression::bin(fst_unique, op, snd_unique)
            }
            Expression::Cmp { left, cmp, right } => {
                let left_unique = left.uniquify(substitutions);
                let right_unique = right.uniquify(substitutions);
                Expression::cmp(left_unique, cmp, right_unique)
            }
            Expression::Tuple { inner } => Expression::Tuple {
                inner: inner
                    .into_iter()
                    .map(|exp| exp.uniquify(substitutions))
                    .collect(),
            },
            Expression::TupleAccess { tup, index } => Expression::TupleAccess {
                tup: Box::new(tup.uniquify(substitutions)),
                index,
            },
            Expression::Reference { inner } => Expression::Reference {
                inner: Box::new(inner.uniquify(substitutions)),
            },
            Expression::Dereference { inner } => Expression::Dereference {
                inner: Box::new(inner.uniquify(substitutions)),
            },
        }
    }
}

#[cfg(test)]
mod uniquify_tests {
    use super::Uniquify;
    use crate::{Expression, Program, Statement};
    use definitions::BinaryOperation;

    #[test]
    fn uniqufy_let_let() {
        let result = Program::new(vec![
            Statement::assign("x", Expression::lit(32)),
            Statement::assign("x", Expression::lit(10)),
            Statement::Return(Expression::bin(
                Expression::var("x"),
                BinaryOperation::Add,
                Expression::var("x"),
            )),
        ])
        .uniquify(&mut Default::default());
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
        let result = Program::new(vec![
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
        ])
        .uniquify(&mut Default::default());
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
