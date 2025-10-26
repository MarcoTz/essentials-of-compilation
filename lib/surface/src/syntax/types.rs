use crate::typecheck::Error;
use std::fmt;

/// Types of the surface language
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    /// Generic type, so far only used for errors
    Variable,
    /// Integers
    Integer,
    /// Boolean
    Bool,
    /// Unit Type
    Unit,
    /// Tuple Type
    Tuple(Vec<Type>),
    /// Reference Type
    Reference(Box<Type>),
}

impl Type {
    /// Try to convert the type to a tuple type, returning the inner types
    pub fn as_tuple(self) -> Result<Vec<Type>, Error> {
        if let Type::Tuple(inner) = self {
            Ok(inner)
        } else {
            Err(Error::mismatch(self, Type::Tuple(vec![Type::Variable])))
        }
    }

    /// Try to convert the type to a reference type, returning the inner type
    pub fn as_ref(self) -> Result<Type, Error> {
        if let Type::Reference(inner) = self {
            Ok(*inner)
        } else {
            Err(Error::mismatch(
                self,
                Type::Reference(Box::new(Type::Variable)),
            ))
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Variable => f.write_str("_"),
            Type::Integer => f.write_str("Int"),
            Type::Bool => f.write_str("Bool"),
            Type::Unit => f.write_str("()"),
            Type::Tuple(tys) => write!(
                f,
                "({})",
                tys.iter()
                    .map(|ty| ty.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            Type::Reference(ty) => write!(f, "&{ty}"),
        }
    }
}
