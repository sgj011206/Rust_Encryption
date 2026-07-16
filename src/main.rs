mod cli;
mod crypto;
mod file_io;
mod gencomp;

use anyhow::Result;
use clap::{CommandFactory, Parser};
use cli::{Cli, Commands};
use std::path::Path;

/// Print the help message generated from the clap definition.
fn print_help() -> Result<()> {
    let mut command = Cli::command();
    command.print_help()?;
    println!();

    Ok(())
}

/// Print the application version.
fn print_version() {
    println!("rust_encryption v{}", env!("CARGO_PKG_VERSION"));
}

/// Execute a parsed subcommand.
fn execute_command(command: &Commands) -> Result<()> {
    match command {
        Commands::Keygen => file_io::handle_keygen(),

        Commands::Encrypt { file_path } => file_io::handle_encrypt(file_path),

        Commands::Decrypt { file_path } => file_io::handle_decrypt(file_path),

        Commands::Version => {
            print_version();
            Ok(())
        }

        Commands::Help => print_help(),
    }
}

/// Dispatch completion generation or a normal subcommand.
fn run(cli: &Cli) -> Result<()> {
    if cli.completions {
        return gencomp::generate(Path::new("completions"));
    }

    match cli.command.as_ref() {
        Some(command) => execute_command(command),
        None => print_help(),
    }
}

fn main() {
    let cli = Cli::parse();

    if let Err(error) = run(&cli) {
        eprintln!("ERROR: {error:#}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_help_command() {
        let result = execute_command(&Commands::Help);
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_version_command() {
        let result = execute_command(&Commands::Version);
        assert!(result.is_ok());
    }
}
