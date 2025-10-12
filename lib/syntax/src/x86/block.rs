use super::Instruction;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block<Arg> {
    pub label: String,
    pub instrs: Vec<Instruction<Arg>>,
}

impl<Arg> Block<Arg> {
    pub fn new(label: &str, instrs: Vec<Instruction<Arg>>) -> Block<Arg> {
        Block {
            label: label.to_owned(),
            instrs,
        }
    }
}

impl<Arg> fmt::Display for Block<Arg>
where
    Arg: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}:\n{}",
            self.label,
            self.instrs
                .iter()
                .map(|instr| format!("\t{instr}",))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}
