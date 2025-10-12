use super::UsedVars;
use crate::lang::{Block, Expression, Program, Statement};
use std::collections::HashSet;

impl UsedVars for Program {
    fn used_vars(&self) -> HashSet<String> {
        self.main.used_vars()
    }
}

impl UsedVars for Block {
    fn used_vars(&self) -> HashSet<String> {
        let mut used = HashSet::new();
        for stmt in self.stmts.iter() {
            used.extend(stmt.used_vars())
        }
        used
    }
}

impl UsedVars for Statement {
    fn used_vars(&self) -> HashSet<String> {
        match self {
            Statement::Return(exp) => exp.used_vars(),
            Statement::Print(exp) => exp.used_vars(),
            Statement::LetBinding { var, bound } => {
                &HashSet::from([var.clone()]) | &bound.used_vars()
            }
            Statement::If {
                cond_exp,
                then_block,
                else_block,
            } => &(&cond_exp.used_vars() | &then_block.used_vars()) | &else_block.used_vars(),
        }
    }
}

impl UsedVars for Expression {
    fn used_vars(&self) -> HashSet<String> {
        match self {
            Expression::Literal(_) => HashSet::new(),
            Expression::Bool(_) => HashSet::new(),
            Expression::Variable(v) => HashSet::from([v.clone()]),
            Expression::ReadInt => HashSet::new(),
            Expression::BinOp { fst, snd, .. } => &fst.used_vars() | &snd.used_vars(),
            Expression::UnOp { arg, .. } => arg.used_vars(),
            Expression::Cmp { left, right, .. } => &left.used_vars() | &right.used_vars(),
        }
    }
}
