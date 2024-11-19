use super::{Reduce, ReduceState};
use crate::{l_var::syntax as l_var, l_var_reduced};

impl Reduce for l_var::Exp {
    type Target = l_var_reduced::Exp;
    fn reduce(self, st: &mut ReduceState) -> Self::Target {
        match self {
            l_var::Exp::Name(name) => {
                st.used_vars.insert(name.clone());
                l_var_reduced::Atm::Var(name).into()
            }
            l_var::Exp::Constant(i) => l_var_reduced::Atm::Int(i).into(),
            l_var::Exp::InputInt => l_var_reduced::Exp::InputInt,
            l_var::Exp::UnaryOp { exp, op } => {
                let exp_red = exp.reduce(st);
                let op_reduced = op.reduce(st);
                if let l_var_reduced::Exp::Atm(at) = exp_red {
                    l_var_reduced::Exp::UnaryOp {
                        op: op_reduced,
                        exp: at,
                    }
                } else {
                    let new_name = st.fresh_var();
                    l_var_reduced::Exp::Assign {
                        name: new_name.clone(),
                        bound_term: Box::new(exp_red),
                        in_term: Box::new(l_var_reduced::Exp::UnaryOp {
                            op: op_reduced,
                            exp: new_name.into(),
                        }),
                    }
                }
            }
            l_var::Exp::BinOp { exp1, op, exp2 } => {
                let exp1_red = exp1.reduce(st);
                let exp2_red = exp2.reduce(st);
                let op_red = op.reduce(st);

                match (exp1_red, exp2_red) {
                    (l_var_reduced::Exp::Atm(at1), l_var_reduced::Exp::Atm(at2)) => {
                        l_var_reduced::Exp::BinOp {
                            exp1: at1,
                            op: op_red,
                            exp2: at2,
                        }
                    }
                    (l_var_reduced::Exp::Atm(at), e) => {
                        let new_name = st.fresh_var();
                        l_var_reduced::Exp::Assign {
                            name: new_name.clone(),
                            bound_term: Box::new(e),
                            in_term: Box::new(l_var_reduced::Exp::BinOp {
                                exp1: at,
                                op: op_red,
                                exp2: new_name.into(),
                            }),
                        }
                    }
                    (e, l_var_reduced::Exp::Atm(at)) => {
                        let new_name = st.fresh_var();
                        l_var_reduced::Exp::Assign {
                            name: new_name.clone(),
                            bound_term: Box::new(e),
                            in_term: Box::new(l_var_reduced::Exp::BinOp {
                                exp1: new_name.into(),
                                op: op_red,
                                exp2: at,
                            }),
                        }
                    }
                    (e1, e2) => {
                        let new_name1 = st.fresh_var();
                        let new_name2 = st.fresh_var();
                        l_var_reduced::Exp::Assign {
                            name: new_name1.clone(),
                            bound_term: Box::new(e1),
                            in_term: Box::new(l_var_reduced::Exp::Assign {
                                name: new_name2.clone(),
                                bound_term: Box::new(e2),
                                in_term: Box::new(l_var_reduced::Exp::BinOp {
                                    exp1: new_name1.into(),
                                    op: op_red,
                                    exp2: new_name2.into(),
                                }),
                            }),
                        }
                    }
                }
            }

            l_var::Exp::Assign {
                name,
                bound_term,
                in_term,
            } => {
                st.used_vars.insert(name.clone());
                l_var_reduced::Exp::Assign {
                    name,
                    bound_term: Box::new(bound_term.reduce(st)),
                    in_term: Box::new(in_term.reduce(st)),
                }
            }
        }
    }
}

#[cfg(test)]
mod exp_tests {
    use super::Reduce;
    use crate::{l_var::syntax as l_var, l_var_reduced};

    #[test]
    fn reduce_name() {
        let result = l_var::Exp::Name("x".to_owned()).reduce(&mut Default::default());
        let expected = l_var_reduced::Exp::Atm("x".to_owned().into()).into();
        assert_eq!(result, expected)
    }

    #[test]
    fn reduce_const() {
        let mut st = Default::default();
        let result = l_var::Exp::Constant(1).reduce(&mut st);
        let expected = l_var_reduced::Exp::Atm(1.into()).into();
        assert_eq!(result, expected);
        assert_eq!(st, Default::default())
    }

    #[test]
    fn reduce_input() {
        let mut st = Default::default();
        let result = l_var::Exp::InputInt.reduce(&mut st);
        let expected = l_var_reduced::Exp::InputInt.into();
        assert_eq!(result, expected);
        assert_eq!(st, Default::default())
    }

    #[test]
    fn reduce_unary() {
        let mut st = Default::default();
        let result = l_var::Exp::UnaryOp {
            op: l_var::UnaryOp::Neg,
            exp: Box::new(1.into()),
        }
        .reduce(&mut st);
        let expected = l_var_reduced::Exp::UnaryOp {
            op: l_var_reduced::UnaryOp::Neg,
            exp: 1.into(),
        }
        .into();
        assert_eq!(result, expected);
        assert_eq!(st, Default::default())
    }

    #[test]
    fn reduce_bin() {
        let mut st = Default::default();
        let result = l_var::Exp::BinOp {
            op: l_var::BinOp::Add,
            exp1: Box::new(1.into()),
            exp2: Box::new(2.into()),
        }
        .reduce(&mut st);
        let expected = l_var_reduced::Exp::BinOp {
            op: l_var_reduced::BinOp::Add,
            exp1: 1.into(),
            exp2: 2.into(),
        };
        assert_eq!(result, expected);
        assert_eq!(st, Default::default())
    }
}
