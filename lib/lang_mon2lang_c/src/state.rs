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

    pub fn add_block(&mut self, tail: lang_c::Tail, label: Option<&str>) -> String {
        let label = match label {
            None => self.fresh_label(),
            Some(lb) => lb.to_owned(),
        };
        let block = lang_c::Block::new(&label, tail);
        self.blocks.push(block);
        label
    }

    pub fn move_blocks(&mut self, prog: &mut lang_c::Program) {
        while !self.blocks.is_empty() {
            let next = self.blocks.remove(0);
            prog.add_block(&next.label, next.tail);
        }
    }
}

impl Default for ExplicateState {
    fn default() -> ExplicateState {
        ExplicateState::new()
    }
}
