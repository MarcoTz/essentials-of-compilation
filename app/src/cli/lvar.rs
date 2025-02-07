use driver::l_var::LVarDriver;
use driver::{l_var_reg::LVarRegDriver, Driver};
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
        let driver = LVarDriver;
        let compiled = driver
            .compile(&contents, false)
            .map_err(|err| err.to_string())
            .unwrap();
        println!("{compiled}");
    } else {
        let driver = LVarRegDriver;
        let compiled = driver
            .compile(&contents, false)
            .map_err(|err| err.to_string())
            .unwrap();
        println!("{compiled}");
    }
}
