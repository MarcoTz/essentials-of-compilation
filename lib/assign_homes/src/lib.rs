mod assign_homes;
mod color_graph;
mod colors;
mod errors;
mod graph;
mod program;
mod uncover_live;

pub use assign_homes::assign_homes;
pub use color_graph::color_graph;
pub use colors::Coloring;
pub use errors::Error;
pub use graph::{InterferenceGraph, build_graph};
pub use program::AnnotProg;
pub use uncover_live::uncover_live;
