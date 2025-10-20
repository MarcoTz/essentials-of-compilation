use crate::{Error, Rule, pair_to_n_inner, parse_expression};
use pest::iterators::Pair;
use surface::{Block, Statement};

pub fn parse_statement(pair: Pair<'_, Rule>) -> Result<Statement, Error> {
    match pair.as_rule() {
        Rule::paren_statement => {
            let stmt_pair = pair_to_n_inner(pair, &[Rule::statement])?.remove(0);
            parse_statement(stmt_pair)
        }
        Rule::if_statement => parse_if(pair),
        Rule::while_statement => parse_while(pair),
        Rule::print_statement => parse_print(pair),
        Rule::let_statement => parse_let(pair),
        r => Err(Error::unexpected(r, "Statement")),
    }
}

fn parse_if(pair: Pair<'_, Rule>) -> Result<Statement, Error> {
    let mut then_stmts = vec![];
    let mut else_stmts = vec![];
    let mut inner = pair.into_inner();
    let cond_pair = inner.next().ok_or(Error::missing(Rule::expression))?;
    let cond_expr = parse_expression(cond_pair)?;
    let mut current_then = true;
    for next in inner {
        if next.as_rule() == Rule::else_start {
            current_then = false;
            continue;
        }
        let mut next_inner = next.into_inner();
        let stmt_rule = next_inner.next().ok_or(Error::missing(Rule::statement))?;
        if let Some(n) = next_inner.next() {
            return Err(Error::remaining(n.as_rule()));
        }
        let stmt = parse_statement(stmt_rule)?;
        if current_then {
            then_stmts.push(stmt);
        } else {
            else_stmts.push(stmt);
        }
    }

    Ok(Statement::cond(
        cond_expr,
        Block::new(then_stmts),
        Block::new(else_stmts),
    ))
}

fn parse_while(pair: Pair<'_, Rule>) -> Result<Statement, Error> {
    let mut inner = pair.into_inner();
    let exp_pair = inner.next().ok_or(Error::missing(Rule::expression))?;
    let cond_exp = parse_expression(exp_pair)?;
    let mut stmts = vec![];
    for next in inner {
        let mut next_inner = next.into_inner();
        let stmt_rule = next_inner.next().ok_or(Error::missing(Rule::statement))?;
        if let Some(n) = next_inner.next() {
            return Err(Error::remaining(n.as_rule()));
        }
        let stmt = parse_statement(stmt_rule)?;
        stmts.push(stmt);
    }
    Ok(Statement::While {
        cond_exp,
        while_block: Block::new(stmts),
    })
}

fn parse_print(pair: Pair<'_, Rule>) -> Result<Statement, Error> {
    let exp_rule = pair_to_n_inner(pair, &[Rule::expression])?.remove(0);
    let exp = parse_expression(exp_rule)?;
    Ok(Statement::Print(exp))
}

fn parse_let(pair: Pair<'_, Rule>) -> Result<Statement, Error> {
    let mut inner = pair_to_n_inner(pair, &[Rule::variable, Rule::expression])?;
    let var_pair = inner.remove(0);
    let var = var_pair.as_str().trim();
    let bound_pair = inner.remove(0);
    let bound_expr = parse_expression(bound_pair)?;
    Ok(Statement::assign(var, bound_expr))
}
