use super::{Error, Typecheck};
use crate::syntax::{Program, Type};
use std::collections::HashMap;

impl Typecheck for Program {
    fn check(&self, var_types: &mut HashMap<String, Type>) -> Result<Type, Error> {
        self.main.check(var_types)?;
        Ok(Type::Unit)
    }
}
