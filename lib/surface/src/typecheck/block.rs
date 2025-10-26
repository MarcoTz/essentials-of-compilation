use super::{Error, Typecheck};
use crate::syntax::{Block, Type};
use std::collections::HashMap;

impl Typecheck for Block {
    fn check(&self, var_types: &mut HashMap<String, Type>) -> Result<Type, Error> {
        let mut ret_ty = Type::Unit;
        for stmt in self.stmts.iter() {
            ret_ty = stmt.check(var_types)?;
        }
        Ok(ret_ty)
    }
}
