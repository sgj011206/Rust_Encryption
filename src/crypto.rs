use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use anyhow::{anyhow, Result};
use rand::RngCore;

// 鍵の長さは32バイト (256ビット) です。
const KEY_SIZE: usize = 32;
// Nonce（使い捨てのランダムな値）の長さは12バイトです。
const NONCE_SIZE: usize = 12;

// 新しいランダムな256ビットの鍵を生成します。
pub fn generate_random_key() -> Vec<u8> {
    let mut key = vec![0u8; KEY_SIZE];
    // OSの安全な乱数生成器を使用します。
    OsRng.fill_bytes(&mut key);
    key
}

// データをAES-256-GCMで暗号化します。
pub fn encrypt_data(key_bytes: &[u8], plaintext: &[u8]) -> Result<Vec<u8>> {
    let key = Key::<Aes256Gcm>::from_slice(key_bytes);
    let cipher = Aes256Gcm::new(key);

    // 12バイトのランダムなNonceを生成します。
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    // 暗号化を実行します。失敗した場合はエラーを返します。
    let ciphertext = cipher
        .encrypt(&nonce, plaintext)
        .map_err(|_| anyhow!("Encryption failed: Invalid data or key"))?;

    // 結果として、[Nonce (12 bytes)] + [Ciphertext] の形式で結合したベクトルを返します。
    let mut encrypted_data = nonce.to_vec();
    encrypted_data.extend_from_slice(&ciphertext);

    Ok(encrypted_data)
}

// 暗号化されたデータを復号化します。
pub fn decrypt_data(key_bytes: &[u8], encrypted_data: &[u8]) -> Result<Vec<u8>> {
    // データがNonceの長さ(12バイト)より短い場合はエラーにします。
    if encrypted_data.len() < NONCE_SIZE {
        return Err(anyhow!("Invalid file format: File is too small"));
    }

    let key = Key::<Aes256Gcm>::from_slice(key_bytes);
    let cipher = Aes256Gcm::new(key);

    // データの先頭12バイトをNonceとして抽出します。
    let (nonce_bytes, ciphertext) = encrypted_data.split_at(NONCE_SIZE);
    let nonce = Nonce::from_slice(nonce_bytes);

    // 復号化を実行します。認証タグの検証に失敗した場合もここでエラーになります。
    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| anyhow!("Decryption failed: Incorrect key or corrupted file"))?;

    Ok(plaintext)
}