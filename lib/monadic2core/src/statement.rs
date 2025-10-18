use super::{BlockAccum, Error, ExplicateControl};

impl ExplicateControl for monadic::Statement {
    type Target = ();
    fn explicate_control(self, state: &mut BlockAccum) -> Result<(), Error> {
        match self {
            monadic::Statement::Return(atm) => {
                let cont = core::Continuation::Return(atm.explicate_control(state)?);
                println!("finishing block from return statement");
                state.next_block(cont);
                Ok(())
            }
            monadic::Statement::Print(atm) => {
                let stmt = core::Statement::Print(atm.explicate_control(state)?);
                state.push_stmt(stmt);
                Ok(())
            }
            monadic::Statement::Assign { var, bound } => {
                let bound_exp = bound.explicate_control(state)?;
                state.push_stmt(core::Statement::assign(&var, bound_exp));
                Ok(())
            }
            monadic::Statement::If {
                cond_exp,
                then_block,
                else_block,
            } => {
                let cond = cond_exp.explicate_control(state)?;
                let then_label = state.fresh_label();
                let else_label = state.fresh_label();
                let cont = core::Continuation::If {
                    cond,
                    then_label: then_label.clone(),
                    else_label: else_label.clone(),
                };
                println!("finishing block from if statement");
                state.next_block(cont);
                println!("explicating then block with label {then_label}");
                state.current_label = then_label;
                then_block.explicate_control(state)?;
                println!("explicating else block with label {else_label}");
                state.current_label = else_label;
                else_block.explicate_control(state)?;
                Ok(())
            }
        }
    }
}
