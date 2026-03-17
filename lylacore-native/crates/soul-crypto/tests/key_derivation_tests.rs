// Authors: Joysusy & Violet Klaudia 💖

use soul_crypto::{
    derive_key, generate_salt, CryptoError, KeyDerivationAlgorithm, Salt, SALT_SIZE,
};

#[tokio::test]
async fn test_derive_key_argon2id_success() {
    let passphrase = "test_passphrase_123";
    let salt = generate_salt();

    let result = derive_key(passphrase, &salt, KeyDerivationAlgorithm::Argon2id).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_derive_key_deterministic() {
    let passphrase = "deterministic_test";
    let salt = Salt::new([0x11u8; SALT_SIZE]);

    let key1 = derive_key(passphrase, &salt, KeyDerivationAlgorithm::Argon2id)
        .await
        .unwrap();
    let key2 = derive_key(passphrase, &salt, KeyDerivationAlgorithm::Argon2id)
        .await
        .unwrap();

    assert_eq!(key1.as_bytes(), key2.as_bytes());
}

#[tokio::test]
async fn test_derive_key_different_salts() {
    let passphrase = "same_passphrase";
    let salt1 = Salt::new([0x01u8; SALT_SIZE]);
    let salt2 = Salt::new([0x02u8; SALT_SIZE]);

    let key1 = derive_key(passphrase, &salt1, KeyDerivationAlgorithm::Argon2id)
        .await
        .unwrap();
    let key2 = derive_key(passphrase, &salt2, KeyDerivationAlgorithm::Argon2id)
        .await
        .unwrap();

    assert_ne!(key1.as_bytes(), key2.as_bytes());
}

#[tokio::test]
async fn test_derive_key_empty_passphrase() {
    let passphrase = "";
    let salt = generate_salt();

    let result = derive_key(passphrase, &salt, KeyDerivationAlgorithm::Argon2id).await;
    assert!(matches!(result, Err(CryptoError::PassphraseRequired)));
}

#[tokio::test]
async fn test_derive_key_pbkdf2() {
    let passphrase = "pbkdf2_test";
    let salt = generate_salt();

    let result = derive_key(passphrase, &salt, KeyDerivationAlgorithm::Pbkdf2).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_derive_key_scrypt() {
    let passphrase = "scrypt_test";
    let salt = generate_salt();

    let result = derive_key(passphrase, &salt, KeyDerivationAlgorithm::Scrypt).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_derive_key_unicode_passphrase() {
    let passphrase = "测试密码🔐";
    let salt = Salt::new([0x55u8; SALT_SIZE]);

    let result = derive_key(passphrase, &salt, KeyDerivationAlgorithm::Argon2id).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_derive_key_long_passphrase() {
    let passphrase = "a".repeat(1024);
    let salt = Salt::new([0x77u8; SALT_SIZE]);

    let result = derive_key(&passphrase, &salt, KeyDerivationAlgorithm::Argon2id).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_salt_from_slice_valid() {
    let bytes = [0x42u8; SALT_SIZE];
    let result = Salt::from_slice(&bytes);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_salt_from_slice_invalid() {
    let bytes = [0x42u8; 16];
    let result = Salt::from_slice(&bytes);
    assert!(matches!(result, Err(CryptoError::InvalidSaltSize { .. })));
}
