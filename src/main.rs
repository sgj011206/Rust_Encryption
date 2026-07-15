mod cli;
mod crypto;
mod file_io;

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

// プログラムのエントリーポイント（メイン関数）です。
fn main() {
    // コマンドライン引数を解析します。
    let cli = Cli::parse();

    // サブコマンドに応じて処理を分岐させます。
    let result = match &cli.command {
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
    };

    // 処理中にエラーが発生した場合は、標準エラー出力に表示して終了コード1を返します。
    if let Err(e) = result {
        eprintln!("ERROR: {:#}", e);
        std::process::exit(1);
    }
}