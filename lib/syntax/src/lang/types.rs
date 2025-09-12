use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Integer,
    Bool,
    Unit,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Integer => f.write_str("Int"),
            Type::Bool => f.write_str("Bool"),
            Type::Unit => f.write_str("()"),
        }
    }
}
