use super::{RemoveComplexOperands, exp_to_atm};
use std::collections::HashSet;

impl RemoveComplexOperands for surface::Statement {
    type Target = Vec<monadic::Statement>;
    fn remove_complex_operands(self, used_vars: &mut HashSet<String>) -> Self::Target {
        match self {
            surface::Statement::Return(exp) => {
                let (mut stmts, exp) = exp.remove_complex_operands(used_vars);
                let (assign, atm) = exp_to_atm(exp, used_vars);
                stmts.push(assign);
                stmts.push(monadic::Statement::Return(atm));
                stmts
            }
            surface::Statement::Print(exp) => {
                let (mut stmts, exp) = exp.remove_complex_operands(used_vars);
                let (assign, atm) = exp_to_atm(exp, used_vars);
                stmts.push(assign);
                stmts.push(monadic::Statement::Print(atm));
                stmts
            }
            surface::Statement::Assignment { var, bound } => {
                let (mut stmts, new_bind) = bound.remove_complex_operands(used_vars);
                stmts.push(monadic::Statement::assign(&var, new_bind));
                stmts
            }
            surface::Statement::If {
                cond_exp,
                then_block,
                else_block,
            } => {
                let (mut stmts, new_cond) = cond_exp.remove_complex_operands(used_vars);
                let (assign, cond_atm) = exp_to_atm(new_cond, used_vars);
                stmts.push(assign);
                let new_then = then_block.remove_complex_operands(used_vars);
                let new_else = else_block.remove_complex_operands(used_vars);
                stmts.push(monadic::Statement::cond(cond_atm, new_then, new_else));
                stmts
            }
            surface::Statement::While {
                cond_exp,
                while_block,
            } => {
                let (mut stmts, new_cond) = cond_exp.remove_complex_operands(used_vars);
                let (assign, cond_atm) = exp_to_atm(new_cond, used_vars);
                stmts.push(assign);
                let new_while = while_block.remove_complex_operands(used_vars);
                stmts.push(monadic::Statement::While {
                    cond: cond_atm,
                    while_block: new_while,
                });
                stmts
            }
        }
    }
}
