use super::{Reduce, ReduceState};
use crate::{
    l_var::syntax::Exp as FullExp,
    l_var_reduced::{Atm, Exp, Stmt},
};

fn exp_to_atm(exp: Exp, st: &mut ReduceState) -> Atm {
    match exp.as_atm() {
        Some(at) => at,
        None => {
            let new_name = st.fresh_var();
            let assign = Stmt::Assign {
                name: new_name.clone(),
                exp,
            };
            st.add_stmt(assign);
            Atm::Name(new_name)
        }
    }
}
impl Reduce for FullExp {
    type Target = Exp;
    fn reduce(self, st: &mut ReduceState) -> Self::Target {
        match self {
            FullExp::Name(name) => Atm::Name(name).into(),
            FullExp::Constant(i) => Atm::Constant(i).into(),
            FullExp::InputInt => Exp::InputInt,
            FullExp::UnaryOp { exp, op } => {
                let exp_red = exp.reduce(st);
                let atm = exp_to_atm(exp_red, st);
                Exp::UnaryOp { op, exp: atm }
            }
            FullExp::BinOp { exp1, op, exp2 } => {
                let exp1_red = exp1.reduce(st);
                let exp2_red = exp2.reduce(st);
                let atm1 = exp_to_atm(exp1_red, st);
                let atm2 = exp_to_atm(exp2_red, st);

                Exp::BinOp {
                    op,
                    exp1: atm1,
                    exp2: atm2,
                }
            }
        }
    }
}

#[cfg(test)]
mod exp_tests {
    use super::{Exp, FullExp, Reduce, ReduceState};
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
            exp: 1.into(),
        }
        .into();
        assert_eq!(result, expected);
        assert_eq!(st, ReduceState::default())
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
            exp1: 1.into(),
            exp2: 2.into(),
        };
        let mut new_st = ReduceState::default();
        new_st.num_vars = 0;
        assert_eq!(result, expected);
        assert_eq!(st, new_st)
    }
}
