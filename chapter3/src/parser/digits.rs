use super::errors::Error;
use std::{fmt, str::FromStr};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Digit {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Zero,
}

impl fmt::Display for Digit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Digit::One => f.write_str("1"),
            Digit::Two => f.write_str("2"),
            Digit::Three => f.write_str("3"),
            Digit::Four => f.write_str("4"),
            Digit::Five => f.write_str("5"),
            Digit::Six => f.write_str("6"),
            Digit::Seven => f.write_str("7"),
            Digit::Eight => f.write_str("8"),
            Digit::Nine => f.write_str("9"),
            Digit::Zero => f.write_str("0"),
        }
    }
}

impl FromStr for Digit {
    type Err = Error;
    fn from_str(s: &str) -> Result<Digit, Self::Err> {
        match s.trim() {
            "1" => Ok(Digit::One),
            "2" => Ok(Digit::Two),
            "3" => Ok(Digit::Three),
            "4" => Ok(Digit::Four),
            "5" => Ok(Digit::Five),
            "6" => Ok(Digit::Six),
            "7" => Ok(Digit::Seven),
            "8" => Ok(Digit::Eight),
            "9" => Ok(Digit::Nine),
            "0" => Ok(Digit::Zero),
            _ => Err(Error::NotADigit {
                input: s.to_owned(),
            }),
        }
    }
}

impl From<Digit> for i32 {
    fn from(d: Digit) -> i32 {
        match d {
            Digit::Zero => 0,
            Digit::One => 1,
            Digit::Two => 2,
            Digit::Three => 3,
            Digit::Four => 4,
            Digit::Five => 5,
            Digit::Six => 6,
            Digit::Seven => 7,
            Digit::Eight => 8,
            Digit::Nine => 9,
        }
    }
}
