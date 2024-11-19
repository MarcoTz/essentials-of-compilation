use super::{UniqueState, Uniquify};
use crate::l_var::syntax::Program;

impl Uniquify for Program {
    type Target = Program;
    fn uniquify(self, st: &mut UniqueState) -> Self::Target {
        Program {
            exp: self.exp.uniquify(st),
        }
    }
}
