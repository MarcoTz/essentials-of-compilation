use super::errors::Error;

pub fn consume_whitespace(input: &mut String) {
    let next = input.chars().nth(0);
    if next == Some(' ') {
        input.remove(0);
        consume_whitespace(input);
    }
}

pub fn consume_character(input: &mut String, c: char) -> Result<(), Error> {
    if input.is_empty() {
        return Err(Error::UnexpectedEOI);
    }
    let next_char = input.remove(0);
    if next_char == c {
        Ok(())
    } else {
        Err(Error::UnexpectedCharacter(next_char))
    }
}

pub fn consume_sequence(input: &mut String, chars: &str) -> Result<(), Error> {
    for c in chars.chars() {
        consume_character(input, c)?;
    }
    Ok(())
}

#[cfg(test)]
mod lexer_tests {
    use super::{consume_character, consume_sequence, consume_whitespace};

    #[test]
    fn consume_space() {
        let mut result = " ".to_owned();
        consume_whitespace(&mut result);
        let expected = "";
        assert_eq!(result, expected)
    }

    #[test]
    fn consume_spaces() {
        let mut result = "    a ".to_owned();
        consume_whitespace(&mut result);
        let expected = "a ";
        assert_eq!(result, expected)
    }

    #[test]
    fn consume_nospace() {
        let mut result = "abc".to_owned();
        consume_whitespace(&mut result);
        let expected = "abc";
        assert_eq!(result, expected)
    }

    #[test]
    fn consume_single() {
        let mut result = "a".to_owned();
        consume_character(&mut result, 'a').unwrap();
        assert_eq!(result, "")
    }

    #[test]
    fn consume_single_fail() {
        let mut s = "a".to_owned();
        let result = consume_character(&mut s, 'b');
        assert!(result.is_err())
    }

    #[test]
    fn consume_mult() {
        let mut result = "read".to_owned();
        consume_sequence(&mut result, "read").unwrap();
        assert_eq!(result, "")
    }

    #[test]
    fn consume_mult_fail() {
        let mut s = "notread".to_owned();
        let result = consume_sequence(&mut s, "read");
        assert!(result.is_err())
    }
}
