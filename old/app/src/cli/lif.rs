use std::path::PathBuf;

#[derive(clap::Args)]
pub struct Args {
    /// The file to compile
    file_path: PathBuf,
}

pub fn exec(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    println!("{:?}", args.file_path);
    Ok(())
}
