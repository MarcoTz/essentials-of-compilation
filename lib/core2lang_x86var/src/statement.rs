use super::SelectInstructions;
use definitions::PRINT_CALL;

impl SelectInstructions for core::Statement {
    type Target = Vec<asm::Instruction<asm::VarArg>>;
    type Arg = ();
    fn select_instructions(self, _: Self::Arg) -> Self::Target {
        match self {
            core::Statement::Assign { var, bound } => {
                bound.select_instructions(asm::VarArg::Var(var))
            }
            core::Statement::Print(atm) => {
                let arg_loc = atm.select_instructions(());
                let mov = asm::Instruction::MovQ {
                    src: arg_loc,
                    dest: asm::Reg::Rdi.into(),
                };
                let print = asm::Instruction::CallQ {
                    label: PRINT_CALL.to_owned(),
                };
                vec![mov, print]
            }
        }
    }
}
