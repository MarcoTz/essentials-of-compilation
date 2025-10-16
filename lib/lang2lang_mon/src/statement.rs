use super::{RemoveComplexOperands, exp_to_atm};
use std::collections::HashSet;

impl RemoveComplexOperands for lang::Statement {
    type Target = Vec<lang_mon::Statement>;
    fn remove_complex_operands(self, used_vars: &mut HashSet<String>) -> Self::Target {
        match self {
            lang::Statement::Return(exp) => {
                let (mut stmts, exp) = exp.remove_complex_operands(used_vars);
                let (assign, atm) = exp_to_atm(exp, used_vars);
                stmts.push(assign);
                stmts.push(lang_mon::Statement::Return(atm));
                stmts
            }
            lang::Statement::Print(exp) => {
                let (mut stmts, exp) = exp.remove_complex_operands(used_vars);
                let (assign, atm) = exp_to_atm(exp, used_vars);
                stmts.push(assign);
                stmts.push(lang_mon::Statement::Print(atm));
                stmts
            }
            lang::Statement::Assignment { var, bound } => {
                let (mut stmts, new_bind) = bound.remove_complex_operands(used_vars);
                stmts.push(lang_mon::Statement::assign(&var, new_bind));
                stmts
            }
            lang::Statement::If {
                cond_exp,
                then_block,
                else_block,
            } => {
                let (mut stmts, new_cond) = cond_exp.remove_complex_operands(used_vars);
                let (assign, cond_atm) = exp_to_atm(new_cond, used_vars);
                stmts.push(assign);
                let new_then = then_block.remove_complex_operands(used_vars);
                let new_else = else_block.remove_complex_operands(used_vars);
                stmts.push(lang_mon::Statement::cond(cond_atm, new_then, new_else));
                stmts
            }
        }
    }
}
