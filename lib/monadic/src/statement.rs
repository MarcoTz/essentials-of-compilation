use super::{Atom, Block, Expression};
use definitions::{PRINT_CALL, RETURN_CALL, traits::UsedVars};
use std::{collections::HashSet, fmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Return(Atom),
    Print(Atom),
    Assign {
        var: String,
        bound: Expression,
    },
    If {
        cond: Atom,
        then_block: Block,
        else_block: Block,
    },
    While {
        cond: Atom,
        while_block: Block,
    },
}

impl Statement {
    pub fn assign(var: &str, bound_exp: Expression) -> Statement {
        Statement::Assign {
            var: var.to_owned(),
            bound: bound_exp,
        }
    }

    pub fn cond(cond_exp: Atom, then_block: Block, else_block: Block) -> Statement {
        Statement::If {
            cond: cond_exp,
            then_block,
            else_block,
        }
    }
}

impl UsedVars for Statement {
    fn used_vars(&self) -> HashSet<String> {
        match self {
            Statement::Return(atm) => atm.used_vars(),
            Statement::Print(atm) => atm.used_vars(),
            Statement::Assign { var, bound } => {
                let mut used = bound.used_vars();
                used.insert(var.clone());
                used
            }
            Statement::If {
                cond: cond_exp,
                then_block,
                else_block,
            } => &cond_exp.used_vars() | &(&then_block.used_vars() | &else_block.used_vars()),
            Statement::While { cond, while_block } => &cond.used_vars() | &while_block.used_vars(),
        }
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Statement::Return(atm) => write!(f, "{RETURN_CALL}({atm})"),
            Statement::Print(atm) => write!(f, "{PRINT_CALL}({atm})"),
            Statement::Assign { var, bound } => write!(f, "let {var} = {bound}"),
            Statement::If {
                cond,
                then_block,
                else_block,
            } => write!(
                f,
                "if {cond} {{\n\t{}\n}} else {{\n\t{}\n}};",
                then_block.to_string().replace("\n", "\n\t"),
                else_block.to_string().replace("\n", "\n\t"),
            ),
            Statement::While { cond, while_block } => write!(
                f,
                "while {cond} {{\n\t{}\n}}",
                while_block.to_string().replace("\n", "\n\t")
            ),
        }
    }
}
