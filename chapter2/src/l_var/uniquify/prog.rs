use super::{UniqueState, Uniquify};
use crate::l_var::syntax::Program;

impl Uniquify for Program {
    type Target = Program;
    fn uniquify(self, st: &mut UniqueState) -> Self::Target {
        Program {
            exp: self.exp.uniquify(st),
        }
    }
}

#[cfg(test)]
mod prog_tests {
    use super::{Program, Uniquify};
    use crate::l_var::syntax::{BinOp, Exp};

    #[test]
    fn unique_simple() {
        let result = Program {
            exp: Exp::Constant(3),
        }
        .uniquify(&mut Default::default());
        let expected = Program {
            exp: Exp::Constant(3),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn unique_complex() {
        let result = Program {
            exp: Exp::Assign {
                name: "x".to_owned(),
                bound_term: Box::new(Exp::BinOp {
                    exp1: Box::new(Exp::Constant(4)),
                    op: BinOp::Add,
                    exp2: Box::new(Exp::Constant(5)),
                }),
                in_term: Box::new(Exp::Assign {
                    name: "x".to_owned(),
                    bound_term: Box::new(Exp::Name("x".to_owned())),
                    in_term: Box::new(Exp::BinOp {
                        exp1: Box::new(Exp::Name("x".to_owned())),
                        op: BinOp::Add,
                        exp2: Box::new(Exp::Name("x".to_owned())),
                    }),
                }),
            },
        }
        .uniquify(&mut Default::default());
        let expected = Program {
            exp: Exp::Assign {
                name: "x0".to_owned(),
                bound_term: Box::new(Exp::BinOp {
                    exp1: Box::new(Exp::Constant(4)),
                    op: BinOp::Add,
                    exp2: Box::new(Exp::Constant(5)),
                }),
                in_term: Box::new(Exp::Assign {
                    name: "x1".to_owned(),
                    bound_term: Box::new(Exp::Name("x0".to_owned())),
                    in_term: Box::new(Exp::BinOp {
                        exp1: Box::new(Exp::Name("x1".to_owned())),
                        op: BinOp::Add,
                        exp2: Box::new(Exp::Name("x1".to_owned())),
                    }),
                }),
            },
        };
        assert_eq!(result, expected)
    }
}
