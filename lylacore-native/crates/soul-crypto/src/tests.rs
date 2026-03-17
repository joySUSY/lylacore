// Authors: Joysusy & Violet Klaudia 💖

#[cfg(test)]
mod unit_tests {
    use crate::*;

    #[tokio::test]
    async fn test_argon2id_key_derivation() {
        let passphrase = "test-passphrase-123";
        let salt = generate_salt();

        let key = derive_key(passphrase, &salt, KeyDerivationAlgorithm::Argon2id)
            .await
            .expect("Key derivation failed");

        assert_eq!(key.as_bytes().len(), KEY_SIZE);

        let key2 = derive_key(passphrase, &salt, KeyDerivationAlgorithm::Argon2id)
            .await
            .expect("Key derivation failed");

        assert_eq!(key.as_bytes(), key2.as_bytes());
    }

    #[tokio::test]
    async fn test_pbkdf2_key_derivation() {
        let passphrase = "test-passphrase-456";
        let salt = generate_salt();

        let key = derive_key(passphrase, &salt, KeyDerivationAlgorithm::Pbkdf2)
            .await
            .expect("Key derivation failed");

        assert_eq!(key.as_bytes().len(), KEY_SIZE);
    }

    #[tokio::test]
    async fn test_scrypt_key_derivation() {
        let passphrase = "test-passphrase-789";
        let salt = generate_salt();

        let key = derive_key(passphrase, &salt, KeyDerivationAlgorithm::Scrypt)
            .await
            .expect("Key derivation failed");

        assert_eq!(key.as_bytes().len(), KEY_SIZE);
    }

    #[tokio::test]
    async fn test_encrypt_decrypt_roundtrip() {
        let passphrase = "my-secret-passphrase";
        let salt = generate_salt();
        let key = derive_key(passphrase, &salt, KeyDerivationAlgorithm::Argon2id)
            .await
            .expect("Key derivation failed");

        let plaintext = b"Hello, Violet! This is a secret message.";
        let (nonce, ciphertext, auth_tag) = encrypt(&key, plaintext).expect("Encryption failed");

        let decrypted = decrypt(&key, &nonce, &ciphertext, &auth_tag).expect("Decryption failed");

        assert_eq!(plaintext, decrypted.as_slice());
    }

    #[tokio::test]
    async fn test_decrypt_with_wrong_key_fails() {
        let passphrase1 = "correct-passphrase";
        let passphrase2 = "wrong-passphrase";
        let salt = generate_salt();

        let key1 = derive_key(passphrase1, &salt, KeyDerivationAlgorithm::Argon2id)
            .await
            .expect("Key derivation failed");
        let key2 = derive_key(passphrase2, &salt, KeyDerivationAlgorithm::Argon2id)
            .await
            .expect("Key derivation failed");

        let plaintext = b"Secret data";
        let (nonce, ciphertext, auth_tag) = encrypt(&key1, plaintext).expect("Encryption failed");

        let result = decrypt(&key2, &nonce, &ciphertext, &auth_tag);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            CryptoError::AuthenticationFailed
        ));
    }

    #[tokio::test]
    async fn test_decrypt_with_tampered_ciphertext_fails() {
        let passphrase = "test-passphrase";
        let salt = generate_salt();
        let key = derive_key(passphrase, &salt, KeyDerivationAlgorithm::Argon2id)
            .await
            .expect("Key derivation failed");

        let plaintext = b"Original message";
        let (nonce, mut ciphertext, auth_tag) =
            encrypt(&key, plaintext).expect("Encryption failed");

        if !ciphertext.is_empty() {
            ciphertext[0] ^= 0xFF;
        }

        let result = decrypt(&key, &nonce, &ciphertext, &auth_tag);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            CryptoError::AuthenticationFailed
        ));
    }

    #[tokio::test]
    async fn test_empty_passphrase_fails() {
        let salt = generate_salt();
        let result = derive_key("", &salt, KeyDerivationAlgorithm::Argon2id).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            CryptoError::PassphraseRequired
        ));
    }

    #[test]
    fn test_salt_from_slice_invalid_size() {
        let invalid_salt = vec![0u8; 16];
        let result = Salt::from_slice(&invalid_salt);
        assert!(result.is_err());
    }

    #[test]
    fn test_nonce_from_slice_invalid_size() {
        let invalid_nonce = vec![0u8; 8];
        let result = Nonce::from_slice(&invalid_nonce);
        assert!(result.is_err());
    }

    #[test]
    fn test_generate_salt_produces_unique_values() {
        let salt1 = generate_salt();
        let salt2 = generate_salt();
        assert_ne!(salt1.as_bytes(), salt2.as_bytes());
    }

    #[test]
    fn test_generate_nonce_produces_unique_values() {
        let nonce1 = generate_nonce();
        let nonce2 = generate_nonce();
        assert_ne!(nonce1.as_bytes(), nonce2.as_bytes());
    }
}
