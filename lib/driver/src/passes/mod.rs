use crate::CompilerPaths;
use std::convert::Infallible;

pub trait Pass: Sized {
    type Next: Pass;
    type Prev: Pass;
    type Error: Into<crate::Error>;

    fn description() -> &'static str;
    fn show_input(&self) -> String;

    fn run(self, paths: &CompilerPaths) -> Result<Self::Next, Self::Error>;

    fn run_debug(self, paths: &CompilerPaths, debug: bool) -> Result<Self::Next, Self::Error> {
        let next = self.run(paths)?;
        if debug {
            println!("=== {} ===", Self::description());
            println!("{}", next.show_input());
            println!();
        }
        Ok(next)
    }
}

pub struct Done;

impl Pass for Done {
    type Next = Self;
    type Prev = Link;
    type Error = Infallible;

    fn description() -> &'static str {
        "Successfully compiled Program"
    }

    fn show_input(&self) -> String {
        "".to_owned()
    }

    fn run(self, _: &CompilerPaths) -> Result<Self::Next, Self::Error> {
        Ok(Done)
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
