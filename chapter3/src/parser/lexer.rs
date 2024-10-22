use super::{digits::Digit, errors::Error, ops::Op, tokens::Token};

pub fn lex_input(input: String) -> Result<Vec<Token>, Error> {
    let mut current_index = 0;
    let mut tokens = vec![];
    while current_index < input.len() {
        let next_token = match input.chars().nth(current_index).unwrap() {
            '+' => Token::Op(Op::Plus),
            '-' => Token::Op(Op::Minus),
            'i' => {
                if input.len() >= current_index + 9
                    && input[current_index..current_index + 9] == *"input_int"
                {
                    current_index += 8;
                    Token::InputInt
                } else {
                    return Err(Error::NotAToken { input });
                }
            }
            'p' => {
                if input.len() >= current_index + 5
                    && input[current_index..current_index + 5] == *"print"
                {
                    current_index += 4;
                    Token::Print
                } else {
                    return Err(Error::NotAToken { input });
                }
            }
            '(' => Token::BrackO,
            ')' => Token::BrackC,
            ' ' => Token::Sep,
            '\n' => Token::Sep,
            c => {
                let d = c
                    .to_string()
                    .parse::<Digit>()
                    .map_err(|_| Error::NotAToken {
                        input: input.clone(),
                    })?;
                Token::Digit(d)
            }
        };
        tokens.push(next_token);
        current_index += 1;
    }
    Ok(tokens)
}

#[cfg(test)]
mod lexer_tests {
    use super::{lex_input, Digit, Op, Token};

    #[test]
    fn lex_plus() {
        let result = lex_input("+".to_owned()).unwrap();
        let expected = vec![Token::Op(Op::Plus)];
        assert_eq!(result, expected)
    }

    #[test]
    fn lex_minus() {
        let result = lex_input("-".to_owned()).unwrap();
        let expected = vec![Token::Op(Op::Minus)];
        assert_eq!(result, expected)
    }

    #[test]
    fn lex_inputint() {
        let result = lex_input("input_int".to_owned()).unwrap();
        let expected = vec![Token::InputInt];
        assert_eq!(result, expected)
    }

    #[test]
    fn lex_inputint_err() {
        let result = lex_input("input_in".to_owned());
        assert!(result.is_err())
    }

    #[test]
    fn lex_print() {
        let result = lex_input("print".to_owned()).unwrap();
        let expected = vec![Token::Print];
        assert_eq!(result, expected)
    }

    #[test]
    fn lex_print_err() {
        let result = lex_input("prin".to_owned());
        assert!(result.is_err())
    }

    #[test]
    fn lex_bracko() {
        let result = lex_input("(".to_owned()).unwrap();
        let expected = vec![Token::BrackO];
        assert_eq!(result, expected)
    }

    #[test]
    fn lex_bracc() {
        let result = lex_input(")".to_owned()).unwrap();
        let expected = vec![Token::BrackC];
        assert_eq!(result, expected)
    }

    #[test]
    fn lex_space() {
        let result = lex_input(" ".to_owned()).unwrap();
        let expected = vec![Token::Sep];
        assert_eq!(result, expected)
    }

    #[test]
    fn lex_newline() {
        let result = lex_input("\n".to_owned()).unwrap();
        let expected = vec![Token::Sep];
        assert_eq!(result, expected)
    }

    #[test]
    fn lex_digit() {
        let result = lex_input("5".to_owned()).unwrap();
        let expected = vec![Token::Digit(Digit::Five)];
        assert_eq!(result, expected)
    }

    #[test]
    fn lex_multiple() {
        let result = lex_input("input_int + 4".to_owned()).unwrap();
        let expected = vec![
            Token::InputInt,
            Token::Sep,
            Token::Op(Op::Plus),
            Token::Sep,
            Token::Digit(Digit::Four),
        ];
        assert_eq!(result, expected)
    }
}
