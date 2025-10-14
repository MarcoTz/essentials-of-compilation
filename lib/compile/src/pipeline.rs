use crate::{
    CompilerPaths, Error,
    passes::{
        Assemble, AssignHomes, BuildFlowGraph, BuildInterferenceGraph, ColorGraph,
        ExplicateControl, GeneratePreludeConclusion, Link, Parse, Pass, PatchInstructions,
        RemoveComplexOperands, SelectInstructions, Typecheck, UncoverLive, Uniquify,
    },
};

#[derive(Debug)]
pub enum Pipeline {
    Parse(<Parse as Pass>::Input),
    Typecheck(<Typecheck as Pass>::Input),
    Uniquify(<Uniquify as Pass>::Input),
    RemoveComplexOperands(<RemoveComplexOperands as Pass>::Input),
    ExplicateControl(<ExplicateControl as Pass>::Input),
    SelectInstructions(<SelectInstructions as Pass>::Input),
    BuildFlowGraph(<BuildFlowGraph as Pass>::Input),
    UncoverLive(<UncoverLive as Pass>::Input),
    BuildGraph(<BuildInterferenceGraph as Pass>::Input),
    ColorGraph(<ColorGraph as Pass>::Input),
    AssignHomes(<AssignHomes as Pass>::Input),
    PatchInstructions(<PatchInstructions as Pass>::Input),
    GeneratePreludeConclusion(<GeneratePreludeConclusion as Pass>::Input),
    Assemble(<Assemble as Pass>::Input),
    Link(<Link as Pass>::Input),
}

impl Pipeline {
    pub fn step(self, comp: &CompilerPaths, debug: bool) -> Result<Option<Pipeline>, Error> {
        match self {
            Pipeline::Parse(input) => {
                let output = <Parse as Pass>::run_debug(input, comp, debug)?;
                Ok(Some(Pipeline::Typecheck(output)))
            }
            Pipeline::Typecheck(input) => {
                let output = <Typecheck as Pass>::run_debug(input, comp, debug)?;
                Ok(Some(Pipeline::Uniquify(output)))
            }
            Pipeline::Uniquify(input) => {
                let output = <Uniquify as Pass>::run_debug(input, comp, debug)?;
                Ok(Some(Pipeline::RemoveComplexOperands(output)))
            }
            Pipeline::RemoveComplexOperands(input) => {
                let output = <RemoveComplexOperands as Pass>::run_debug(input, comp, debug)?;
                Ok(Some(Pipeline::ExplicateControl(output)))
            }
            Pipeline::ExplicateControl(input) => {
                let output = <ExplicateControl as Pass>::run_debug(input, comp, debug)?;
                Ok(Some(Pipeline::SelectInstructions(output)))
            }
            Pipeline::SelectInstructions(input) => {
                let output = <SelectInstructions as Pass>::run_debug(input, comp, debug)?;
                Ok(Some(Pipeline::BuildFlowGraph(output)))
            }
            Pipeline::BuildFlowGraph(input) => {
                let output = <BuildFlowGraph as Pass>::run_debug(input, comp, debug)?;
                Ok(Some(Pipeline::UncoverLive(output)))
            }
            Pipeline::UncoverLive(input) => {
                let output = <UncoverLive as Pass>::run_debug(input, comp, debug)?;

                Ok(Some(Pipeline::BuildGraph(output)))
            }
            Pipeline::BuildGraph(input) => {
                let output = <BuildInterferenceGraph as Pass>::run_debug(input, comp, debug)?;

                Ok(Some(Pipeline::ColorGraph(output)))
            }
            Pipeline::ColorGraph(input) => {
                let output = <ColorGraph as Pass>::run_debug(input, comp, debug)?;

                Ok(Some(Pipeline::AssignHomes(output)))
            }
            Pipeline::AssignHomes(input) => {
                let output = <AssignHomes as Pass>::run_debug(input, comp, debug)?;

                Ok(Some(Pipeline::PatchInstructions(output)))
            }
            Pipeline::PatchInstructions(input) => {
                let output = <PatchInstructions as Pass>::run_debug(input, comp, debug)?;

                Ok(Some(Pipeline::GeneratePreludeConclusion(output)))
            }
            Pipeline::GeneratePreludeConclusion(input) => {
                let output = <GeneratePreludeConclusion as Pass>::run_debug(input, comp, debug)?;

                Ok(Some(Pipeline::Assemble(output)))
            }
            Pipeline::Assemble(input) => {
                let _ = <Assemble as Pass>::run_debug(input, comp, debug)?;
                Ok(Some(Pipeline::Link(())))
            }
            Pipeline::Link(input) => {
                let _ = <Link as Pass>::run_debug(input, comp, debug)?;
                Ok(None)
            }
        }
    }
}
