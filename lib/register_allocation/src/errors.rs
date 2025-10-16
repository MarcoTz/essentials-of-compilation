use std::fmt;

#[derive(Debug)]
pub enum Error {
    NoAssignment(String),
    MissingLiveBefore(String),
    NextVertex,
    FlowCycle,
    MissingBlock(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::NoAssignment(var) => write!(f, "Could not find assignment for variable {var}"),
            Error::MissingLiveBefore(label) => {
                write!(f, "Could not find live before set for label {label}")
            }
            Error::NextVertex => write!(f, "Could not get next vertex to color"),
            Error::FlowCycle => write!(f, "Flow Graph contains cycle"),
            Error::MissingBlock(label) => write!(f, "Could not find block with label {label}"),
        }
    }
}

impl std::error::Error for Error {}
