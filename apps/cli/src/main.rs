use clap::Parser;
use driver::Driver;
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    in_file: PathBuf,
    #[arg(short, long)]
    out: Option<PathBuf>,
    #[arg(long)]
    object_out: Option<PathBuf>,
    #[arg(long)]
    asm_out: Option<PathBuf>,
    #[arg(short, long)]
    verbose: bool,
}

fn run_cli() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let compiler = Driver::new(
        args.verbose,
        args.in_file,
        args.asm_out,
        args.object_out,
        args.out,
    )?;
    compiler.run()?;
    Ok(())
}

fn main() {
    match run_cli() {
        Ok(_) => (),
        Err(err) => eprintln!("Driver encountered an error\n{err}"),
    }
}
