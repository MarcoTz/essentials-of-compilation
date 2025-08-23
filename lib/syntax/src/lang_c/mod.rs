use std::{collections::HashMap, fmt};

mod atm;
mod expr;
mod stmt;
mod tail;

pub use atm::Atom;
pub use expr::Expression;
pub use stmt::Statement;
pub use tail::Tail;

#[derive(Debug, Clone)]
pub struct Program {
    pub blocks: HashMap<String, Tail>,
}

impl Program {
    pub fn new() -> Program {
        Program {
            blocks: HashMap::new(),
        }
    }

    pub fn add_block(&mut self, label: &str, tail: Tail) {
        self.blocks.insert(label.to_owned(), tail);
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (label, tail) in self.blocks.iter() {
            write!(f, "{label}:\n  {}", tail.to_string().replace('\n', "\n  "))?;
        }
        Ok(())
    }
}
