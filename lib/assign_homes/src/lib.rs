mod colors;
pub use colors::Coloring;
mod graph;
pub use graph::{InterferenceGraph, build_graph};
mod program;
pub use program::AnnotProg;

mod color_graph;
pub use color_graph::color_graph;

mod uncover_live;
pub use uncover_live::uncover_live;

mod assign_homes;
pub use assign_homes::assign_homes;
