use super::UsedVars;
use crate::lang_mon::{Atom, Block, Expression, Program, Statement};
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
            used.extend(stmt.used_vars());
        }
        used
    }
}

impl UsedVars for Statement {
    fn used_vars(&self) -> HashSet<String> {
        match self {
            Statement::Return(atm) => atm.used_vars(),
            Statement::Print(atm) => atm.used_vars(),
            Statement::LetBinding { var, bound } => {
                let mut used = bound.used_vars();
                used.insert(var.clone());
                used
            }
            Statement::If {
                cond_exp,
                then_block,
                else_block,
            } => &cond_exp.used_vars() | &(&then_block.used_vars() | &else_block.used_vars()),
        }
    }
}

impl UsedVars for Expression {
    fn used_vars(&self) -> HashSet<String> {
        match self {
            Expression::Atm(atm) => atm.used_vars(),
            Expression::ReadInt => HashSet::new(),
            Expression::UnaryOp { arg, .. } => arg.used_vars(),
            Expression::BinaryOp { fst, snd, .. } => &fst.used_vars() | &snd.used_vars(),
            Expression::Cmp { left, right, .. } => &left.used_vars() | &right.used_vars(),
        }
    }
}

impl UsedVars for Atom {
    fn used_vars(&self) -> HashSet<String> {
        match self {
            Atom::Variable(v) => HashSet::from([v.clone()]),
            _ => HashSet::new(),
        }
    }
}
