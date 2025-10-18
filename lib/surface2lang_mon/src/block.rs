use super::RemoveComplexOperands;
use std::collections::HashSet;

impl RemoveComplexOperands for surface::Block {
    type Target = lang_mon::Block;
    fn remove_complex_operands(self, used_vars: &mut HashSet<String>) -> Self::Target {
        let mut removed = vec![];
        for stmt in self.stmts {
            removed.extend(stmt.remove_complex_operands(used_vars));
        }
        lang_mon::Block::new(removed)
    }
}
