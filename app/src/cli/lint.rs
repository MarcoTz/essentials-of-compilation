use driver::{l_int::LIntDriver, Driver};
use std::{fs::read_to_string, path::PathBuf};

#[derive(clap::Args)]
pub struct Args {
    /// The file to compile and evaluate
    file_path: PathBuf,
}

pub fn exec(args: Args) {
    let driver = LIntDriver;
    let contents = read_to_string(args.file_path)
        .map_err(|err| err.to_string())
        .unwrap();
    let parsed = driver
        .parse(&contents)
        .map_err(|err| err.to_string())
        .unwrap();
    let evaled = driver
        .evaluate(parsed)
        .map_err(|err| err.to_string())
        .unwrap();
    println!("{evaled}");
}
