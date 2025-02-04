use chapter2::compile::compile;
use chapter3::compile::compile as reg_compile;
use std::{fs::read_to_string, path::PathBuf};

#[derive(clap::Args)]
pub struct Args {
    /// The file to compile
    file_path: PathBuf,
    /// use register allocation (chapter 3) or skip
    #[arg(long)]
    skip_alloc: bool,
}

pub fn exec(args: Args) {
    let contents = read_to_string(args.file_path).unwrap();
    if args.skip_alloc {
        let compiled = compile(&contents).map_err(|err| err.to_string()).unwrap();
        println!("{compiled}");
    } else {
        let compiled = reg_compile(&contents)
            .map_err(|err| err.to_string())
            .unwrap();
        println!("{compiled}");
    }
}
