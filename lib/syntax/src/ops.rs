use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperation {
    Add,
    Sub,
    And,
    Or,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperation {
    Neg,
    Not,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Comparator {
    Eq,
    Lt,
    Leq,
    Gt,
    Geq,
}

impl fmt::Display for BinaryOperation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BinaryOperation::Add => f.write_str("+"),
            BinaryOperation::Sub => f.write_str("-"),
            BinaryOperation::And => f.write_str("&&"),
            BinaryOperation::Or => f.write_str("||"),
        }
    }
}

impl fmt::Display for UnaryOperation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UnaryOperation::Neg => f.write_str("-"),
            UnaryOperation::Not => f.write_str("!"),
        }
    }
}

impl fmt::Display for Comparator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Comparator::Eq => f.write_str("=="),
            Comparator::Lt => f.write_str("<"),
            Comparator::Leq => f.write_str("<="),
            Comparator::Gt => f.write_str(">"),
            Comparator::Geq => f.write_str(">="),
        }
    }
}
