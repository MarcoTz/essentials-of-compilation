use super::{errors::Error, s_exp::SExp, Symbol};
use std::{collections::VecDeque, fmt};

mod lexer;
use lexer::lex;

#[derive(Debug, PartialEq, Eq)]
enum Token {
    ParensO,
    ParensC,
    Symbol(String),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::ParensO => f.write_str("("),
            Token::ParensC => f.write_str(")"),
            Token::Symbol(s) => f.write_str(&s),
        }
    }
}

pub fn parse_sexpr<T: Symbol>(input: &str) -> Result<SExp<T>, Error> {
    let tokens: VecDeque<Token> = lex(input)?.into();
    parse_tokens(tokens)
}

fn parse_tokens<T: Symbol>(mut tokens: VecDeque<Token>) -> Result<SExp<T>, Error> {
    let next = match tokens.pop_front() {
        None => return Err(Error::UnexpectedEOI),
        Some(n) => n,
    };

    match next {
        Token::ParensO => {
            let end = tokens.pop_back();
            if end.is_none() {
                return Err(Error::UnexpectedEOI);
            } else if end != Some(Token::ParensC) {
                return Err(Error::MissingParens);
            }

            let tokens_split = split_parens(tokens)?;
            if tokens_split.is_empty() {
                return Err(Error::UnexpectedEOI);
            }
            let exprs = tokens_split
                .into_iter()
                .map(|tok| parse_tokens(tok))
                .collect::<Result<Vec<SExp<T>>, Error>>()?;
            Ok(SExp::Expr(exprs))
        }
        Token::Symbol(s) => {
            let sym = s.parse::<T>()?;
            Ok(SExp::Symbol(sym))
        }
        Token::ParensC => Err(Error::RemainingInput(next.to_string())),
    }
}

fn split_parens(mut tokens: VecDeque<Token>) -> Result<Vec<VecDeque<Token>>, Error> {
    let next = match tokens.pop_front() {
        None => return Ok(vec![]),
        Some(t) => t,
    };

    match next {
        Token::ParensO => {
            let close_ind = next_close_parens(&tokens)?;
            let mut rest = tokens.split_off(close_ind);
            rest.pop_front();
            let mut res = split_parens(rest)?;
            tokens.push_front(Token::ParensO);
            tokens.push_back(Token::ParensC);
            res.insert(0, tokens);
            Ok(res)
        }
        Token::ParensC => Err(Error::UnexpectedSymbol(next.to_string())),
        Token::Symbol(_) => {
            let mut rest = split_parens(tokens)?;
            rest.insert(0, VecDeque::from([next]));
            Ok(rest)
        }
    }
}

fn next_close_parens(tokens: &VecDeque<Token>) -> Result<usize, Error> {
    let mut num_parens = 0;
    for (ind, token) in tokens.iter().enumerate() {
        match token {
            Token::ParensO => num_parens += 1,
            Token::ParensC => {
                if num_parens == 0 {
                    return Ok(ind);
                } else {
                    num_parens -= 1
                }
            }
            Token::Symbol(_) => (),
        }
    }
    Err(Error::MissingParens)
}

#[cfg(test)]
mod parse_tests {
    use super::parse_sexpr;
    use crate::{SExp, StrSym};

    #[test]
    fn parse_nil() {
        let result = parse_sexpr::<StrSym>("Nil").unwrap();
        let expected = SExp::Symbol("Nil".into());
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_list() {
        let result = parse_sexpr::<StrSym>("(1 ( 2 ( 3  Nil)))").unwrap();
        let expected = SExp::Expr(vec![
            SExp::Symbol("1".into()),
            SExp::Expr(vec![
                SExp::Symbol("2".into()),
                SExp::Expr(vec![SExp::Symbol("3".into()), SExp::Symbol("Nil".into())]),
            ]),
        ]);
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_tree() {
        let result = parse_sexpr::<StrSym>("(1 (3 (5) 4) 2)").unwrap();
        let expected = SExp::Expr(vec![
            SExp::Symbol("1".into()),
            SExp::Expr(vec![
                SExp::Symbol("3".into()),
                SExp::Expr(vec![SExp::Symbol("5".into())]),
                SExp::Symbol("4".into()),
            ]),
            SExp::Symbol("2".into()),
        ]);
        assert_eq!(result, expected)
    }
}
