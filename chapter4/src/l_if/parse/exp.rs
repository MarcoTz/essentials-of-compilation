use super::{
    errors::ParseRes,
    keywords::{parse_keyword, parse_non_keyword, Keyword},
    ops::parse_op,
};
use crate::l_if::syntax::{Exp, Op};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, space0, space1},
    multi::separated_list0,
};

fn parse_let(input: &str) -> ParseRes<Exp> {
    let (input, _) = tag("(")(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = parse_keyword(input, Keyword::Let)?;
    let (input, _) = space0(input)?;
    let (input, _) = tag("(")(input)?;
    let (input, var) = parse_non_keyword(input)?;
    println!("parsed let {var}");
    let (input, _) = space1(input)?;
    let (input, bound_exp) = parse_exp(input)?;
    println!("parsed bound exp {bound_exp}");
    let (input, _) = space0(input)?;
    let (input, _) = tag(")")(input)?;
    let (input, _) = space0(input)?;
    let (input, in_exp) = parse_exp(input)?;
    println!("parsed in exp {in_exp}");
    let (input, _) = space0(input)?;
    let (input, _) = tag(")")(input)?;
    Ok((
        input,
        Exp::Let {
            var,
            bound_exp: Box::new(bound_exp),
            in_exp: Box::new(in_exp),
        },
    ))
}

fn parse_prim(input: &str) -> ParseRes<Exp> {
    let (input, _) = tag("(")(input)?;
    let (input, _) = space0(input)?;
    let (input, mut op) = parse_op(input)?;
    println!("parsed op {op}");
    let (input, _) = space0(input)?;
    let (input, args) = separated_list0(space1, parse_exp)(input)?;
    if matches!(op, Op::Neg) && args.len() > 1 {
        op = Op::Sub;
    }
    let (input, _) = space0(input)?;
    let (input, _) = tag(")")(input)?;
    Ok((input, Exp::Prim { op, args }))
}

fn parse_if(input: &str) -> ParseRes<Exp> {
    let (input, _) = tag("(")(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = parse_keyword(input, Keyword::If)?;
    let (input, _) = space0(input)?;
    let (input, ifc) = parse_exp(input)?;
    let (input, _) = space1(input)?;
    let (input, thenc) = parse_exp(input)?;
    let (input, _) = space1(input)?;
    let (input, elsec) = parse_exp(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = tag(")")(input)?;
    Ok((
        input,
        Exp::If {
            ifc: Box::new(ifc),
            thenc: Box::new(thenc),
            elsec: Box::new(elsec),
        },
    ))
}

fn parse_const(input: &str) -> ParseRes<Exp> {
    let (rem, dig) = digit1(input)?;
    Ok((rem, Exp::Int(dig.parse::<i64>().unwrap())))
}

fn parse_bool(input: &str) -> ParseRes<Exp> {
    alt((
        |s| tag("true")(s).map(|(rem, _)| (rem, Exp::Bool(true))),
        |s| tag("false")(s).map(|(rem, _)| (rem, Exp::Bool(false))),
    ))(input)
}

fn parse_var(input: &str) -> ParseRes<Exp> {
    parse_non_keyword(input).map(|(rem, var)| (rem, Exp::Var(var)))
}

pub fn parse_exp(input: &str) -> ParseRes<Exp> {
    alt((
        parse_let,
        parse_prim,
        parse_if,
        parse_const,
        parse_bool,
        parse_var,
    ))(input)
}

#[cfg(test)]
mod exp_tests {
    use super::{parse_bool, parse_const, parse_if, parse_let, parse_prim, parse_var};
    use crate::l_if::syntax::{Cmp, Exp, Op};

    #[test]
    fn let_parse() {
        let (_, result) = parse_let("(let (x (read)) x)").unwrap();
        let expected = Exp::Let {
            var: "x".to_owned(),
            bound_exp: Box::new(Exp::Prim {
                op: Op::Read,
                args: vec![],
            }),
            in_exp: Box::new(Exp::Var("x".to_owned())),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn prim_parse() {
        let (_, result) = parse_prim("(- 4 2)").unwrap();
        let expected = Exp::Prim {
            op: Op::Sub,
            args: vec![Exp::Int(4), Exp::Int(2)],
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn if_parse() {
        let (_, result) = parse_if("(if (= x 2) (* x x) (+ x 1))").unwrap();
        let expected = Exp::If {
            ifc: Box::new(Exp::Prim {
                op: Op::Cmp(Cmp::Equal),
                args: vec![Exp::Var("x".to_owned()), Exp::Int(2)],
            }),
            thenc: Box::new(Exp::Prim {
                op: Op::Mult,
                args: vec![Exp::Var("x".to_owned()), Exp::Var("x".to_owned())],
            }),
            elsec: Box::new(Exp::Prim {
                op: Op::Plus,
                args: vec![Exp::Var("x".to_owned()), Exp::Int(1)],
            }),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn const_parse() {
        let (_, result) = parse_const("1234").unwrap();
        let expected = Exp::Int(1234);
        assert_eq!(result, expected)
    }

    #[test]
    fn bool_parse() {
        let (_, result) = parse_bool("true").unwrap();
        let expected = Exp::Bool(true);
        assert_eq!(result, expected)
    }

    #[test]
    fn var_parse() {
        let (_, result) = parse_var("x").unwrap();
        let expected = Exp::Var("x".to_owned());
        assert_eq!(result, expected)
    }
}
