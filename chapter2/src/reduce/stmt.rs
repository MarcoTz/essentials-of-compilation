use super::{errors::Error, Reduce, ReduceState};
use crate::{
    l_var::syntax::Stmt as FullStmt,
    l_var_reduced::{Atm, Stmt},
};

impl Reduce for FullStmt {
    type Target = Stmt;
    fn reduce(self, st: &mut ReduceState) -> Result<Self::Target, Error> {
        match self {
            FullStmt::Exp(e) => e.reduce(st),
            FullStmt::Assign { name, exp } => {
                let exp_reduced = exp.reduce(st)?;
                if let Stmt::Exp(e) = exp_reduced {
                    Ok(Stmt::Assign { name, exp: e })
                } else {
                    Err(Error::StmtShouldBeExp(exp_reduced))
                }
            }
            FullStmt::Print(e) => {
                let exp_reduced = e.reduce(st)?;
                if let Stmt::Exp(e) = exp_reduced {
                    let new_name = st.fresh_var();
                    let assign = Stmt::Assign {
                        name: new_name.clone(),
                        exp: e,
                    };
                    st.add_stmt(assign);
                    Ok(Stmt::Print(Atm::Name(new_name)))
                } else {
                    Err(Error::StmtShouldBeExp(exp_reduced))
                }
            }
        }
    }
}
