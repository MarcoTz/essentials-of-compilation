use crate::{Error, Rule};
use definitions::{BinaryOperation, Comparator, UnaryOperation};
use pest::iterators::Pair;

pub(crate) fn parse_un_op(pair: Pair<'_, Rule>) -> Result<UnaryOperation, Error> {
    match pair.as_str().trim() {
        "-" => Ok(UnaryOperation::Neg),
        "!" => Ok(UnaryOperation::Not),
        s => Err(Error::unknown(s)),
    }
}

pub(crate) fn parse_bin_op(pair: Pair<'_, Rule>) -> Result<BinaryOperation, Error> {
    match pair.as_str().trim() {
        "+" => Ok(BinaryOperation::Add),
        "-" => Ok(BinaryOperation::Sub),
        "&&" => Ok(BinaryOperation::And),
        "||" => Ok(BinaryOperation::Or),
        s => Err(Error::unknown(s)),
    }
}

pub(crate) fn parse_cmp(pair: Pair<'_, Rule>) -> Result<Comparator, Error> {
    match pair.as_str().trim() {
        "==" => Ok(Comparator::Eq),
        "<" => Ok(Comparator::Lt),
        "<=" => Ok(Comparator::Leq),
        ">" => Ok(Comparator::Gt),
        ">=" => Ok(Comparator::Geq),
        s => Err(Error::unknown(s)),
    }
}
