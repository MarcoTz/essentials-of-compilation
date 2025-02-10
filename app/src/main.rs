mod cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    cli::exec()
}
