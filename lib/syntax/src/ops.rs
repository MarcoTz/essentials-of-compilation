use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum BinaryOperation {
    Add,
    Sub,
}

#[derive(Debug, Clone, Copy)]
pub enum UnaryOperation {
    Neg,
}

impl fmt::Display for BinaryOperation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BinaryOperation::Add => f.write_str("+"),
            BinaryOperation::Sub => f.write_str("-"),
        }
    }
}

impl fmt::Display for UnaryOperation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UnaryOperation::Neg => f.write_str("-"),
        }
    }
}
