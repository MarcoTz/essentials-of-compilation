use driver::{l_int::LIntDriver, Driver};
use std::path::PathBuf;

#[derive(clap::Args)]
pub struct Args {
    /// The file to compile and evaluate
    file_path: PathBuf,
}

pub fn exec(args: Args) {
    let driver = LIntDriver;
    let evaled = driver
        .compile_and_eval_file(&args.file_path)
        .map_err(|err| err.to_string())
        .unwrap();
    println!("{evaled}");
}
