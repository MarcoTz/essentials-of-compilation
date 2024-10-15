use super::l_var_reduced::Stmt;
use crate::Var;
use errors::Error;

pub mod errors;
pub mod exp;
pub mod stmt;

pub struct ReduceState {
    previous_prog: Vec<Stmt>,
    num_vars: i64,
}

impl ReduceState {
    pub fn fresh_var(&mut self) -> Var {
        let new_var = "x".to_owned() + &self.num_vars.to_string();
        self.num_vars += 1;
        new_var
    }

    pub fn add_stmt(&mut self, stmt: Stmt) {
        while stmt.occurs(&("x".to_owned() + &self.num_vars.to_string())) {
            self.num_vars += 1;
        }
        self.previous_prog.push(stmt);
    }
}

pub trait Reduce {
    type Target;
    fn reduce(self, st: &mut ReduceState) -> Result<Self::Target, Error>;
}
