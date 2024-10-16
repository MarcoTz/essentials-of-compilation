use super::l_var_reduced::Stmt;
use crate::Var;

pub mod exp;
pub mod prog;
pub mod stmt;

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct ReduceState {
    previous_prog: Vec<Stmt>,
    num_vars: usize,
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
    fn reduce(self, st: &mut ReduceState) -> Self::Target;
}

#[cfg(test)]
mod reduce_tests {
    use super::{ReduceState, Stmt};
    use crate::l_var_reduced::Exp;

    fn example_state() -> ReduceState {
        ReduceState {
            previous_prog: vec![Stmt::Print(1.into()), Stmt::Exp(Exp::Atm(2.into()))],
            num_vars: 4,
        }
    }

    #[test]
    fn fresh_var_default() {
        let result = ReduceState::default().fresh_var();
        let expected = "x0";
        assert_eq!(result, expected)
    }

    #[test]
    fn fresh_var_ex() {
        let result = example_state().fresh_var();
        let expected = "x4";
        assert_eq!(result, expected)
    }

    #[test]
    fn add_stmt() {
        let mut result = example_state();
        result.add_stmt(Stmt::Assign {
            name: "x4".to_owned(),
            exp: Exp::Atm(1.into()),
        });
        let mut expected = example_state();
        expected.previous_prog.push(Stmt::Assign {
            name: "x4".to_owned(),
            exp: Exp::Atm(1.into()),
        });
        expected.num_vars = 5;
        assert_eq!(result, expected)
    }
}
