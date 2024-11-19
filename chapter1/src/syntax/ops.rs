use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum UnaryOp {
    Neg,
}

#[derive(Debug, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Sub,
}
impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UnaryOp::Neg => f.write_str("-"),
        }
    }
}

impl fmt::Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BinOp::Add => f.write_str("+"),
            BinOp::Sub => f.write_str("-"),
        }
    }
}

#[cfg(test)]
mod ops_test {
    use super::{BinOp, UnaryOp};

    #[test]
    fn display_unary() {
        let result = format!("{}", UnaryOp::Neg);
        let expected = "-";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_add() {
        let result = format!("{}", BinOp::Add);
        let expected = "+";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_sub() {
        let result = format!("{}", BinOp::Sub);
        let expected = "-";
        assert_eq!(result, expected)
    }
}
