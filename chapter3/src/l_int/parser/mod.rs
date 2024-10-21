pub mod digits;
pub mod errors;
pub mod expr;
pub mod lexer;
pub mod ops;
pub mod stmt;
pub mod tokens;

use super::syntax::Module;
use errors::Error;
use lexer::lex_input;
use std::collections::VecDeque;
use stmt::parse_stmt;

pub fn parse_module(src: String) -> Result<Module, Error> {
    let mut tokens = VecDeque::from(lex_input(src)?);
    let mut stmts = vec![];
    while !tokens.is_empty() {
        let new_stmt = parse_stmt(&mut tokens)?;
        stmts.push(new_stmt);
    }
    Ok(stmts)
}

#[cfg(test)]
mod module_test {
    use super::parse_module;
    use crate::l_int::syntax::{
        exp::Exp,
        ops::{BinOp, UnaryOp},
        stmt::Stmt,
    };

    #[test]
    fn parse_simple() {
        let result = parse_module("print(1)".to_owned()).unwrap();
        let expected = vec![Stmt::Print(Exp::Constant(1))];
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_complex() {
        let result = parse_module("print(3+4)\n -3\n 5-3".to_owned()).unwrap();
        let expected = vec![
            Stmt::Print(Exp::BinOp {
                exp1: Box::new(Exp::Constant(3)),
                op: BinOp::Add,
                exp2: Box::new(Exp::Constant(4)),
            }),
            Stmt::Exp(Exp::UnaryOp {
                op: UnaryOp::Neg,
                exp: Box::new(Exp::Constant(3)),
            }),
            Stmt::Exp(Exp::BinOp {
                exp1: Box::new(Exp::Constant(5)),
                op: BinOp::Sub,
                exp2: Box::new(Exp::Constant(3)),
            }),
        ];
        assert_eq!(result, expected)
    }
}
