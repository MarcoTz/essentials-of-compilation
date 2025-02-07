use chapter2::x86_var::Arg;
use std::{
    collections::{hash_map::DefaultHasher, HashSet},
    fmt,
    hash::{Hash, Hasher},
};

#[derive(Debug)]
pub struct Edge {
    from: Arg,
    to: Arg,
}

impl From<(Arg, Arg)> for Edge {
    fn from((a1, a2): (Arg, Arg)) -> Edge {
        Edge { from: a1, to: a2 }
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Edge) -> bool {
        (self.from == other.from && self.to == other.to)
            || (self.to == other.from && self.from == other.to)
    }
}

impl Eq for Edge {}

impl Hash for Edge {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        let mut s = DefaultHasher::new();
        self.from.hash(&mut s);
        let hash1 = s.finish();
        s = DefaultHasher::new();
        self.to.hash(&mut s);
        let hash2 = s.finish();
        state.write_u64(hash1 ^ hash2)
    }
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct InterferenceGraph {
    pub vertices: HashSet<Arg>,
    pub edges: HashSet<Edge>,
}

impl InterferenceGraph {
    pub fn add_vertex(&mut self, vert: Arg) {
        self.vertices.insert(vert);
    }

    pub fn add_edge(&mut self, start: Arg, end: Arg) {
        self.add_vertex(start.clone());
        self.add_vertex(end.clone());
        if start == end {
            return;
        }

        self.edges.insert((start, end).into());
    }

    pub fn adjacent(&self, v: &Arg) -> HashSet<Arg> {
        self.edges
            .iter()
            .filter_map(|Edge { from: v1, to: v2 }| {
                if v1 == v {
                    Some(v2)
                } else if v2 == v {
                    Some(v1)
                } else {
                    None
                }
            })
            .cloned()
            .collect()
    }
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}->{}", self.from, self.to)
    }
}

impl fmt::Display for InterferenceGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(
            &self
                .edges
                .iter()
                .map(|edg| edg.to_string())
                .collect::<Vec<String>>()
                .join("\n"),
        )
    }
}

#[cfg(test)]
mod graph_tests {
    use super::{Edge, InterferenceGraph};

    #[test]
    fn egdges_eq() {
        let edge1 = Edge {
            from: "a".into(),
            to: "b".into(),
        };
        let edge2 = Edge {
            from: "b".into(),
            to: "a".into(),
        };
        assert_eq!(edge1, edge2)
    }

    #[test]
    fn add_same() {
        let mut result = InterferenceGraph::default();
        result.add_edge("a".into(), "b".into());
        result.add_edge("b".into(), "a".into());
        let mut expected = InterferenceGraph::default();
        expected.add_edge("a".into(), "b".into());
        assert_eq!(result, expected)
    }
}
