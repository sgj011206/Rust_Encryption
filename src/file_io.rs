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



// 在 src/file_io.rs 的最底部添加：

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_keygen_and_file_flow() {
        // 1. 测试密钥生成
        let keygen_res = handle_keygen();
        assert!(keygen_res.is_ok());
        assert!(fs::metadata(DEFAULT_KEY_FILE).is_ok());

        // 2. 创建一个临时测试文件
        let test_file = "test_input.txt";
        let test_content = "This is a file I want to protect.";
        fs::write(test_file, test_content).unwrap();

        // 3. 测试加密流程
        let enc_res = handle_encrypt(test_file);
        assert!(enc_res.is_ok());
        
        let enc_file = format!("{}.{}", test_file, ENC_EXTENSION);
        assert!(fs::metadata(&enc_file).is_ok());

        // 删除原文件模拟解密环境
        fs::remove_file(test_file).unwrap();

        // 4. 测试解密流程
        let dec_res = handle_decrypt(&enc_file);
        assert!(dec_res.is_ok());
        assert!(fs::metadata(test_file).is_ok());

        // 验证解密后的内容是否一致
        let recovered_content = fs::read_to_string(test_file).unwrap();
        assert_eq!(recovered_content, test_content);

        // 清理测试产生的垃圾文件
        let _ = fs::remove_file(DEFAULT_KEY_FILE);
        let _ = fs::remove_file(test_file);
        let _ = fs::remove_file(enc_file);
    }
}