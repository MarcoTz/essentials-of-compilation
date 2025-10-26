use super::{BlockAccum, Error, ExplicateControl};

impl ExplicateControl for monadic::Block {
    type Target = ();
    fn explicate_control(self, accum: &mut BlockAccum) -> Result<Self::Target, Error> {
        for stmt in self.stmts {
            stmt.explicate_control(accum)?;
        }
        if !accum.current_statements.is_empty() {
            let cont = accum
                .next_cont
                .clone()
                .unwrap_or(core::Continuation::Return(core::Atom::Unit));
            accum.next_block(cont);
        }
        Ok(())
    }
}
