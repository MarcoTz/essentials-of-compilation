use super::{Reduce, ReduceState};
use crate::{
    l_var::syntax::Exp as FullExp,
    l_var_reduced::{Atm, Exp, Stmt},
};

impl Reduce for FullExp {
    type Target = Exp;
    fn reduce(self, st: &mut ReduceState) -> Self::Target {
        match self {
            FullExp::Name(name) => Atm::Name(name).into(),
            FullExp::Constant(i) => Atm::Constant(i).into(),
            FullExp::InputInt => Exp::InputInt.into(),
            FullExp::UnaryOp { exp, op } => {
                let exp_red = exp.reduce(st);
                let new_name = st.fresh_var();
                let assign = Stmt::Assign {
                    name: new_name.clone(),
                    exp: exp_red,
                };
                let atm = Atm::Name(new_name);
                st.add_stmt(assign);
                Exp::UnaryOp { op, exp: atm }.into()
            }
            FullExp::BinOp { exp1, op, exp2 } => {
                let exp1_red = exp1.reduce(st);
                let exp2_red = exp2.reduce(st);

                let new_name1 = st.fresh_var();
                let assign1 = Stmt::Assign {
                    name: new_name1.clone(),
                    exp: exp1_red,
                };
                st.add_stmt(assign1);
                let atm1 = Atm::Name(new_name1);

                let new_name2 = st.fresh_var();
                let assign2 = Stmt::Assign {
                    name: new_name2.clone(),
                    exp: exp2_red,
                };
                st.add_stmt(assign2);
                let atm2 = Atm::Name(new_name2);

                Exp::BinOp {
                    op,
                    exp1: atm1,
                    exp2: atm2,
                }
                .into()
            }
        }
    }
}

#[cfg(test)]
mod exp_tests {
    use super::{Exp, FullExp, Reduce, ReduceState, Stmt};
    use crate::{BinOp, UnaryOp};

    #[test]
    fn reduce_name() {
        let result = FullExp::Name("x".to_owned()).reduce(&mut ReduceState::default());
        let expected = Exp::Atm("x".to_owned().into()).into();
        assert_eq!(result, expected)
    }

    #[test]
    fn reduce_const() {
        let mut st = ReduceState::default();
        let result = FullExp::Constant(1).reduce(&mut st);
        let expected = Exp::Atm(1.into()).into();
        assert_eq!(result, expected);
        assert_eq!(st, ReduceState::default())
    }

    #[test]
    fn reduce_input() {
        let mut st = ReduceState::default();
        let result = FullExp::InputInt.reduce(&mut st);
        let expected = Exp::InputInt.into();
        assert_eq!(result, expected);
        assert_eq!(st, ReduceState::default())
    }

    #[test]
    fn reduce_unary() {
        let mut st = ReduceState::default();
        let result = FullExp::UnaryOp {
            op: UnaryOp::Neg,
            exp: Box::new(1.into()),
        }
        .reduce(&mut st);
        let expected = Exp::UnaryOp {
            op: UnaryOp::Neg,
            exp: "x0".to_owned().into(),
        }
        .into();
        let mut new_st = ReduceState::default();
        new_st.num_vars = 1;
        new_st.add_stmt(Stmt::Assign {
            name: "x0".to_owned(),
            exp: Exp::Atm(1.into()),
        });
        assert_eq!(result, expected);
        assert_eq!(st, new_st)
    }

    #[test]
    fn reduce_bin() {
        let mut st = ReduceState::default();
        let result = FullExp::BinOp {
            op: BinOp::Add,
            exp1: Box::new(1.into()),
            exp2: Box::new(2.into()),
        }
        .reduce(&mut st);
        let expected = Exp::BinOp {
            op: BinOp::Add,
            exp1: "x0".to_owned().into(),
            exp2: "x1".to_owned().into(),
        };
        let mut new_st = ReduceState::default();
        new_st.num_vars = 2;
        new_st.add_stmt(Stmt::Assign {
            name: "x0".to_owned(),
            exp: Exp::Atm(1.into()),
        });
        new_st.add_stmt(Stmt::Assign {
            name: "x1".to_owned(),
            exp: Exp::Atm(2.into()),
        });
        assert_eq!(result, expected);
        assert_eq!(st, new_st)
    }
}
