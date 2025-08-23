use super::{UniqueState, Uniquify};
use crate::l_var::{subst::Subst, syntax::Exp};

impl Uniquify for Exp {
    type Target = Exp;
    fn uniquify(self, st: &mut UniqueState) -> Self::Target {
        match self {
            Exp::Assign {
                name,
                bound_term,
                in_term,
            } => {
                let bound_subst = bound_term.subst_vars(&st.var_subst).uniquify(st);
                let new_name = st.fresh_var(&name);
                st.var_subst.insert(name.clone(), new_name.clone());
                Exp::Assign {
                    name: new_name,
                    bound_term: Box::new(bound_subst),
                    in_term: Box::new(in_term.subst_vars(&st.var_subst).uniquify(st)),
                }
            }
            Exp::UnaryOp { op, exp } => Exp::UnaryOp {
                op,
                exp: Box::new(exp.uniquify(st)),
            },
            Exp::BinOp { exp1, op, exp2 } => Exp::BinOp {
                exp1: Box::new(exp1.uniquify(st)),
                op,
                exp2: Box::new(exp2.uniquify(st)),
            },
            _ => self,
        }
    }
}

#[cfg(test)]
mod uniquify_test {
    use super::{Exp, Uniquify};
    use crate::l_var::syntax::BinOp;

    #[test]
    fn uniquify() {
        let result = Exp::Assign {
            name: "x".to_owned(),
            bound_term: Box::new(32.into()),
            in_term: Box::new(Exp::BinOp {
                exp1: Box::new(Exp::Assign {
                    name: "x".to_owned(),
                    bound_term: Box::new(10.into()),
                    in_term: Box::new("x".to_owned().into()),
                }),
                op: BinOp::Add,
                exp2: Box::new("x".to_owned().into()),
            }),
        }
        .uniquify(&mut Default::default());
        let expected = Exp::Assign {
            name: "x0".to_owned(),
            bound_term: Box::new(32.into()),
            in_term: Box::new(Exp::BinOp {
                exp1: Box::new(Exp::Assign {
                    name: "x1".to_owned(),
                    bound_term: Box::new(10.into()),
                    in_term: Box::new("x1".to_owned().into()),
                }),
                op: BinOp::Add,
                exp2: Box::new("x0".to_owned().into()),
            }),
        };
        assert_eq!(result, expected)
    }
}
