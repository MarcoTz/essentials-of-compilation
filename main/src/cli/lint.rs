use std::path::PathBuf;

#[derive(clap::Args)]
pub struct Args {
    file_path: PathBuf,
}

pub fn exec(args: Args) {
    println!("{:?}", args.file_path);
}
