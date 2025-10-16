use crate::CompilerPaths;
use std::fmt;
pub trait Pass {
    type Input;
    type Output;
    type Error: Into<crate::Error>;
    fn description() -> &'static str;
    fn run(input: Self::Input, paths: &CompilerPaths) -> Result<Self::Output, Self::Error>;

    fn run_debug(
        input: Self::Input,
        paths: &CompilerPaths,
        debug: bool,
    ) -> Result<Self::Output, Self::Error>
    where
        Self::Output: fmt::Display,
    {
        let output = Self::run(input, paths)?;
        if debug {
            println!("=== {} ===", Self::description());
            println!("{output}");
            println!();
        }
        Ok(output)
    }
}

mod assemble;
mod assign_homes;
mod build_flow_graph;
mod build_interference_graph;
mod color_graph;
mod explicate_control;
mod generate_prelude_conclusion;
mod link;
mod parse;
mod patch_instructions;
mod remove_complex_operands;
mod select_instructions;
mod typecheck;
mod uncover_live;
mod uniquify;

pub use assemble::Assemble;
pub use assign_homes::AssignHomes;
pub use build_flow_graph::BuildFlowGraph;
pub use build_interference_graph::BuildInterferenceGraph;
pub use color_graph::ColorGraph;
pub use explicate_control::Explicate;
pub use generate_prelude_conclusion::GeneratePreludeConclusion;
pub use link::Link;
pub use parse::Parse;
pub use patch_instructions::PatchInstrs;
pub use remove_complex_operands::Rco;
pub use select_instructions::SelectInstrs;
pub use typecheck::CheckTypes;
pub use uncover_live::UncoverLive;
pub use uniquify::UniquifyVariables;
