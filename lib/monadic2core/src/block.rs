use super::{BlockAccum, Error, ExplicateControl};

impl ExplicateControl for monadic::Block {
    type Target = ();
    fn explicate_control(self, accum: &mut BlockAccum) -> Result<Self::Target, Error> {
        for stmt in self.stmts {
            stmt.explicate_control(accum)?;
        }
        if !accum.current_statements.is_empty() {
            println!("finishing block from explicate block");
            accum.next_block(core::Continuation::Return(core::Atom::Unit));
        }
        Ok(())
    }
}
