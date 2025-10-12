mod assign;
mod color_graph;
mod colors;
mod errors;
mod graph;
mod interference_graph;
mod move_graph;
mod program;
mod uncover_live;

pub use assign::assign_homes;
pub use color_graph::color_graph;
pub use colors::Coloring;
pub use errors::Error;
pub use graph::LocationGraph;
pub use interference_graph::build_interference_graph;
pub use move_graph::build_move_graph;
pub use program::AnnotProg;
pub use uncover_live::uncover_live;
