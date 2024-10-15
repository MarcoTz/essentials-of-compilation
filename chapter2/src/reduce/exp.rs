use super::{errors::Error, Reduce, ReduceState};
use crate::{
    l_var::syntax::Exp as FullExp,
    l_var_reduced::{Atm, Exp, Stmt},
};

impl Reduce for FullExp {
    type Target = Stmt;
    fn reduce(self, st: &mut ReduceState) -> Result<Self::Target, Error> {
        match self {
            FullExp::Name(name) => Ok(Atm::Name(name).into()),
            FullExp::Constant(i) => Ok(Atm::Constant(i).into()),
            FullExp::InputInt => Ok(Exp::InputInt.into()),
            FullExp::UnaryOp { exp, op } => {
                let exp_red = exp.reduce(st)?;
                if let Stmt::Exp(e) = exp_red {
                    let new_name = st.fresh_var();
                    let assign = Stmt::Assign {
                        name: new_name.clone(),
                        exp: e,
                    };
                    let atm = Atm::Name(new_name);
                    st.add_stmt(assign);
                    Ok(Exp::UnaryOp { op, exp: atm }.into())
                } else {
                    Err(Error::StmtShouldBeExp(exp_red))
                }
            }
            FullExp::BinOp { exp1, op, exp2 } => {
                let exp1_red = exp1.reduce(st)?;
                let exp2_red = exp2.reduce(st)?;
                let exp1 = if let Stmt::Exp(e) = exp1_red {
                    Ok(e)
                } else {
                    Err(Error::StmtShouldBeExp(exp1_red))
                }?;
                let exp2 = if let Stmt::Exp(e) = exp2_red {
                    Ok(e)
                } else {
                    Err(Error::StmtShouldBeExp(exp2_red))
                }?;

                let new_name1 = st.fresh_var();
                let assign1 = Stmt::Assign {
                    name: new_name1.clone(),
                    exp: exp1,
                };
                st.add_stmt(assign1);
                let atm1 = Atm::Name(new_name1);

                let new_name2 = st.fresh_var();
                let assign2 = Stmt::Assign {
                    name: new_name2.clone(),
                    exp: exp2,
                };
                st.add_stmt(assign2);
                let atm2 = Atm::Name(new_name2);

                Ok(Exp::BinOp {
                    op,
                    exp1: atm1,
                    exp2: atm2,
                }
                .into())
            }
        }
    }
}
