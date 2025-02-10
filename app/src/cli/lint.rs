use driver::{l_int::LIntDriver, Driver};
use std::{fs::read_to_string, path::PathBuf};

#[derive(clap::Args)]
pub struct Args {
    /// The file to compile and evaluate
    file_path: PathBuf,
}

pub fn exec(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let driver = LIntDriver;
    let contents = read_to_string(args.file_path)?;
    let parsed = driver.parse(&contents)?;
    let evaled = driver.evaluate(parsed)?;
    println!("{evaled}");
    Ok(())
}
