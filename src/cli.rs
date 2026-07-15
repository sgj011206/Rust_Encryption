use clap::{Parser, Subcommand};

// CLIの基本構造体を定義します。
// clapのderiveマクロを使用して、自動的にヘルプメッセージなどを生成します。
#[derive(Parser)]
#[command(name = "rust_encryption")]
#[command(author = "sgj011206")]
#[command(version = "0.1.0")]
#[command(about = "A fast and secure file encryption CLI tool", long_about = None)]
#[command(disable_help_subcommand = true)]
pub struct Cli {
    // サブコマンドを受け付けるためのフィールド
    #[command(subcommand)]
    pub command: Commands,
}

// 実行可能なサブコマンドの一覧を定義します。
#[derive(Subcommand)]
pub enum Commands {
    /// Encrypts a specified file
    Encrypt {
        /// The path to the file you want to encrypt
        file_path: String,
    },
    /// Decrypts a specified file
    Decrypt {
        /// The path to the file you want to decrypt
        file_path: String,
    },
    /// Generates a new random encryption key
    Keygen,
    /// Shows the current application version
    Version,
    /// Shows all commands and their descriptions
    Help,
}