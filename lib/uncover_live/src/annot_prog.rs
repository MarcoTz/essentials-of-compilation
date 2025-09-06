use super::LiveInstruction;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct AnnotProg {
    pub blocks: HashMap<String, Vec<LiveInstruction>>,
}

impl AnnotProg {
    pub fn new() -> AnnotProg {
        AnnotProg {
            blocks: HashMap::new(),
        }
    }

    pub fn add_block(&mut self, label: &str, instrs: Vec<LiveInstruction>) {
        self.blocks.insert(label.to_owned(), instrs);
    }
}

impl Default for AnnotProg {
    fn default() -> AnnotProg {
        AnnotProg::new()
    }
}
