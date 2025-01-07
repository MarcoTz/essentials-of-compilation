use std::fmt;

#[derive(Debug)]
pub enum Type {
    Int,
    Bool,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Int => f.write_str("int"),
            Type::Bool => f.write_str("bool"),
        }
    }
}
