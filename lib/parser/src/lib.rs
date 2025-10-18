use pest::{Parser, iterators::Pair};
use pest_derive::Parser;
use surface::Program;

mod errors;
mod expressions;
mod statements;
mod symbols;
use expressions::parse_expression;
use statements::parse_statement;

pub use errors::Error;

#[derive(Parser)]
#[grammar = "../lang.pest"]
pub struct LangParser;

pub fn parse_program(input: &str) -> Result<Program, Error> {
    let mut pairs = LangParser::parse(Rule::program, input)?;
    let prog_pair = pairs.next().ok_or(Error::missing(Rule::program))?;
    if let Some(p) = pairs.next() {
        return Err(Error::remaining(p.as_rule()));
    }

    let mut stmts = vec![];
    let mut prog_inner = prog_pair.into_inner();
    for pair in prog_inner.by_ref() {
        if pair.as_rule() == Rule::EOI {
            break;
        }
        let mut stmt_inner = pair.into_inner();
        let stmt_pair = stmt_inner.next().ok_or(Error::missing(Rule::statement))?;
        if let Some(n) = stmt_inner.next() {
            return Err(Error::remaining(n.as_rule()));
        }
        let stmt = parse_statement(stmt_pair)?;
        stmts.push(stmt);
    }
    if let Some(p) = prog_inner.next() {
        return Err(Error::remaining(p.as_rule()));
    }
    Ok(Program::new(stmts))
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
