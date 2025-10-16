use super::Tail;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    pub label: String,
    pub tail: Tail,
}

impl Block {
    pub fn new(lb: &str, tail: Tail) -> Block {
        Block {
            label: lb.to_owned(),
            tail,
        }
    }
}
