// Authors: Joysusy & Violet Klaudia 💖

use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi(object)]
pub struct DeriveKeyOptions {
    pub algorithm: String,
    pub memory_cost: Option<u32>,
    pub time_cost: Option<u32>,
    pub parallelism: Option<u32>,
}

#[napi(object)]
pub struct EncryptResult {
    pub nonce: Buffer,
    pub ciphertext: Buffer,
    pub auth_tag: Buffer,
}

fn parse_algorithm(opts: Option<DeriveKeyOptions>) -> soul_crypto::KeyDerivationAlgorithm {
    opts.and_then(|o| match o.algorithm.as_str() {
        "argon2id" => Some(soul_crypto::KeyDerivationAlgorithm::Argon2id),
        "pbkdf2" => Some(soul_crypto::KeyDerivationAlgorithm::Pbkdf2),
        "scrypt" => Some(soul_crypto::KeyDerivationAlgorithm::Scrypt),
        _ => None,
    })
    .unwrap_or(soul_crypto::KeyDerivationAlgorithm::Argon2id)
}

#[napi]
pub async fn derive_key(
    passphrase: String,
    salt: Buffer,
    options: Option<DeriveKeyOptions>,
) -> Result<Buffer> {
    let algorithm = parse_algorithm(options);

    let salt_obj =
        soul_crypto::Salt::from_slice(&salt).map_err(crate::error::crypto_error_to_napi)?;

    let key = soul_crypto::derive_key(&passphrase, &salt_obj, algorithm)
        .await
        .map_err(crate::error::crypto_error_to_napi)?;

    Ok(Buffer::from(key.as_bytes()))
}

#[napi]
pub fn derive_key_sync(
    passphrase: String,
    salt: Buffer,
    options: Option<DeriveKeyOptions>,
) -> Result<Buffer> {
    let algorithm = parse_algorithm(options);

    let salt_obj =
        soul_crypto::Salt::from_slice(&salt).map_err(crate::error::crypto_error_to_napi)?;

    // Create new runtime to avoid deadlock when called from existing runtime
    let rt = tokio::runtime::Runtime::new()
        .map_err(|e| napi::Error::from_reason(format!("Failed to create runtime: {}", e)))?;

    let key = rt
        .block_on(soul_crypto::derive_key(&passphrase, &salt_obj, algorithm))
        .map_err(crate::error::crypto_error_to_napi)?;

    Ok(Buffer::from(key.as_bytes()))
}

#[napi]
pub fn encrypt(key: Buffer, plaintext: Buffer) -> Result<EncryptResult> {
    let key_obj = soul_crypto::Key::from_slice(&key).map_err(crate::error::crypto_error_to_napi)?;

    let (nonce, ciphertext, auth_tag) =
        soul_crypto::encrypt(&key_obj, &plaintext).map_err(crate::error::crypto_error_to_napi)?;

    Ok(EncryptResult {
        nonce: Buffer::from(nonce.as_bytes()),
        ciphertext: Buffer::from(ciphertext.as_slice()),
        auth_tag: Buffer::from(auth_tag.as_bytes()),
    })
}

#[napi]
pub fn decrypt(key: Buffer, nonce: Buffer, ciphertext: Buffer, auth_tag: Buffer) -> Result<Buffer> {
    let key_obj = soul_crypto::Key::from_slice(&key).map_err(crate::error::crypto_error_to_napi)?;

    let nonce_obj =
        soul_crypto::Nonce::from_slice(&nonce).map_err(crate::error::crypto_error_to_napi)?;

    let auth_tag_obj =
        soul_crypto::AuthTag::from_slice(&auth_tag).map_err(crate::error::crypto_error_to_napi)?;

    let plaintext = soul_crypto::decrypt(&key_obj, &nonce_obj, &ciphertext, &auth_tag_obj)
        .map_err(crate::error::crypto_error_to_napi)?;

    Ok(Buffer::from(plaintext.as_slice()))
}

#[napi]
pub fn generate_salt() -> Buffer {
    let salt = soul_crypto::generate_salt();
    Buffer::from(salt.as_bytes())
}

#[napi]
pub fn generate_nonce() -> Buffer {
    let nonce = soul_crypto::generate_nonce();
    Buffer::from(nonce.as_bytes())
}

// Overhead measurement functions for performance analysis
#[napi]
pub async fn noop_async() -> Result<()> {
    Ok(())
}

#[napi]
pub fn noop_sync() -> Result<()> {
    Ok(())
}

#[napi]
pub fn copy_buffer(input: Buffer) -> Result<Buffer> {
    Ok(Buffer::from(input.as_ref()))
}
