mod cli;
mod domain;
mod handlers;
mod persistence;
mod services;
mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    cli::execute()?;

    Ok(())
}
