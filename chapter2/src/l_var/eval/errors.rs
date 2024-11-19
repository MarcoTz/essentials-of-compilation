use crate::l_var::syntax::Var;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    VarNotFound { name: Var },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::VarNotFound { name } => {
                write!(f, "Could not find variable {name} in environment.")
            }
        }
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod error_tests {
    use super::Error;

    #[test]
    fn display_not_found() {
        let result = format!(
            "{}",
            Error::VarNotFound {
                name: "x".to_owned()
            }
        );
        let expected = "Could not find variable x in environment.";
        assert_eq!(result, expected)
    }
}
