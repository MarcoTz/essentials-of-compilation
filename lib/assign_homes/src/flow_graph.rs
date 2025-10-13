use crate::errors::Error;
use std::collections::HashSet;
use syntax::x86::{Block, Instruction, VarArg, VarProgram};

pub struct FlowGraph {
    verts: HashSet<String>,
    edges: HashSet<(String, String)>,
}

impl FlowGraph {
    pub fn new() -> FlowGraph {
        FlowGraph {
            verts: HashSet::new(),
            edges: HashSet::new(),
        }
    }

    pub fn add_vert(&mut self, vert: &str) {
        self.verts.insert(vert.to_owned());
    }

    pub fn add_edge(&mut self, from: &str, to: &str) {
        if !self.verts.contains(from) {
            self.add_vert(from);
        }
        if !self.verts.contains(to) {
            self.add_vert(to);
        }
        self.edges.insert((from.to_owned(), to.to_owned()));
    }

    pub fn incoming(&self, vert: &str) -> Vec<(String, String)> {
        let mut inc = vec![];
        for edge in self.edges.iter() {
            if edge.1 == vert {
                inc.push(edge.clone());
            }
        }
        inc
    }

    pub fn outgoing(&self, vert: &str) -> Vec<(String, String)> {
        let mut out = vec![];
        for edge in self.edges.iter() {
            if edge.0 == vert {
                out.push(edge.clone());
            }
        }
        out
    }

    pub fn build(&mut self, prog: &VarProgram) {
        for block in prog.blocks.iter() {
            self.build_block(&block);
        }
    }

    fn build_block(&mut self, block: &Block<VarArg>) {
        self.add_vert(&block.label);
        for instr in block.instrs.iter() {
            self.build_instr(instr, &block.label);
        }
    }

    fn build_instr(&mut self, instr: &Instruction<VarArg>, current_label: &str) {
        match instr {
            //Only relevant once top-level functions are available
            //Instruction::CallQ { label } => self.add_edge(current_label, label),
            Instruction::Jump { label } => self.add_edge(current_label, label),
            Instruction::JumpCC { label, .. } => self.add_edge(current_label, label),
            _ => (),
        }
    }

    pub fn topo_sort(mut self) -> Result<Vec<String>, Error> {
        let mut sorted = vec![];
        let mut start = self
            .verts
            .iter()
            .filter(|vert| self.incoming(vert).len() == 0)
            .cloned()
            .collect::<Vec<_>>();
        while !start.is_empty() {
            let next = start.remove(0);
            let outgoing = self.outgoing(&next);
            for edge in outgoing {
                self.edges.remove(&edge);
                if self.incoming(&edge.1).len() == 0 {
                    sorted.push(edge.1);
                }
            }
            sorted.push(next);
        }

        if !self.edges.is_empty() {
            return Err(Error::FlowCycle);
        }
        Ok(sorted)
    }
}

impl Default for FlowGraph {
    fn default() -> FlowGraph {
        FlowGraph::new()
    }
}
