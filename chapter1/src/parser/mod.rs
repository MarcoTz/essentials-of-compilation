use crate::syntax::{BinOp, Exp, Program, UnaryOp};
use errors::Error;

pub mod errors;
pub mod lexer;

use lexer::{consume_sequence, consume_whitespace, parse_int};

///Accepted Grammar:
/// exp ::=
///   int
///   | (read)
///   | (-exp)
///   | (+ exp exp)
///   | (- exp exp)
pub fn parse_l_int(input: &mut String) -> Result<Program, Error> {
    consume_whitespace(input);
    let exp = parse_exp(input)?;
    Ok(Program { exp })
}

fn parse_exp(input: &mut String) -> Result<Exp, Error> {
    let exp = match input.chars().next() {
        None => Err(Error::UnexpectedEOI),
        Some('(') => parse_paren_exp(input),
        Some(_) => Ok(parse_int(input)?.into()),
    }?;
    consume_whitespace(input);
    Ok(exp)
}

fn parse_op(input: &mut String) -> Result<Exp, Error> {
    if input.is_empty() {
        return Err(Error::UnexpectedEOI);
    }

    consume_whitespace(input);
    match input.remove(0) {
        '+' => {
            consume_whitespace(input);
            let exp1 = parse_exp(input)?;
            consume_whitespace(input);
            let exp2 = parse_exp(input)?;
            Ok(Exp::BinOp {
                exp1: Box::new(exp1),
                op: BinOp::Add,
                exp2: Box::new(exp2),
            })
        }
        '-' => {
            let exp1 = parse_exp(input)?;
            consume_whitespace(input);
            if input.starts_with(')') {
                Ok(Exp::UnaryOp {
                    op: UnaryOp::Neg,
                    exp: Box::new(exp1),
                })
            } else {
                let exp2 = parse_exp(input)?;
                Ok(Exp::BinOp {
                    exp1: Box::new(exp1),
                    op: BinOp::Sub,
                    exp2: Box::new(exp2),
                })
            }
        }
        c => Err(Error::UnexpectedCharacter(c)),
    }
}

fn parse_paren_exp(input: &mut String) -> Result<Exp, Error> {
    if input.is_empty() {
        return Err(Error::UnexpectedEOI);
    }

    let paren_o = input.remove(0);
    if let '(' = paren_o {
        Ok(())
    } else {
        Err(Error::ParenMismatch)
    }?;

    consume_whitespace(input);

    let exp = match input.chars().next() {
        None => Err(Error::UnexpectedEOI),
        Some('r') => {
            consume_sequence(input, "read")?;
            Ok(Exp::InputInt)
        }
        Some(_) => parse_op(input),
    }?;

    consume_whitespace(input);

    if input.is_empty() {
        return Err(Error::UnexpectedEOI);
    }

    let paren_c = input.remove(0);
    if let ')' = paren_c {
        Ok(())
    } else {
        Err(Error::ParenMismatch)
    }?;

    Ok(exp)
}

#[cfg(test)]
mod parser_tests {
    use super::{parse_int, parse_paren_exp, BinOp, Exp, UnaryOp};

    #[test]
    fn parse_read() {
        let result = parse_paren_exp(&mut "(read )".to_owned()).unwrap();
        let expected = Exp::InputInt;
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_sum() {
        let result = parse_paren_exp(&mut "(+ 40 3)".to_owned()).unwrap();
        let expected = Exp::BinOp {
            exp1: Box::new(40.into()),
            op: BinOp::Add,
            exp2: Box::new(3.into()),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_diff() {
        let result = parse_paren_exp(&mut "( - 10 5)".to_owned()).unwrap();
        let expected = Exp::BinOp {
            exp1: Box::new(10.into()),
            op: BinOp::Sub,
            exp2: Box::new(5.into()),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_neg() {
        let result = parse_paren_exp(&mut "(- 5)".to_owned()).unwrap();
        let expected = Exp::UnaryOp {
            exp: Box::new(5.into()),
            op: UnaryOp::Neg,
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_100() {
        let result = parse_int(&mut " 100 ".to_owned()).unwrap();
        let expected = 100.into();
        assert_eq!(result, expected)
    }
}
