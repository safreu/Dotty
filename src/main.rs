mod cli;
mod domain;
mod filesystem;
mod handlers;
mod persistence;
mod services;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    cli::dispatch()?;

    Ok(())
}
