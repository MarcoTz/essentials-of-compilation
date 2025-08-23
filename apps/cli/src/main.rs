use clap::Parser;
use compile::Compiler;
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    in_file: PathBuf,
    #[arg(short, long)]
    out_file: Option<PathBuf>,
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut compiler = Compiler::new(args.verbose, args.in_file)?;
    compiler.explicate_control()?;
    Ok(())
}
