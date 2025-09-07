use std::env::{current_dir, set_current_dir};

mod errors;
mod examples;
pub use errors::Error;
pub use examples::{Example, load_examples};

pub const EXAMPLES_DIR: &str = "examples";

pub fn set_working_dir() -> Result<(), Error> {
    let curr = current_dir().map_err(|_| Error::ReadDir("Current Directory".to_owned()))?;
    let working_dir = curr
        .parent()
        .ok_or(Error::ReadDir("Current Parnte".to_owned()))?;
    set_current_dir(working_dir).map_err(|_| Error::SetWorkingDir(working_dir.to_path_buf()))?;
    Ok(())
}
