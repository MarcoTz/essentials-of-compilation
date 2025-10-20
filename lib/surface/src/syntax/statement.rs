use super::{Block, expression::Expression};
use definitions::{
    PRINT_CALL, RETURN_CALL,
    traits::{SubstVar, UsedVars},
};
use std::{collections::HashSet, fmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Return(Expression),
    Print(Expression),
    Assignment {
        var: String,
        bound: Expression,
    },
    Set {
        var: String,
        bound: Expression,
    },
    If {
        cond_exp: Expression,
        then_block: Block,
        else_block: Block,
    },
    While {
        cond_exp: Expression,
        while_block: Block,
    },
}

impl Statement {
    pub fn assign(var: &str, bound: Expression) -> Statement {
        Statement::Assignment {
            var: var.to_owned(),
            bound,
        }
    }

    pub fn set(var: &str, bound: Expression) -> Statement {
        Statement::Set {
            var: var.to_owned(),
            bound,
        }
    }

    pub fn cond(cond: Expression, then_block: Block, else_block: Block) -> Statement {
        Statement::If {
            cond_exp: cond,
            then_block,
            else_block,
        }
    }
}

impl UsedVars for Statement {
    fn used_vars(&self) -> HashSet<String> {
        match self {
            Statement::Return(exp) => exp.used_vars(),
            Statement::Print(exp) => exp.used_vars(),
            Statement::Assignment { var, bound } => {
                &HashSet::from([var.clone()]) | &bound.used_vars()
            }
            Statement::Set { var, bound } => &HashSet::from([var.clone()]) | &bound.used_vars(),
            Statement::If {
                cond_exp,
                then_block,
                else_block,
            } => &(&cond_exp.used_vars() | &then_block.used_vars()) | &else_block.used_vars(),
            Statement::While {
                cond_exp,
                while_block,
            } => &cond_exp.used_vars() | &while_block.used_vars(),
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
            Statement::Set { var, bound } => {
                let bound_subst = bound.subst_var(old, new);
                Statement::Set {
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
            Statement::While {
                cond_exp,
                while_block,
            } => Statement::While {
                cond_exp: cond_exp.subst_var(old, new),
                while_block: while_block.subst_var(old, new),
            },
        }
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Statement::Return(exp) => write!(f, "{RETURN_CALL}({exp});"),
            Statement::Print(exp) => write!(f, "{PRINT_CALL}({exp});"),
            Statement::Assignment { var, bound } => write!(f, "let {var} = {bound};"),
            Statement::Set { var, bound } => write!(f, "set {var} = {bound};"),
            Statement::If {
                cond_exp,
                then_block,
                else_block,
            } => write!(
                f,
                "if {cond_exp} {{\n\t{}\n}} else {{\n\t{}\n}};",
                then_block.to_string().replace("\n", "\n\t"),
                else_block.to_string().replace("\n", "\n\t"),
            ),
            Statement::While {
                cond_exp,
                while_block,
            } => write!(
                f,
                "while {cond_exp} {{\n\t{}\n}};",
                while_block.to_string().replace("\n", "\n\t")
            ),
        }
    }
}
