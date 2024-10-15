use super::{Reduce, ReduceState};
use crate::{
    l_var::syntax::Stmt as FullStmt,
    l_var_reduced::{Atm, Stmt},
};

impl Reduce for FullStmt {
    type Target = Stmt;
    fn reduce(self, st: &mut ReduceState) -> Self::Target {
        match self {
            FullStmt::Exp(e) => {
                let e_red = e.reduce(st);
                e_red.into()
            }
            FullStmt::Assign { name, exp } => {
                let exp_red = exp.reduce(st);
                Stmt::Assign { name, exp: exp_red }
            }
            FullStmt::Print(e) => {
                let exp_reduced = e.reduce(st);
                let new_name = st.fresh_var();
                let assign = Stmt::Assign {
                    name: new_name.clone(),
                    exp: exp_reduced,
                };
                st.add_stmt(assign);
                Stmt::Print(Atm::Name(new_name))
            }
        }
    }
}
