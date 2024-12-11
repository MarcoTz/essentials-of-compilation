use crate::l_var::{BinOp, Exp, Program, UnaryOp};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, digit1, space0, space1},
};
use std::str;

pub mod errors;
pub mod keywords;
use errors::ParseRes;
use keywords::{parse_keyword, parse_non_keyword, Keyword};

fn parse_assign(input: &str) -> ParseRes<Exp> {
    let (rem, _) = parse_keyword(input, Keyword::Let)?;
    let (rem, _) = space0(rem)?;
    let (rem, var) = alphanumeric1(rem)?;
    let (rem, _) = space0(rem)?;
    let (rem, _) = tag("=")(rem)?;
    let (rem, _) = space0(rem)?;
    let (rem, bound_term) = parse_exp(rem)?;
    let (rem, _) = space0(rem)?;
    let (rem, _) = parse_keyword(rem, Keyword::In)?;
    let (rem, _) = space0(rem)?;
    let (rem, in_term) = parse_exp(rem)?;
    Ok((
        rem,
        Exp::Assign {
            name: var.to_owned(),
            bound_term: Box::new(bound_term),
            in_term: Box::new(in_term),
        },
    ))
}

fn parse_const(input: &str) -> ParseRes<Exp> {
    let (rem, dig) = digit1(input)?;
    Ok((rem, Exp::Constant(dig.parse::<i64>().unwrap())))
}

fn parse_var(input: &str) -> ParseRes<Exp> {
    let (rem, var) = parse_non_keyword(input)?;
    Ok((rem, Exp::Name(var)))
}

fn parse_input(input: &str) -> ParseRes<Exp> {
    let (remaining, _) = tag("input_int")(input)?;
    Ok((remaining, Exp::InputInt))
}

fn parse_unary(input: &str) -> ParseRes<Exp> {
    let (rem, _) = tag("(")(input)?;
    let (rem, _) = space0(rem)?;
    let (rem, _) = tag("-")(rem)?;
    let (rem, exp) = parse_exp(rem)?;
    let (rem, _) = space0(rem)?;
    let (rem, _) = tag(")")(rem)?;
    Ok((
        rem,
        Exp::UnaryOp {
            op: UnaryOp::Neg,
            exp: Box::new(exp),
        },
    ))
}

fn parse_binop(input: &str) -> ParseRes<Exp> {
    let (rem, _) = tag("(")(input)?;
    let (rem, _) = space0(rem)?;
    let (rem, op_str) = alt((tag("+"), tag("-")))(rem)?;
    let op = if op_str == "+" {
        BinOp::Add
    } else {
        BinOp::Sub
    };
    let (rem, _) = space0(rem)?;
    let (rem, exp1) = parse_exp(rem)?;
    let (rem, _) = space1(rem)?;
    let (rem, exp2) = parse_exp(rem)?;
    let (rem, _) = space0(rem)?;
    let (rem, _) = tag(")")(rem)?;

    Ok((
        rem,
        Exp::BinOp {
            exp1: Box::new(exp1),
            op,
            exp2: Box::new(exp2),
        },
    ))
}

fn parse_exp(input: &str) -> ParseRes<Exp> {
    alt((
        parse_assign,
        parse_input,
        parse_unary,
        parse_binop,
        parse_const,
        parse_var,
    ))(input)
}

pub fn parse_program(input: &str) -> ParseRes<Program> {
    let (rem, exp) = parse_exp(input)?;
    Ok((rem, Program { exp }))
}

#[cfg(test)]
mod parse_tests {
    use super::{parse_program, Exp, Program};
    use crate::l_var::{BinOp, UnaryOp};

    #[test]
    fn parse_var() {
        let (_, result) = parse_program("x").unwrap();
        let expected = Program {
            exp: Exp::Name("x".to_owned()),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_input() {
        let (_, result) = parse_program("input_int").unwrap();
        let expected = Program { exp: Exp::InputInt };
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_const() {
        let (_, result) = parse_program("15").unwrap();
        let expected = Program {
            exp: Exp::Constant(15),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_unary() {
        let (_, result) = parse_program("(-29)").unwrap();
        let expected = Program {
            exp: Exp::UnaryOp {
                exp: Box::new(Exp::Constant(29)),
                op: UnaryOp::Neg,
            },
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_binop() {
        let (_, result) = parse_program("(+ (-4) 5)").unwrap();
        let expected = Program {
            exp: Exp::BinOp {
                exp1: Box::new(Exp::UnaryOp {
                    exp: Box::new(Exp::Constant(4)),
                    op: UnaryOp::Neg,
                }),
                op: BinOp::Add,
                exp2: Box::new(Exp::Constant(5)),
            },
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_assign() {
        let (_, result) = parse_program("let x= (+ 5 3) in (-x)").unwrap();
        let expected = Program {
            exp: Exp::Assign {
                name: "x".to_owned(),
                bound_term: Box::new(Exp::BinOp {
                    exp1: Box::new(Exp::Constant(5)),
                    op: BinOp::Add,
                    exp2: Box::new(Exp::Constant(3)),
                }),
                in_term: Box::new(Exp::UnaryOp {
                    exp: Box::new(Exp::Name("x".to_owned())),
                    op: UnaryOp::Neg,
                }),
            },
        };
        assert_eq!(result, expected)
    }
}
