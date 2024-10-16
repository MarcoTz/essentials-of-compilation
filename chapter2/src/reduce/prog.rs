use super::{Reduce, ReduceState};
use crate::{l_var::syntax::Module as FullModule, l_var_reduced::Module};

impl Reduce for FullModule {
    type Target = Module;
    fn reduce(self, st: &mut ReduceState) -> Self::Target {
        for stmt in self.stmts.into_iter() {
            let stmt_red = stmt.reduce(st);
            st.add_stmt(stmt_red)
        }
        st.previous_prog.clone()
    }
}

#[cfg(test)]
mod prog_tests {
    use super::{FullModule, Reduce, ReduceState};
    use crate::{
        l_var::syntax::{Exp, Stmt},
        l_var_reduced, BinOp,
    };

    #[test]
    fn reduce_empty() {
        let result = FullModule { stmts: vec![] }.reduce(&mut ReduceState::default());
        let expected = vec![];
        assert_eq!(result, expected)
    }

    #[test]
    fn reduce_prog() {
        let result = FullModule {
            stmts: vec![
                Stmt::Assign {
                    name: "x".to_owned(),
                    exp: Exp::InputInt,
                },
                Stmt::Assign {
                    name: "y".to_owned(),
                    exp: Exp::InputInt,
                },
                Stmt::Exp(Exp::BinOp {
                    exp1: Box::new(Exp::Name("x".to_owned())),
                    exp2: Box::new(Exp::Name("y".to_owned())),
                    op: BinOp::Add,
                }),
            ],
        }
        .reduce(&mut ReduceState::default());
        let expected = vec![
            l_var_reduced::Stmt::Assign {
                name: "x".to_owned(),
                exp: l_var_reduced::Exp::InputInt,
            },
            l_var_reduced::Stmt::Assign {
                name: "y".to_owned(),
                exp: l_var_reduced::Exp::InputInt,
            },
            l_var_reduced::Stmt::Exp(l_var_reduced::Exp::BinOp {
                exp1: l_var_reduced::Atm::Name("x".to_owned()),
                exp2: l_var_reduced::Atm::Name("y".to_owned()),
                op: BinOp::Add,
            }),
        ];
        assert_eq!(result, expected)
    }
}
