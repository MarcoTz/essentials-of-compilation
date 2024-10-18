pub mod arg;
pub mod instr;
pub mod reg;

pub use arg::Arg;
pub use instr::Instr;
pub use reg::Reg;

use std::fmt;

use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct Prog {
    pub instrs: Vec<Instr<Arg>>,
    pub stack_space: usize,
    pub labels: HashMap<String, usize>,
}

impl fmt::Display for Prog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let label_strs: Vec<String> = self
            .labels
            .keys()
            .map(|lb| format!(".globl {}", lb))
            .collect();
        let mut instr_strs: Vec<String> = self
            .instrs
            .iter()
            .map(|instr| format!("{}", instr))
            .collect();
        for (label, location) in self.labels.iter() {
            instr_strs[*location] = format!("{}: {}", label, instr_strs[*location]);
        }
        write!(f, "{}\n{}", label_strs.join("\n"), instr_strs.join("\n"))
    }
}

#[cfg(test)]
mod prog_tests {
    use super::{Arg, Instr, Prog, Reg};
    use std::collections::HashMap;

    #[test]
    fn display_empty() {
        let result = format!(
            "{}",
            Prog {
                instrs: vec![],
                labels: HashMap::new(),
                stack_space: 0,
            }
        );
        let expected = "\n";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_prog() {
        let result = format!(
            "{}",
            Prog {
                instrs: vec![
                    Instr::MovQ(Arg::Immediate(1), Arg::Reg(Reg::Rax)),
                    Instr::CallQ("print_int".to_owned(), 0),
                    Instr::Jump("start".to_owned())
                ],
                labels: HashMap::from([("start".to_owned(), 1)]),
                stack_space: 0
            }
        );
        let expected = ".globl start\nmovq $1 %rax\nstart: callq print_int\njump start";
        assert_eq!(result, expected)
    }
}
