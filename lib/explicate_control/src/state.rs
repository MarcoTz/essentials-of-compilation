use syntax::lang_c;

pub struct ExplicateState {
    pub blocks: Vec<lang_c::Block>,
}

impl ExplicateState {
    pub fn new() -> ExplicateState {
        ExplicateState { blocks: vec![] }
    }

    pub fn fresh_label(&self) -> String {
        let prefix = "block_";
        let mut num = 0;
        let mut label = format!("{prefix}{num}");
        while self
            .blocks
            .iter()
            .map(|block| &block.label)
            .collect::<Vec<_>>()
            .contains(&&label)
        {
            num += 1;
            label = format!("{prefix}{num}");
        }
        label
    }

    pub fn add_block(&mut self, tail: lang_c::Tail) -> String {
        let label = self.fresh_label();
        let block = lang_c::Block::new(&label, tail);
        self.blocks.push(block);
        label
    }

    pub fn move_blocks(self, prog: &mut lang_c::Program) {
        for block in self.blocks {
            prog.add_block(&block.label, block.tail);
        }
    }
}

impl Default for ExplicateState {
    fn default() -> ExplicateState {
        ExplicateState::new()
    }
}
