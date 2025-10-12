use super::SubstVar;
use crate::lang::{Block, Expression, Statement};

impl SubstVar for Block {
    fn subst_var(self, old: &str, new: &str) -> Block {
        let mut stmts_subst = Vec::with_capacity(self.stmts.len());
        for stmt in self.stmts {
            stmts_subst.push(stmt.subst_var(old, new));
        }
        Block { stmts: stmts_subst }
    }
}

impl SubstVar for Expression {
    fn subst_var(self, old: &str, new: &str) -> Expression {
        match self {
            Expression::Literal(_) => self,
            Expression::Bool(_) => self,
            Expression::Variable(ref v) => {
                if v == old {
                    Expression::var(new)
                } else {
                    self
                }
            }
            Expression::ReadInt => self,
            Expression::BinOp { fst, op, snd } => {
                let fst_subst = fst.subst_var(old, new);
                let snd_subst = snd.subst_var(old, new);
                Expression::bin(fst_subst, op, snd_subst)
            }
            Expression::UnOp { arg, op } => Expression::un(arg.subst_var(old, new), op),
            Expression::Cmp { left, cmp, right } => {
                Expression::cmp(left.subst_var(old, new), cmp, right.subst_var(old, new))
            }
        }
    }
}

impl SubstVar for Statement {
    fn subst_var(self, old: &str, new: &str) -> Statement {
        match self {
            Statement::Return(exp) => Statement::Return(exp.subst_var(old, new)),
            Statement::Print(exp) => Statement::Print(exp.subst_var(old, new)),
            Statement::Assignment { var, bound } => {
                let bound_subst = bound.subst_var(old, new);
                Statement::Assignment {
                    var,
                    bound: bound_subst,
                }
            }
            Statement::If {
                cond_exp,
                then_block,
                else_block,
            } => Statement::If {
                cond_exp: cond_exp.subst_var(old, new),
                then_block: then_block.subst_var(old, new),
                else_block: else_block.subst_var(old, new),
            },
        }
    }
}
