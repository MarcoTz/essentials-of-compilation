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

#[cfg(test)]
mod exp_tests {
    use super::{c_var, l_var_reduced, ExplicateControl};

    #[test]
    fn explicate_atm() {
        let result = l_var_reduced::Exp::Atm(l_var_reduced::Atm::Int(2)).explicate_control();
        let expected = c_var::Tail::Return(c_var::Exp::Atm(c_var::Atm::Int(2)));
        assert_eq!(result, expected)
    }

    #[test]
    fn explicate_read() {
        let result = l_var_reduced::Exp::InputInt.explicate_control();
        let expected = c_var::Tail::Return(c_var::Exp::Read);
        assert_eq!(result, expected)
    }

    #[test]
    fn explicate_unary() {
        let result = l_var_reduced::Exp::UnaryOp {
            exp: l_var_reduced::Atm::Var("x".to_owned()),
            op: l_var_reduced::UnaryOp::Neg,
        }
        .explicate_control();
        let expected = c_var::Tail::Return(c_var::Exp::UnaryOp {
            exp: c_var::Atm::Var("x".to_owned()),
            op: c_var::UnaryOp::Neg,
        });
        assert_eq!(result, expected)
    }

    #[test]
    fn explicate_binary() {
        let result = l_var_reduced::Exp::BinOp {
            exp1: l_var_reduced::Atm::Int(1),
            op: l_var_reduced::BinOp::Add,
            exp2: l_var_reduced::Atm::Int(3),
        }
        .explicate_control();
        let expected = c_var::Tail::Return(c_var::Exp::BinOp {
            exp1: c_var::Atm::Int(1),
            op: c_var::BinOp::Add,
            exp2: c_var::Atm::Int(3),
        });
        assert_eq!(result, expected)
    }

    #[test]
    fn explicate_assign() {
        let result = l_var_reduced::Exp::Assign {
            name: "y".to_owned(),
            bound_term: Box::new(l_var_reduced::Exp::Assign {
                name: "x1".to_owned(),
                bound_term: Box::new(l_var_reduced::Exp::Atm(l_var_reduced::Atm::Int(20))),
                in_term: Box::new(l_var_reduced::Exp::Assign {
                    name: "x2".to_owned(),
                    bound_term: Box::new(l_var_reduced::Exp::Atm(l_var_reduced::Atm::Int(22))),
                    in_term: Box::new(l_var_reduced::Exp::BinOp {
                        exp1: l_var_reduced::Atm::Var("x1".to_owned()),
                        op: l_var_reduced::BinOp::Add,
                        exp2: l_var_reduced::Atm::Var("x2".to_owned()),
                    }),
                }),
            }),
            in_term: Box::new(l_var_reduced::Exp::Atm(l_var_reduced::Atm::Var(
                "y".to_owned(),
            ))),
        }
        .explicate_control();
        let expected = c_var::Tail::Seq(
            c_var::Stmt::Assign {
                var: "x1".to_owned(),
                exp: c_var::Exp::Atm(c_var::Atm::Int(20)),
            },
            Box::new(c_var::Tail::Seq(
                c_var::Stmt::Assign {
                    var: "x2".to_owned(),
                    exp: c_var::Exp::Atm(c_var::Atm::Int(22)),
                },
                Box::new(c_var::Tail::Seq(
                    c_var::Stmt::Assign {
                        var: "y".to_owned(),
                        exp: c_var::Exp::BinOp {
                            exp1: c_var::Atm::Var("x1".to_owned()),
                            op: c_var::BinOp::Add,
                            exp2: c_var::Atm::Var("x2".to_owned()),
                        },
                    },
                    Box::new(c_var::Tail::Return(c_var::Exp::Atm(c_var::Atm::Var(
                        "y".to_owned(),
                    )))),
                )),
            )),
        );
        assert_eq!(result, expected)
    }
}
