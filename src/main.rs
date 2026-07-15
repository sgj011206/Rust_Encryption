mod cli;
mod crypto;
mod file_io;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

fn print_help() {
    println!("Available commands:\n");
    println!("  keygen              Generate a new random encryption key");
    println!("  encrypt <file_path> Encrypt a specified file");
    println!("  decrypt <file_path> Decrypt a specified file");
    println!("  version             Show the current application version");
    println!("  help                Show all commands and their descriptions");
}

fn print_version() {
    println!("rust_encryption v{}", env!("CARGO_PKG_VERSION"));
}

fn execute_command(command: &Commands) -> Result<()> {
    match command {
        Commands::Keygen => file_io::handle_keygen(),
        Commands::Encrypt { file_path } => file_io::handle_encrypt(file_path),
        Commands::Decrypt { file_path } => file_io::handle_decrypt(file_path),
        Commands::Version => {
            print_version();
            Ok(())
        }
        Commands::Help => {
            print_help();
            Ok(())
        }
    }
}

// プログラムのエントリーポイント（メイン関数）です。
fn main() {
    // コマンドライン引数を解析します。
    let cli = Cli::parse();

    // サブコマンドに応じて処理を分岐させます。
    let result = execute_command(&cli.command);

    // 処理中にエラーが発生した場合は、標準エラー出力に表示して終了コード1を返します。
    if let Err(e) = result {
        eprintln!("ERROR: {:#}", e);
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