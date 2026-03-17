// Authors: Joysusy & Violet Klaudia 💖

use soul_crypto::{
    decrypt, derive_key, encrypt, generate_salt, KeyDerivationAlgorithm, Salt, SALT_SIZE,
};

#[tokio::test]
async fn test_full_encryption_workflow() {
    let passphrase = "my_secure_passphrase";
    let salt = generate_salt();

    let key = derive_key(passphrase, &salt, KeyDerivationAlgorithm::Argon2id)
        .await
        .unwrap();

    let plaintext = b"Sensitive data for Violet";
    let (nonce, ciphertext, auth_tag) = encrypt(&key, plaintext).unwrap();
    let decrypted = decrypt(&key, &nonce, &ciphertext, &auth_tag).unwrap();

    assert_eq!(decrypted, plaintext);
}

#[tokio::test]
async fn test_multiple_encryptions_same_key() {
    let passphrase = "shared_passphrase";
    let salt = Salt::new([0x99u8; SALT_SIZE]);

    let key = derive_key(passphrase, &salt, KeyDerivationAlgorithm::Argon2id)
        .await
        .unwrap();

    let plaintext1 = b"First message";
    let plaintext2 = b"Second message";
    let plaintext3 = b"Third message";

    let (nonce1, ciphertext1, auth_tag1) = encrypt(&key, plaintext1).unwrap();
    let (nonce2, ciphertext2, auth_tag2) = encrypt(&key, plaintext2).unwrap();
    let (nonce3, ciphertext3, auth_tag3) = encrypt(&key, plaintext3).unwrap();

    assert_eq!(
        decrypt(&key, &nonce1, &ciphertext1, &auth_tag1).unwrap(),
        plaintext1
    );
    assert_eq!(
        decrypt(&key, &nonce2, &ciphertext2, &auth_tag2).unwrap(),
        plaintext2
    );
    assert_eq!(
        decrypt(&key, &nonce3, &ciphertext3, &auth_tag3).unwrap(),
        plaintext3
    );
}

#[tokio::test]
async fn test_key_isolation() {
    let passphrase1 = "user1_passphrase";
    let passphrase2 = "user2_passphrase";
    let salt1 = Salt::new([0xAAu8; SALT_SIZE]);
    let salt2 = Salt::new([0xBBu8; SALT_SIZE]);

    let key1 = derive_key(passphrase1, &salt1, KeyDerivationAlgorithm::Argon2id)
        .await
        .unwrap();
    let key2 = derive_key(passphrase2, &salt2, KeyDerivationAlgorithm::Argon2id)
        .await
        .unwrap();

    let plaintext = b"shared plaintext";
    let (nonce, ciphertext, auth_tag) = encrypt(&key1, plaintext).unwrap();

    let result = decrypt(&key2, &nonce, &ciphertext, &auth_tag);
    assert!(result.is_err());
}

#[tokio::test]
async fn test_salt_uniqueness_ensures_key_uniqueness() {
    let passphrase = "same_passphrase";

    let salt1 = generate_salt();
    let salt2 = generate_salt();

    let key1 = derive_key(passphrase, &salt1, KeyDerivationAlgorithm::Argon2id)
        .await
        .unwrap();
    let key2 = derive_key(passphrase, &salt2, KeyDerivationAlgorithm::Argon2id)
        .await
        .unwrap();

    assert_ne!(key1.as_bytes(), key2.as_bytes());
}

#[tokio::test]
async fn test_encryption_with_pbkdf2() {
    let passphrase = "pbkdf2_passphrase";
    let salt = Salt::new([0xCCu8; SALT_SIZE]);

    let key = derive_key(passphrase, &salt, KeyDerivationAlgorithm::Pbkdf2)
        .await
        .unwrap();
    let plaintext = b"PBKDF2 test";

    let (nonce, ciphertext, auth_tag) = encrypt(&key, plaintext).unwrap();
    let decrypted = decrypt(&key, &nonce, &ciphertext, &auth_tag).unwrap();

    assert_eq!(decrypted, plaintext);
}

#[tokio::test]
async fn test_encryption_with_scrypt() {
    let passphrase = "scrypt_passphrase";
    let salt = Salt::new([0xDDu8; SALT_SIZE]);

    let key = derive_key(passphrase, &salt, KeyDerivationAlgorithm::Scrypt)
        .await
        .unwrap();
    let plaintext = b"Scrypt test";

    let (nonce, ciphertext, auth_tag) = encrypt(&key, plaintext).unwrap();
    let decrypted = decrypt(&key, &nonce, &ciphertext, &auth_tag).unwrap();

    assert_eq!(decrypted, plaintext);
}

#[tokio::test]
async fn test_large_data_encryption_workflow() {
    let passphrase = "large_data_passphrase";
    let salt = Salt::new([0xEEu8; SALT_SIZE]);

    let key = derive_key(passphrase, &salt, KeyDerivationAlgorithm::Argon2id)
        .await
        .unwrap();
    let plaintext = vec![0x42u8; 10 * 1024 * 1024];

    let (nonce, ciphertext, auth_tag) = encrypt(&key, &plaintext).unwrap();
    let decrypted = decrypt(&key, &nonce, &ciphertext, &auth_tag).unwrap();

    assert_eq!(decrypted, plaintext);
}

#[tokio::test]
async fn test_key_derivation_consistency_across_calls() {
    let passphrase = "consistency_test";
    let salt = Salt::new([0xFFu8; SALT_SIZE]);

    let mut keys = Vec::new();
    for _ in 0..5 {
        let key = derive_key(passphrase, &salt, KeyDerivationAlgorithm::Argon2id)
            .await
            .unwrap();
        keys.push(key);
    }

    for key in &keys[1..] {
        assert_eq!(keys[0].as_bytes(), key.as_bytes());
    }
}
