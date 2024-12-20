use super::PatchInstructions;
use chapter2::x86_int::{Arg, Instr};

impl PatchInstructions for Instr {
    type Target = Vec<Instr>;
    fn patch(self) -> Self::Target {
        let args: Vec<Arg> = todo!(); //self.get_args();
        if args.len() == 2 && args.first() == args.get(1) {
            return vec![];
        }

        let max_immediate = 2_i64.pow(16);
        let no_double_deref: Vec<Instr> = todo!(); //self.remove_double_deref();
        let no_max_immediate: Vec<Instr> =
            no_double_deref
                .into_iter()
                .fold(vec![], |instrs, next_instr| {
                    let mut new_instrs = instrs;
                    todo!();
                    //new_instrs.extend(next_instr.remove_max_immediate(max_immediate));
                    new_instrs
                });
        no_max_immediate
    }
}
