use super::{
    errors::ParseRes,
    keywords::{parse_keyword, Keyword},
};
use crate::l_if::syntax::{Cmp, Op};
use nom::{branch::alt, bytes::complete::tag};

pub fn parse_op(input: &str) -> ParseRes<Op> {
    alt((
        |s| parse_keyword(s, Keyword::Read).map(|(rem, _)| (rem, Op::Read)),
        |s| parse_keyword(s, Keyword::And).map(|(rem, _)| (rem, Op::And)),
        |s| parse_keyword(s, Keyword::Or).map(|(rem, _)| (rem, Op::Or)),
        |s| parse_keyword(s, Keyword::Not).map(|(rem, _)| (rem, Op::Not)),
        |s| tag("+")(s).map(|(rem, _)| (rem, Op::Plus)),
        |s| tag("-")(s).map(|(rem, _)| (rem, Op::Neg)),
        |s| tag("*")(s).map(|(rem, _)| (rem, Op::Mult)),
        |s| parse_cmp(s).map(|(rem, cmp)| (rem, cmp.into())),
    ))(input)
}

fn parse_cmp(input: &str) -> ParseRes<Cmp> {
    alt((
        |s| tag("=")(s).map(|(rem, _)| (rem, Cmp::Equal)),
        |s| tag("<")(s).map(|(rem, _)| (rem, Cmp::Less)),
        |s| tag("<=")(s).map(|(rem, _)| (rem, Cmp::LessEq)),
        |s| tag(">")(s).map(|(rem, _)| (rem, Cmp::Greater)),
        |s| tag(">=")(s).map(|(rem, _)| (rem, Cmp::GreaterEq)),
    ))(input)
}

#[cfg(test)]
mod ops_tests {
    use super::{parse_op, Op};

    #[test]
    fn parse_read() {
        let (_, result) = parse_op("read").unwrap();
        let expected = Op::Read;
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_and() {
        let (_, result) = parse_op("and").unwrap();
        let expected = Op::And;
        assert_eq!(result, expected)
    }
    #[test]
    fn parse_or() {
        let (_, result) = parse_op("or").unwrap();
        let expected = Op::Or;
        assert_eq!(result, expected)
    }
    #[test]
    fn parse_not() {
        let (_, result) = parse_op("not").unwrap();
        let expected = Op::Not;
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_plus() {
        let (_, result) = parse_op("+").unwrap();
        let expected = Op::Plus;
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_neg() {
        let (_, result) = parse_op("-").unwrap();
        let expected = Op::Neg;
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_mult() {
        let (_, result) = parse_op("*").unwrap();
        let expected = Op::Mult;
        assert_eq!(result, expected)
    }
}
