use crate::l_if::{
    errors::Error as LangErr,
    syntax::{Exp, Program},
};

pub(self) mod errors;
pub(self) mod exp;
pub(self) mod keywords;
pub(self) mod ops;

pub use errors::Error;
use exp::parse_exp;

pub fn parse(input: String) -> Result<Program, LangErr> {
    let result: Result<(&str, Exp), Error> = parse_exp(&input.as_str()).map_err(|err| err.into());
    let (rem, exp) = result?;
    if rem.is_empty() {
        Ok(exp)
    } else {
        Err(Error::RemainingInput(rem.to_owned()))?
    }
}

#[cfg(test)]
mod parse_tests {
    use super::parse;
    use crate::l_if::syntax::{Cmp, Exp, Op};

    #[test]
    fn example_parse() {
        let result = parse(
            "(let (x (read)) (let (y (read)) (if (if (< x 1) (= x 0) (= x 2)) (* y 2) (+ y 10))))"
                .to_owned(),
        )
        .unwrap();
        let expected = Exp::Let {
            var: "x".to_owned(),
            bound_exp: Box::new(Exp::Prim {
                op: Op::Read,
                args: vec![],
            }),
            in_exp: Box::new(Exp::Let {
                var: "y".to_owned(),
                bound_exp: Box::new(Exp::Prim {
                    op: Op::Read,
                    args: vec![],
                }),
                in_exp: Box::new(Exp::If {
                    ifc: Box::new(Exp::If {
                        ifc: Box::new(Exp::Prim {
                            op: Op::Cmp(Cmp::Less),
                            args: vec![Exp::Var("x".to_owned()), Exp::Int(1)],
                        }),
                        thenc: Box::new(Exp::Prim {
                            op: Op::Cmp(Cmp::Equal),
                            args: vec![Exp::Var("x".to_owned()), Exp::Int(0)],
                        }),
                        elsec: Box::new(Exp::Prim {
                            op: Op::Cmp(Cmp::Equal),
                            args: vec![Exp::Var("x".to_owned()), Exp::Int(2)],
                        }),
                    }),
                    thenc: Box::new(Exp::Prim {
                        op: Op::Mult,
                        args: vec![Exp::Var("y".to_owned()), Exp::Int(2)],
                    }),
                    elsec: Box::new(Exp::Prim {
                        op: Op::Plus,
                        args: vec![Exp::Var("y".to_owned()), Exp::Int(10)],
                    }),
                }),
            }),
        };
        assert_eq!(result, expected)
    }
}
