use super::UsedVars;
use crate::lang_mon::{Atom, Expression, Program};
use std::collections::HashSet;

impl UsedVars for Program {
    fn used_vars(&self) -> HashSet<String> {
        let mut used = HashSet::new();
        for exp in self.exps.iter() {
            used.extend(exp.used_vars());
        }
        used
    }
}

impl UsedVars for Expression {
    fn used_vars(&self) -> HashSet<String> {
        match self {
            Expression::Atm(atm) => atm.used_vars(),
            Expression::ReadInt => HashSet::new(),
            Expression::Print(atm) => atm.used_vars(),
            Expression::UnaryOp { arg, .. } => arg.used_vars(),
            Expression::BinaryOp { fst, snd, .. } => &fst.used_vars() | &snd.used_vars(),
            Expression::Cmp { left, right, .. } => &left.used_vars() | &right.used_vars(),
            Expression::LetIn { var, bound } => {
                let mut used = bound.used_vars();
                used.insert(var.clone());
                used
            }
            Expression::If {
                cond_exp,
                then_block,
                else_block,
            } => {
                let mut used = cond_exp.used_vars();
                for then_exp in then_block.iter() {
                    used.extend(then_exp.used_vars());
                }
                for else_exp in else_block.iter() {
                    used.extend(else_exp.used_vars());
                }
                used
            }
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
