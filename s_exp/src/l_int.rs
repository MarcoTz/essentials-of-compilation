use super::{errors::Error, s_exp::SExp, Grammar, Symbol};
use chapter1::syntax::{BinOp, Exp, Program, UnaryOp};
use std::{fmt, str::FromStr};

pub enum LIntSymbol {
    Num(i64),
    Read,
    Minus,
    Plus,
}

impl LIntSymbol {
    fn to_exp(self, mut args: Vec<Exp>) -> Result<Exp, Error> {
        match (self, args.len()) {
            (LIntSymbol::Num(n), 0) => Ok(Exp::Constant(n)),
            (LIntSymbol::Read, 0) => Ok(Exp::InputInt),
            (LIntSymbol::Minus, 1) => Ok(Exp::UnaryOp {
                op: UnaryOp::Neg,
                exp: Box::new(args.remove(0)),
            }),
            (LIntSymbol::Minus, 2) => Ok(Exp::BinOp {
                op: BinOp::Sub,
                exp1: Box::new(args.remove(0)),
                exp2: Box::new(args.remove(0)),
            }),
            (LIntSymbol::Plus, 2) => Ok(Exp::BinOp {
                op: BinOp::Add,
                exp1: Box::new(args.remove(0)),
                exp2: Box::new(args.remove(0)),
            }),
            (s, n) => Err(Error::ArgumentMismatch(format!(
                "{s} cannot have {n} arguments"
            ))),
        }
    }
}

pub struct LInt;

impl LInt {
    fn s_exp_to_exp(&self, exp: SExp<LIntSymbol>) -> Result<Exp, Error> {
        match exp {
            SExp::Symbol(sym) => sym.to_exp(vec![]),
            SExp::Expr(mut exprs) => {
                if exprs.is_empty() {
                    return Err(Error::UnexpectedEOI);
                }
                let first = exprs.remove(0);
                let args_conv = exprs
                    .into_iter()
                    .map(|exp| self.s_exp_to_exp(exp))
                    .collect::<Result<Vec<Exp>, Error>>()?;
                match first {
                    SExp::Symbol(s) => s.to_exp(args_conv),
                    SExp::Expr(_) => Err(Error::UnexpectedSymbol(first.to_string())),
                }
            }
        }
    }
}

impl Grammar for LInt {
    type Symbol = LIntSymbol;
    type Target = Program;

    fn to_target(&self, exp: SExp<Self::Symbol>) -> Result<Self::Target, Error> {
        self.s_exp_to_exp(exp).map(|exp| Program { exp })
    }
}

impl FromStr for LIntSymbol {
    type Err = Error;
    fn from_str(s: &str) -> Result<LIntSymbol, Self::Err> {
        match s.to_lowercase().trim() {
            "read" => Ok(LIntSymbol::Read),
            "-" => Ok(LIntSymbol::Minus),
            "+" => Ok(LIntSymbol::Plus),
            s => s
                .parse::<i64>()
                .map(|i| LIntSymbol::Num(i))
                .map_err(|_| Error::UnexpectedSymbol(s.to_owned())),
        }
    }
}

impl fmt::Display for LIntSymbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LIntSymbol::Read => f.write_str("read"),
            LIntSymbol::Plus => f.write_str("+"),
            LIntSymbol::Minus => f.write_str("-"),
            LIntSymbol::Num(n) => write!(f, "{n}"),
        }
    }
}

impl Symbol for LIntSymbol {}

#[cfg(test)]
mod l_int_tests {
    use super::{BinOp, Exp, Grammar, LInt, LIntSymbol, Program, SExp, UnaryOp};

    #[test]
    fn parse_input() {
        let result = LInt.to_target(SExp::Symbol(LIntSymbol::Read)).unwrap();
        let expected = Program { exp: Exp::InputInt };
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_neg() {
        let result = LInt
            .to_target(SExp::Expr(vec![
                SExp::Symbol(LIntSymbol::Minus),
                SExp::Symbol(LIntSymbol::Num(10)),
            ]))
            .unwrap();
        let expected = Program {
            exp: Exp::UnaryOp {
                op: UnaryOp::Neg,
                exp: Box::new(Exp::Constant(10)),
            },
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_add() {
        let result = LInt
            .to_target(SExp::Expr(vec![
                SExp::Symbol(LIntSymbol::Plus),
                SExp::Symbol(LIntSymbol::Num(2)),
                SExp::Symbol(LIntSymbol::Num(3)),
            ]))
            .unwrap();
        let expected = Program {
            exp: Exp::BinOp {
                op: BinOp::Add,
                exp1: Box::new(Exp::Constant(2)),
                exp2: Box::new(Exp::Constant(3)),
            },
        };
        assert_eq!(result, expected)
    }
}
