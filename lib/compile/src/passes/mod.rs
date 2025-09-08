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
mod build_graph;
mod color_graph;
mod explicate_control;
mod generate_prelude_conclusion;
mod link;
mod parse;
mod patch_instructions;
mod remove_complex_operands;
mod select_instructions;
mod uncover_live;
mod uniquify;

pub use assemble::Assemble;
pub use assign_homes::AssignHomes;
pub use build_graph::BuildGraph;
pub use color_graph::ColorGraph;
pub use explicate_control::ExplicateControl;
pub use generate_prelude_conclusion::GeneratePreludeConclusion;
pub use link::Link;
pub use parse::Parse;
pub use patch_instructions::PatchInstructions;
pub use remove_complex_operands::RemoveComplexOperands;
pub use select_instructions::SelectInstructions;
pub use uncover_live::UncoverLive;
pub use uniquify::Uniquify;
