// Authors: Joysusy & Violet Klaudia 💖

use crate::domain::types::{Key, KeyDerivationAlgorithm, Salt, KEY_SIZE};
use crate::error::{CryptoError, Result};
use argon2::{Argon2, ParamsBuilder, Version};

const ARGON2_MEMORY_COST: u32 = 65536; // 64 MB (optimal for security and performance)
const ARGON2_TIME_COST: u32 = 3;
const ARGON2_PARALLELISM: u32 = 4;

const PBKDF2_ITERATIONS: u32 = 600000;

const SCRYPT_LOG_N: u8 = 15;
const SCRYPT_R: u32 = 8;
const SCRYPT_P: u32 = 1;

pub async fn derive_key(
    passphrase: &str,
    salt: &Salt,
    algorithm: KeyDerivationAlgorithm,
) -> Result<Key> {
    if passphrase.is_empty() {
        return Err(CryptoError::PassphraseRequired);
    }

    let passphrase = passphrase.to_string();
    let salt_bytes = salt.as_bytes().to_vec();

    tokio::task::spawn_blocking(move || derive_key_blocking(&passphrase, &salt_bytes, algorithm))
        .await
        .map_err(|e| CryptoError::KeyDerivationFailed(e.to_string()))?
}

fn derive_key_blocking(
    passphrase: &str,
    salt: &[u8],
    algorithm: KeyDerivationAlgorithm,
) -> Result<Key> {
    match algorithm {
        KeyDerivationAlgorithm::Argon2id => derive_argon2id(passphrase, salt),
        KeyDerivationAlgorithm::Pbkdf2 => derive_pbkdf2(passphrase, salt),
        KeyDerivationAlgorithm::Scrypt => derive_scrypt(passphrase, salt),
    }
}

fn derive_argon2id(passphrase: &str, salt: &[u8]) -> Result<Key> {
    let params = ParamsBuilder::new()
        .m_cost(ARGON2_MEMORY_COST)
        .t_cost(ARGON2_TIME_COST)
        .p_cost(ARGON2_PARALLELISM)
        .output_len(KEY_SIZE)
        .build()
        .map_err(|e| CryptoError::KeyDerivationFailed(e.to_string()))?;

    let argon2 = Argon2::new(argon2::Algorithm::Argon2id, Version::V0x13, params);

    let mut output = [0u8; KEY_SIZE];
    argon2
        .hash_password_into(passphrase.as_bytes(), salt, &mut output)
        .map_err(|e| CryptoError::KeyDerivationFailed(e.to_string()))?;

    let key = Key::new(output);

    use zeroize::Zeroize;
    output.zeroize();

    Ok(key)
}

fn derive_pbkdf2(passphrase: &str, salt: &[u8]) -> Result<Key> {
    let mut output = [0u8; KEY_SIZE];
    pbkdf2::pbkdf2_hmac::<sha2::Sha256>(
        passphrase.as_bytes(),
        salt,
        PBKDF2_ITERATIONS,
        &mut output,
    );

    let key = Key::new(output);

    use zeroize::Zeroize;
    output.zeroize();

    Ok(key)
}

fn derive_scrypt(passphrase: &str, salt: &[u8]) -> Result<Key> {
    let params = scrypt::Params::new(SCRYPT_LOG_N, SCRYPT_R, SCRYPT_P, KEY_SIZE)
        .map_err(|e| CryptoError::KeyDerivationFailed(e.to_string()))?;

    let mut output = [0u8; KEY_SIZE];
    scrypt::scrypt(passphrase.as_bytes(), salt, &params, &mut output)
        .map_err(|e| CryptoError::KeyDerivationFailed(e.to_string()))?;

    let key = Key::new(output);

    use zeroize::Zeroize;
    output.zeroize();

    Ok(key)
}
