use super::{errors::Error, expr::parse_exp, tokens::Token};
use crate::syntax::Stmt;
use std::collections::VecDeque;

pub fn parse_stmt(tokens: &mut VecDeque<Token>) -> Result<Stmt, Error> {
    match tokens.pop_front() {
        None => Err(Error::UnexpectedEndOfInput),
        Some(Token::Print) => {
            let _ = if let Some(Token::BrackO) = tokens.pop_front() {
                Ok(())
            } else {
                Err(Error::BracketMismatch)
            }?;

            let exp = parse_exp(tokens)?;

            let _ = if let Some(Token::BrackC) = tokens.pop_front() {
                Ok(())
            } else {
                Err(Error::BracketMismatch)
            }?;
            Ok(Stmt::Print(exp))
        }
        Some(tk) => {
            tokens.push_front(tk);
            let exp = parse_exp(tokens)?;
            Ok(Stmt::Exp(exp))
        }
    }
}

#[cfg(test)]
mod stmt_tests {
    use super::{parse_stmt, Stmt, Token};
    use crate::{parser::digits::Digit, syntax::Exp};
    use std::collections::VecDeque;

    #[test]
    fn parse_print() {
        let result = parse_stmt(&mut VecDeque::from([
            Token::Print,
            Token::BrackO,
            Token::Digit(Digit::One),
            Token::BrackC,
        ]))
        .unwrap();
        let expected = Stmt::Print(Exp::Constant(1));
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_print_no_brack() {
        let result = parse_stmt(&mut VecDeque::from([
            Token::Print,
            Token::Digit(Digit::One),
        ]));
        assert!(result.is_err())
    }

    #[test]
    fn parse_exp() {
        let result = parse_stmt(&mut VecDeque::from([Token::Digit(Digit::Two)])).unwrap();
        let expected = Stmt::Exp(Exp::Constant(2));
        assert_eq!(result, expected)
    }
}
