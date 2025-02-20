use super::Token;
use crate::errors::Error;

pub fn lex(input: &str) -> Result<Vec<Token>, Error> {
    let mut tokens = vec![];
    let mut input = input.trim();

    while !input.is_empty() {
        let (c, rest) = input.split_at(1);
        match c {
            "(" => {
                tokens.push(Token::ParensO);
                input = rest.trim();
            }
            ")" => {
                tokens.push(Token::ParensC);
                input = rest.trim();
            }
            _ => {
                let sym = get_symbol(input);
                input = input.split_at(sym.len()).1.trim();
                tokens.push(Token::Symbol(sym));
            }
        }
    }

    Ok(tokens)
}

fn get_symbol(input: &str) -> String {
    let mut sym = "".to_owned();
    for c in input.chars() {
        if c.is_whitespace() || c == '(' || c == ')' {
            break;
        }
        sym.push(c);
    }
    sym
}

#[cfg(test)]
mod lexer_tests {
    use super::{lex, Token};

    #[test]
    fn lex_sym() {
        let result = lex("Nil").unwrap();
        let expected = vec![Token::Symbol("Nil".to_owned())];
        assert_eq!(result, expected)
    }

    #[test]
    fn lex_brack() {
        let result = lex("((Nil))").unwrap();
        let expected = vec![
            Token::ParensO,
            Token::ParensO,
            Token::Symbol("Nil".to_owned()),
            Token::ParensC,
            Token::ParensC,
        ];
        assert_eq!(result, expected)
    }

    #[test]
    fn lex_list() {
        let result = lex("(1 (2 ( Nil )))").unwrap();
        let expected = vec![
            Token::ParensO,
            Token::Symbol("1".to_owned()),
            Token::ParensO,
            Token::Symbol("2".to_owned()),
            Token::ParensO,
            Token::Symbol("Nil".to_owned()),
            Token::ParensC,
            Token::ParensC,
            Token::ParensC,
        ];
        assert_eq!(result, expected)
    }

    #[test]
    fn lex_tree() {
        let result = lex("(1 (3 (5) 4) 2)").unwrap();
        let expected = vec![
            Token::ParensO,
            Token::Symbol("1".to_owned()),
            Token::ParensO,
            Token::Symbol("3".to_owned()),
            Token::ParensO,
            Token::Symbol("5".to_owned()),
            Token::ParensC,
            Token::Symbol("4".to_owned()),
            Token::ParensC,
            Token::Symbol("2".to_owned()),
            Token::ParensC,
        ];
        assert_eq!(result, expected)
    }
}
