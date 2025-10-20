use std::{collections::HashSet, mem::swap};

pub struct BlockAccum {
    blocks: Vec<core::Block>,
    pub current_statements: Vec<core::Statement>,
    pub current_label: String,
    used_labels: HashSet<String>,
}

impl BlockAccum {
    pub fn new() -> BlockAccum {
        BlockAccum {
            blocks: vec![],
            current_statements: vec![],
            current_label: "start".to_owned(),
            used_labels: HashSet::from(["start".to_owned()]),
        }
    }

    pub fn push_block(&mut self, block: core::Block) {
        self.used_labels.insert(block.label.clone());
        self.blocks.push(block)
    }

    pub fn push_stmt(&mut self, stmt: core::Statement) {
        self.current_statements.push(stmt)
    }

    pub fn next_block(&mut self, cont: core::Continuation) {
        let mut old = vec![];
        swap(&mut self.current_statements, &mut old);
        let block = core::Block {
            label: self.current_label.clone(),
            tail: core::Tail { stmts: old, cont },
        };
        self.used_labels.insert(self.current_label.to_owned());
        self.blocks.push(block);
    }

    pub fn fresh_label(&mut self) -> String {
        let mut num = 0;
        let mut next = format!("block_{}", num);
        while self.used_labels.contains(&next) {
            num += 1;
            next = format!("block_{}", num);
        }
        self.used_labels.insert(next.clone());
        next
    }

    pub fn build_prog(self) -> core::Program {
        let mut prog = core::Program::new();
        for block in self.blocks {
            prog.add_block(&block.label, block.tail);
        }
        prog
    }
}

impl Default for BlockAccum {
    fn default() -> BlockAccum {
        BlockAccum::new()
    }
}
