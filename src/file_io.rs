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




#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::{Mutex, OnceLock};
    use std::time::{SystemTime, UNIX_EPOCH};

    fn fs_test_lock() -> &'static Mutex<()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
    }

    fn unique_test_file(prefix: &str) -> String {
        let millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let pid = std::process::id();
        let filename = format!("{}_{}_{}", prefix, pid, millis);
        let path = std::env::temp_dir().join(filename);
        path.to_string_lossy().to_string()
    }

    #[test]
    fn test_keygen_and_file_flow() {
        let _guard = fs_test_lock().lock().unwrap();

        // 1. キー生成をテスト
        let keygen_res = handle_keygen();
        assert!(keygen_res.is_ok());
        assert!(fs::metadata(DEFAULT_KEY_FILE).is_ok());

        // 2. 一時的なテストファイルを作成
        let test_file = unique_test_file("test_input.txt");
        let test_content = "This is a file I want to protect.";
        fs::write(&test_file, test_content).unwrap();

        // 3. 暗号化処理をテスト
        let enc_res = handle_encrypt(&test_file);
        assert!(enc_res.is_ok());
        
        let enc_file = format!("{}.{}", test_file, ENC_EXTENSION);
        assert!(fs::metadata(&enc_file).is_ok());

        // 復号環境を再現するために元のファイルを削除
        fs::remove_file(&test_file).unwrap();

        // 4. 復号処理をテスト
        let dec_res = handle_decrypt(&enc_file);
        assert!(dec_res.is_ok());
        assert!(fs::metadata(&test_file).is_ok());

        // 復号後の内容が元の内容と一致することを確認
        let recovered_content = fs::read_to_string(&test_file).unwrap();
        assert_eq!(recovered_content, test_content);

        // テストで生成された不要なファイルを削除
        let _ = fs::remove_file(DEFAULT_KEY_FILE);
        let _ = fs::remove_file(&test_file);
        let _ = fs::remove_file(enc_file);
    }

    #[test]
    fn test_encrypt_without_key_fails() {
        let _guard = fs_test_lock().lock().unwrap();
        let _ = fs::remove_file(DEFAULT_KEY_FILE);

        let test_file = unique_test_file("no_key_input.txt");
        fs::write(&test_file, "data").unwrap();

        let result = handle_encrypt(&test_file);
        assert!(result.is_err());

        let err_text = result.unwrap_err().to_string();
        assert!(err_text.contains("Please run 'keygen' first"));

        let _ = fs::remove_file(&test_file);
    }

    #[test]
    fn test_decrypt_non_enc_extension_outputs_dec_file() {
        let _guard = fs_test_lock().lock().unwrap();

        handle_keygen().unwrap();

        let base_file = unique_test_file("branch_case.txt");
        let content = "branch test content";
        fs::write(&base_file, content).unwrap();

        handle_encrypt(&base_file).unwrap();
        let enc_file = format!("{}.{}", base_file, ENC_EXTENSION);

        let renamed_file = format!("{}.bin", base_file);
        fs::rename(&enc_file, &renamed_file).unwrap();

        handle_decrypt(&renamed_file).unwrap();
        let dec_file = format!("{}.dec", renamed_file);

        let recovered = fs::read_to_string(&dec_file).unwrap();
        assert_eq!(recovered, content);

        let _ = fs::remove_file(DEFAULT_KEY_FILE);
        let _ = fs::remove_file(&base_file);
        let _ = fs::remove_file(&renamed_file);
        let _ = fs::remove_file(&dec_file);
    }

    #[test]
    fn test_decrypt_corrupted_file_fails() {
        let _guard = fs_test_lock().lock().unwrap();

        handle_keygen().unwrap();

        let test_file = unique_test_file("corrupted_case.txt");
        fs::write(&test_file, "sensitive content").unwrap();
        handle_encrypt(&test_file).unwrap();

        let enc_file = format!("{}.{}", test_file, ENC_EXTENSION);
        let mut encrypted_data = fs::read(&enc_file).unwrap();
        let last_idx = encrypted_data.len() - 1;
        encrypted_data[last_idx] ^= 0xFF;
        fs::write(&enc_file, &encrypted_data).unwrap();

        let result = handle_decrypt(&enc_file);
        assert!(result.is_err());

        let _ = fs::remove_file(DEFAULT_KEY_FILE);
        let _ = fs::remove_file(&test_file);
        let _ = fs::remove_file(&enc_file);
    }
}

