use super::{Atom, Block, Expression};
use crate::{PRINT_CALL, RETURN_CALL};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Return(Atom),
    Print(Atom),
    Assign {
        var: String,
        bound: Expression,
    },
    If {
        cond_exp: Atom,
        then_block: Block,
        else_block: Block,
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
            cond_exp,
            then_block,
            else_block,
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
                cond_exp,
                then_block,
                else_block,
            } => write!(
                f,
                "if {cond_exp} {{\n\t{}\n}} else {{\n\t{}\n}};",
                then_block.to_string().replace("\n", "\n\t"),
                else_block.to_string().replace("\n", "\n\t"),
            ),
        }
    }
}
