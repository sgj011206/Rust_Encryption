use crate::crypto;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

// デフォルトの鍵ファイル名
const DEFAULT_KEY_FILE: &str = "rust_encryption.key";
// 暗号化ファイルの拡張子
const ENC_EXTENSION: &str = "enc";

// 鍵を生成してローカルファイルに保存します。
pub fn handle_keygen() -> Result<()> {
    let key = crypto::generate_random_key();
    fs::write(DEFAULT_KEY_FILE, &key)
        .with_context(|| format!("Failed to write key to {}", DEFAULT_KEY_FILE))?;
    
    println!("SUCCESS: A new key has been generated.");
    println!("Key saved to: {}", DEFAULT_KEY_FILE);
    println!("WARNING: Keep this key safe. If you lose it, your data cannot be recovered!");
    Ok(())
}

// 鍵ファイルから鍵を読み込みます。
fn load_key() -> Result<Vec<u8>> {
    fs::read(DEFAULT_KEY_FILE)
        .with_context(|| format!("Failed to read key file: {}. Please run 'keygen' first.", DEFAULT_KEY_FILE))
}

// 指定されたファイルを暗号化します。
pub fn handle_encrypt(file_path: &str) -> Result<()> {
    let key = load_key()?;
    
    // 対象のファイルからすべてのデータを読み込みます。
    let plaintext = fs::read(file_path)
        .with_context(|| format!("Failed to read file: {}", file_path))?;

    println!("Encrypting '{}'...", file_path);
    let encrypted_data = crypto::encrypt_data(&key, &plaintext)?;

    // 出力ファイル名を作成します (例: text.txt -> text.txt.enc)
    let output_path = format!("{}.{}", file_path, ENC_EXTENSION);
    
    // 暗号化されたデータを書き込みます。
    fs::write(&output_path, encrypted_data)
        .with_context(|| format!("Failed to write encrypted file to {}", output_path))?;

    println!("SUCCESS: File encrypted safely.");
    println!("Output file: {}", output_path);
    Ok(())
}

// 指定された暗号化ファイルを復号化します。
pub fn handle_decrypt(file_path: &str) -> Result<()> {
    let key = load_key()?;

    // 対象の暗号化ファイルを読み込みます。
    let encrypted_data = fs::read(file_path)
        .with_context(|| format!("Failed to read encrypted file: {}", file_path))?;

    println!("Decrypting '{}'...", file_path);
    let plaintext = crypto::decrypt_data(&key, &encrypted_data)?;

    // 出力ファイル名を作成します。
    // .enc 拡張子がついている場合はそれを削除し、そうでない場合は .dec 拡張子を付けます。
    let path = Path::new(file_path);
    let output_path = if path.extension().and_then(|s| s.to_str()) == Some(ENC_EXTENSION) {
        path.with_extension("").to_string_lossy().to_string()
    } else {
        format!("{}.dec", file_path)
    };

    // 復号化されたデータを書き込みます。
    fs::write(&output_path, plaintext)
        .with_context(|| format!("Failed to write decrypted file to {}", output_path))?;

    println!("SUCCESS: File decrypted safely.");
    println!("Output file: {}", output_path);
    Ok(())
}