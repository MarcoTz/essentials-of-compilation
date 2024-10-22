use super::{digits::Digit, errors::Error, ops::Op, tokens::Token};
use crate::syntax::{BinOp, Exp, UnaryOp};
use std::collections::VecDeque;

pub fn parse_exp(tokens: &mut VecDeque<Token>) -> Result<Exp, Error> {
    println!("parsing tokens {tokens:?}");
    let exp = match tokens.pop_front() {
        None => Err(Error::UnexpectedEndOfInput),
        Some(Token::Op(Op::Minus)) => {
            let exp2 = parse_exp(tokens)?;
            Ok(Exp::UnaryOp {
                exp: Box::new(exp2),
                op: UnaryOp::Neg,
            })
        }
        Some(Token::Digit(d)) => {
            let mut digits = vec![d];
            let mut next_token = tokens.pop_front();
            let mut done = false;
            while !done {
                if let Some(Token::Digit(dg)) = &next_token {
                    digits.push(dg.clone());
                    next_token = tokens.pop_front();
                } else if let Some(tk) = &next_token {
                    tokens.push_front(tk.clone());
                    done = true;
                } else {
                    done = true;
                }
            }
            let num: i32 = digits.iter().enumerate().fold(0, |sum, (i, next_digit)| {
                sum + <Digit as Into<i32>>::into(next_digit.clone())
                    * 10_i32.pow((digits.len() - i) as u32 - 1)
            });
            Ok(Exp::Constant(num))
        }
        Some(Token::InputInt) => Ok(Exp::InputInt),
        Some(Token::Print) => Err(Error::NotAnExpr {
            token: Token::Print,
        }),
        Some(Token::BrackO) => match tokens
            .iter()
            .enumerate()
            .find(|(_, tk)| **tk == Token::BrackC)
        {
            None => Err(Error::BracketMismatch),
            Some((i, _)) => {
                let mut inner_tokens = tokens.split_off(i - 1);
                tokens.remove(i);
                let exp = parse_exp(&mut inner_tokens)?;
                Ok(exp)
            }
        },
        Some(Token::BrackC) => Err(Error::BracketMismatch),
        Some(Token::Op(Op::Plus)) => Err(Error::UnexpectedToken {
            token: Token::Op(Op::Plus),
        }),
        Some(Token::Sep) => parse_exp(tokens),
    }?;

    let next_token = tokens.pop_front();
    if let Some(Token::Op(op)) = next_token {
        let exp2 = parse_exp(tokens)?;
        Ok(Exp::BinOp {
            exp1: Box::new(exp),
            op: match op {
                Op::Plus => BinOp::Add,
                Op::Minus => BinOp::Sub,
            },
            exp2: Box::new(exp2),
        })
    } else if let Some(tk) = next_token {
        tokens.push_front(tk);
        Ok(exp)
    } else {
        Ok(exp)
    }
}

#[cfg(test)]
mod expr_tests {
    use super::{parse_exp, BinOp, Digit, Exp, Op, Token, UnaryOp};
    use std::collections::VecDeque;

    #[test]
    fn parse_unary() {
        let result = parse_exp(&mut VecDeque::from([
            Token::Op(Op::Minus),
            Token::Digit(Digit::Three),
        ]))
        .unwrap();
        let expected = Exp::UnaryOp {
            op: UnaryOp::Neg,
            exp: Box::new(Exp::Constant(3)),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_num() {
        let result = parse_exp(&mut VecDeque::from([
            Token::Digit(Digit::One),
            Token::Digit(Digit::Two),
            Token::Digit(Digit::Three),
        ]))
        .unwrap();
        let expected = Exp::Constant(123);
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_inputint() {
        let result = parse_exp(&mut VecDeque::from([Token::InputInt])).unwrap();
        let expected = Exp::InputInt;
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_bracket() {
        let result = parse_exp(&mut VecDeque::from([
            Token::BrackO,
            Token::Digit(Digit::One),
            Token::BrackC,
        ]))
        .unwrap();
        let expected = Exp::Constant(1);
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_binop() {
        let result = parse_exp(&mut VecDeque::from([
            Token::Digit(Digit::One),
            Token::Op(Op::Plus),
            Token::Digit(Digit::Two),
        ]))
        .unwrap();
        let expected = Exp::BinOp {
            exp1: Box::new(Exp::Constant(1)),
            op: BinOp::Add,
            exp2: Box::new(Exp::Constant(2)),
        };
        assert_eq!(result, expected)
    }
}
