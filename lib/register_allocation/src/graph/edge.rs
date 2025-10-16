use crate::program::Location;
use std::{
    fmt,
    hash::{DefaultHasher, Hash, Hasher},
};

#[derive(Debug, Clone)]
pub struct Edge {
    pub left: Location,
    pub right: Location,
}

impl Edge {
    pub fn new(left: Location, right: Location) -> Edge {
        Edge { left, right }
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        (self.left == other.left && self.right == other.right)
            || (self.left == other.right && self.right == other.left)
    }
}

impl Eq for Edge {}

impl Hash for Edge {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut left_hasher = DefaultHasher::new();
        self.left.hash(&mut left_hasher);
        let left_hash = left_hasher.finish();

        let mut right_hasher = DefaultHasher::new();
        self.right.hash(&mut right_hasher);
        let right_hash = right_hasher.finish();

        (left_hash ^ right_hash).hash(state)
    }
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -- {}", self.left, self.right)
    }
}

#[cfg(test)]
mod edge_tests {
    use super::Edge;
    use lang_x86::Reg;
    use std::collections::HashSet;

    #[test]
    fn edge_neq() {
        let edg1 = Edge::new("x".into(), "y".into());
        let edg2 = Edge::new("z".into(), "x".into());
        assert_ne!(edg1, edg2)
    }

    #[test]
    fn edge_eq() {
        let edg1 = Edge::new("x".into(), "y".into());
        let edg2 = Edge::new("y".into(), "x".into());
        assert_eq!(edg1, edg2)
    }

    #[test]
    fn edge_hash_neq() {
        let edg1 = Edge::new("x".into(), "y".into());
        let edg2 = Edge::new("z".into(), "x".into());
        let mut set = HashSet::new();
        set.insert(edg1);
        assert!(!set.contains(&edg2))
    }

    #[test]
    fn edge_hash_eq() {
        let edg1 = Edge::new("x".into(), "y".into());
        let edg2 = Edge::new("y".into(), "x".into());
        let mut set = HashSet::new();
        set.insert(edg1);
        assert!(set.contains(&edg2));
    }

    #[test]
    fn edge_hash_reg() {
        let edg1 = Edge::new(Reg::Rax.into(), "t".into());
        let edg2 = Edge::new("t".into(), Reg::Rax.into());
        let mut set = HashSet::new();
        set.insert(edg1);
        assert!(set.contains(&edg2));
    }
}
