use crate::Type;
use std::collections::HashMap;

mod block;
mod errors;
mod expression;
mod program;
mod statement;
pub use errors::Error;

/// trait used for type checking the language
/// implemented for all kinds of expressions (expression,statement,etc)
pub trait Typecheck {
    /// check self returning the type if successful
    /// var_types contains types of variables in scope
    fn check(&self, var_types: &mut HashMap<String, Type>) -> Result<Type, Error>;
}
