use clap::{Parser, Subcommand, ValueHint};

/// Command-line interface definition.
#[derive(Debug, Parser)]
#[command(name = "rust_encryption")]
#[command(author = "sgj011206")]
#[command(version)]
#[command(
    about = "A fast and secure file encryption CLI tool",
    long_about = None
)]
#[command(disable_help_subcommand = true)]
pub struct Cli {
    /// Generate completion files for all supported shells.
    #[arg(long, help = "Generate completion files", default_value_t = false)]
    pub completions: bool,

    /// Subcommand to execute.
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Available subcommands.
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Encrypts a specified file.
    Encrypt {
        /// The path to the file you want to encrypt.
        #[arg(value_hint = ValueHint::FilePath)]
        file_path: String,
    },

    /// Decrypts a specified file.
    Decrypt {
        /// The path to the file you want to decrypt.
        #[arg(value_hint = ValueHint::FilePath)]
        file_path: String,
    },

    /// Generates a new random encryption key.
    Keygen,

    /// Shows the current application version.
    Version,

    /// Shows all commands and their descriptions.
    Help,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_keygen_command() {
        let cli = Cli::try_parse_from(["rust_encryption", "keygen"]).unwrap();

        assert!(!cli.completions);
        assert!(matches!(cli.command, Some(Commands::Keygen)));
    }

    #[test]
    fn test_parse_encrypt_command() {
        let cli = Cli::try_parse_from(["rust_encryption", "encrypt", "sample.txt"]).unwrap();

        match cli.command {
            Some(Commands::Encrypt { file_path }) => {
                assert_eq!(file_path, "sample.txt");
            }
            _ => panic!("Encrypt command was not parsed"),
        }
    }

    #[test]
    fn test_parse_decrypt_command() {
        let cli = Cli::try_parse_from(["rust_encryption", "decrypt", "sample.txt.enc"]).unwrap();

        match cli.command {
            Some(Commands::Decrypt { file_path }) => {
                assert_eq!(file_path, "sample.txt.enc");
            }
            _ => panic!("Decrypt command was not parsed"),
        }
    }

    #[test]
    fn test_parse_help_command() {
        let cli = Cli::try_parse_from(["rust_encryption", "help"]).unwrap();

        assert!(matches!(cli.command, Some(Commands::Help)));
    }

    #[test]
    fn test_parse_version_command() {
        let cli = Cli::try_parse_from(["rust_encryption", "version"]).unwrap();

        assert!(matches!(cli.command, Some(Commands::Version)));
    }

    #[test]
    fn test_parse_completions_option_without_subcommand() {
        let cli = Cli::try_parse_from(["rust_encryption", "--completions"]).unwrap();

        assert!(cli.completions);
        assert!(cli.command.is_none());
    }
}
