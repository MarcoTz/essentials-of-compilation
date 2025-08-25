use pest::{Parser, iterators::Pair};
use pest_derive::Parser;
use syntax::{
    BinaryOperation, UnaryOperation,
    lang::{Expression, Program},
};

mod errors;

pub use errors::Error;

#[derive(Parser)]
#[grammar = "../lang.pest"]
struct LangParser;

pub fn parse_program(input: &str) -> Result<Program, Error> {
    let mut pairs = LangParser::parse(Rule::program, input)?;
    let prog_pair = pairs.next().ok_or(Error::missing(Rule::program))?;
    if let Some(p) = pairs.next() {
        return Err(Error::remaining(p.as_rule()));
    }

    let mut exps = vec![];
    let mut prog_inner = prog_pair.into_inner();
    while let Some(pair) = prog_inner.next() {
        if pair.as_rule() == Rule::EOI {
            break;
        }
        let exp = parse_expression(pair)?;
        exps.push(exp);
    }
    if let Some(p) = prog_inner.next() {
        return Err(Error::remaining(p.as_rule()));
    }
    Ok(Program::new(exps))
}

fn parse_expression(pair: Pair<'_, Rule>) -> Result<Expression, Error> {
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
        Rule::let_in => parse_let_expr(pair),
        Rule::unary_op => parse_unary_expr(pair),
        Rule::read_int => Ok(Expression::ReadInt),
        Rule::print => parse_print(pair),
        Rule::literal => {
            let num = pair.as_str().trim().parse::<i64>()?;
            Ok(Expression::lit(num))
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
        r => Err(Error::unexpected(r, "Left Recursive Expression")),
    }
}

fn pair_to_n_inner<'a>(
    pair: Pair<'a, Rule>,
    inner_rules: &[Rule],
) -> Result<Vec<Pair<'a, Rule>>, Error> {
    let mut inner = pair.into_inner();
    let mut rules = vec![];
    for next_rule in inner_rules {
        match inner.next() {
            None => return Err(Error::missing(*next_rule)),
            Some(p) => {
                if p.as_rule() == *next_rule {
                    rules.push(p)
                } else {
                    return Err(Error::unexpected(p.as_rule(), &format!("{next_rule:?}")));
                }
            }
        }
    }
    if let Some(p) = inner.next() {
        return Err(Error::remaining(p.as_rule()));
    }
    Ok(rules)
}

fn parse_print(pair: Pair<'_, Rule>) -> Result<Expression, Error> {
    let exp_rule = pair_to_n_inner(pair, &[Rule::expression])?.remove(0);
    let exp = parse_expression(exp_rule)?;
    Ok(Expression::Print(Box::new(exp)))
}

fn parse_let_expr(pair: Pair<'_, Rule>) -> Result<Expression, Error> {
    let mut inner = pair_to_n_inner(pair, &[Rule::variable, Rule::expression])?;
    let var_pair = inner.remove(0);
    let var = var_pair.as_str().trim();
    let bound_pair = inner.remove(0);
    let bound_expr = parse_expression(bound_pair)?;
    Ok(Expression::let_in(var, bound_expr))
}

fn parse_unary_expr(pair: Pair<'_, Rule>) -> Result<Expression, Error> {
    let mut inner = pair_to_n_inner(pair, &[Rule::un_op, Rule::expression])?;
    let op_pair = inner.remove(0);
    let op = parse_un_op(op_pair)?;
    let arg_pair = inner.remove(0);
    let arg_expr = parse_expression(arg_pair)?;
    Ok(Expression::un(arg_expr, op))
}

fn parse_binary_expr(pair: Pair<'_, Rule>, fst: Expression) -> Result<Expression, Error> {
    let mut inner = pair_to_n_inner(pair, &[Rule::bin_op, Rule::expression])?;
    let op_pair = inner.remove(0);
    let op = parse_bin_op(op_pair)?;
    let snd_pair = inner.remove(0);
    let snd_expr = parse_expression(snd_pair)?;
    Ok(Expression::bin(fst, op, snd_expr))
}

fn parse_un_op(pair: Pair<'_, Rule>) -> Result<UnaryOperation, Error> {
    match pair.as_str().trim() {
        "-" => Ok(UnaryOperation::Neg),
        s => Err(Error::unknown(s)),
    }
}

fn parse_bin_op(pair: Pair<'_, Rule>) -> Result<BinaryOperation, Error> {
    match pair.as_str().trim() {
        "+" => Ok(BinaryOperation::Add),
        "-" => Ok(BinaryOperation::Sub),
        s => Err(Error::unknown(s)),
    }
}
