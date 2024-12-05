pub mod arg;
pub mod instr;
pub mod reg;

pub use arg::Arg;
pub use instr::Instr;
pub use reg::Reg;

use std::fmt;

use std::collections::HashMap;

pub type Label = String;
pub type Var = String;

#[derive(Debug, PartialEq, Eq)]
pub enum Type {
    Int,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Program {
    pub blocks: HashMap<Label, Vec<Instr>>,
    pub types: HashMap<Var, Type>,
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let main_str = if self.blocks.contains_key("main") {
            ".globl main"
        } else {
            ""
        };
        let block_strs = self
            .blocks
            .iter()
            .map(|(label, instrs)| {
                format!(
                    "{label}: {}",
                    instrs
                        .iter()
                        .map(|instr| format!("{instr}\n"))
                        .collect::<Vec<String>>()
                        .join("\n")
                )
            })
            .collect::<Vec<String>>();
        write!(f, "{}\n{}", main_str, block_strs.join("\n"))
    }
}

#[cfg(test)]
mod prog_tests {
    use super::{Arg, Instr, Program, Reg};
    use std::collections::HashMap;

    #[test]
    fn display_empty() {
        let result = format!(
            "{}",
            Program {
                blocks: HashMap::new(),
                types: HashMap::new()
            }
        );
        let expected = "\n";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_prog() {
        let result = format!(
            "{}",
            Program {
                blocks: HashMap::from([(
                    "main".to_owned(),
                    vec![
                        Instr::MovQ(Arg::Immediate(1), Arg::Reg(Reg::Rax)),
                        Instr::CallQ("print_int".to_owned(), 0),
                        Instr::Jump("start".to_owned())
                    ]
                )]),
                types: HashMap::new()
            }
        );
        let expected = ".globl main\nmain: movq $1 %rax\n\ncallq print_int\n\njump start\n";
        assert_eq!(result, expected)
    }
}
