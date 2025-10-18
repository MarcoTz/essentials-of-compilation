use crate::{
    Error, Rule, pair_to_n_inner,
    symbols::{parse_bin_op, parse_cmp, parse_un_op},
};
use pest::iterators::Pair;
use surface::Expression;

pub fn parse_expression(pair: Pair<'_, Rule>) -> Result<Expression, Error> {
    let mut inner = pair.into_inner();
    let prim_pair = inner.next().ok_or(Error::missing(Rule::prim_expression))?;
    let mut prim_inner = prim_pair.into_inner();
    let inner_pair = prim_inner
        .next()
        .ok_or(Error::missing(Rule::prim_expression))?;
    if let Some(p) = prim_inner.next() {
        return Err(Error::remaining(p.as_rule()));
    }
    let prim_expr = parse_prim_expression(inner_pair)?;

    let expr = match inner.next() {
        None => prim_expr,
        Some(left_rec_pair) => {
            let mut left_rec_inner = left_rec_pair.into_inner();
            let inner_pair = left_rec_inner
                .next()
                .ok_or(Error::missing(Rule::left_rec_expression))?;
            if let Some(p) = left_rec_inner.next() {
                return Err(Error::remaining(p.as_rule()));
            }
            parse_leftrec_expression(inner_pair, prim_expr)?
        }
    };

    if let Some(p) = inner.next() {
        return Err(Error::remaining(p.as_rule()));
    }
    Ok(expr)
}

fn parse_prim_expression(pair: Pair<'_, Rule>) -> Result<Expression, Error> {
    match pair.as_rule() {
        Rule::paren_exp => {
            let expr_pair = pair_to_n_inner(pair, &[Rule::expression])?.remove(0);
            parse_expression(expr_pair)
        }
        Rule::unary_op => parse_unary_expr(pair),
        Rule::read_int => Ok(Expression::ReadInt),
        Rule::literal => {
            let num = pair.as_str().trim().parse::<i64>()?;
            Ok(Expression::lit(num))
        }
        Rule::bool => {
            let b = pair.as_str().trim().parse::<bool>()?;
            Ok(Expression::bool(b))
        }
        Rule::variable => {
            let var = pair.as_str().trim();
            Ok(Expression::var(var))
        }
        r => Err(Error::unexpected(r, "Non Left-recursive Expression")),
    }
}

fn parse_leftrec_expression(pair: Pair<'_, Rule>, expr: Expression) -> Result<Expression, Error> {
    match pair.as_rule() {
        Rule::binary_op => parse_binary_expr(pair, expr),
        Rule::cmp_op => parse_cmp_expr(pair, expr),
        r => Err(Error::unexpected(r, "Left Recursive Expression")),
    }
}
fn parse_binary_expr(pair: Pair<'_, Rule>, fst: Expression) -> Result<Expression, Error> {
    let mut inner = pair_to_n_inner(pair, &[Rule::bin_op, Rule::expression])?;
    let op_pair = inner.remove(0);
    let op = parse_bin_op(op_pair)?;
    let snd_pair = inner.remove(0);
    let snd_expr = parse_expression(snd_pair)?;
    Ok(Expression::bin(fst, op, snd_expr))
}
fn parse_unary_expr(pair: Pair<'_, Rule>) -> Result<Expression, Error> {
    let mut inner = pair_to_n_inner(pair, &[Rule::un_op, Rule::expression])?;
    let op_pair = inner.remove(0);
    let op = parse_un_op(op_pair)?;
    let arg_pair = inner.remove(0);
    let arg_expr = parse_expression(arg_pair)?;
    Ok(Expression::un(arg_expr, op))
}
fn parse_cmp_expr(pair: Pair<'_, Rule>, left: Expression) -> Result<Expression, Error> {
    let mut inner = pair_to_n_inner(pair, &[Rule::cmp, Rule::expression])?;
    let cmp_pair = inner.remove(0);
    let cmp = parse_cmp(cmp_pair)?;
    let right_pair = inner.remove(0);
    let right = parse_expression(right_pair)?;
    Ok(Expression::cmp(left, cmp, right))
}
