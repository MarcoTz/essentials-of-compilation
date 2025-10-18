use super::RemoveComplexOperands;
use definitions::traits::UsedVars;
use std::collections::HashSet;

impl RemoveComplexOperands for surface::Program {
    type Target = lang_mon::Program;
    fn remove_complex_operands(self, used_vars: &mut HashSet<String>) -> Self::Target {
        used_vars.extend(self.used_vars());
        let new_main = self.main.remove_complex_operands(used_vars);
        new_main.into()
    }
}
