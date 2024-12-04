use super::ExplicateControl;
use crate::{c_var, l_var_reduced};

impl ExplicateControl for l_var_reduced::Exp {
    type Target = c_var::Tail;
    fn explicate_control(self) -> Self::Target {
        match self {
            l_var_reduced::Exp::Assign {
                name,
                bound_term,
                in_term,
            } => {
                let in_explicated = in_term.explicate_control();
                explicate_tail(bound_term.explicate_control(), name, in_explicated)
            }
            l_var_reduced::Exp::Atm(a) => {
                c_var::Tail::Return(c_var::Exp::Atm(a.explicate_control()))
            }
            l_var_reduced::Exp::InputInt => c_var::Tail::Return(c_var::Exp::Read),
            l_var_reduced::Exp::UnaryOp { op, exp } => c_var::Tail::Return(c_var::Exp::UnaryOp {
                op: op.explicate_control(),
                exp: exp.explicate_control(),
            }),
            l_var_reduced::Exp::BinOp { exp1, op, exp2 } => {
                c_var::Tail::Return(c_var::Exp::BinOp {
                    exp1: exp1.explicate_control(),
                    op: op.explicate_control(),
                    exp2: exp2.explicate_control(),
                })
            }
        }
    }
}

fn explicate_tail(tl: c_var::Tail, var: c_var::Var, ret_term: c_var::Tail) -> c_var::Tail {
    match tl {
        c_var::Tail::Return(exp) => {
            c_var::Tail::Seq(c_var::Stmt::Assign { var, exp }, Box::new(ret_term))
        }
        c_var::Tail::Seq(stmt, tl) => {
            c_var::Tail::Seq(stmt, Box::new(explicate_tail(*tl, var, ret_term)))
        }
    }
}
