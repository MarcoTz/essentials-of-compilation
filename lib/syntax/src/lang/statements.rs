use super::{Block, expressions::Expression};
use crate::{PRINT_CALL, RETURN_CALL};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Return(Expression),
    Print(Expression),
    Assignment {
        var: String,
        bound: Expression,
    },
    If {
        cond_exp: Expression,
        then_block: Block,
        else_block: Block,
    },
}

impl Statement {
    pub fn assign(var: &str, bound: Expression) -> Statement {
        Statement::Assignment {
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

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Statement::Return(exp) => write!(f, "{RETURN_CALL}({exp});"),
            Statement::Print(exp) => write!(f, "{PRINT_CALL}({exp});"),
            Statement::Assignment { var, bound } => write!(f, "let {var} = {bound};"),
            Statement::If {
                cond_exp,
                then_block,
                else_block,
            } => write!(
                f,
                "if {cond_exp} {{ {} }} else {{ {} }}",
                then_block, else_block
            ),
        }
    }
}
