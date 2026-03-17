// Authors: Joysusy & Violet Klaudia 💖

use soul_crypto::{
    decrypt, encrypt, AuthTag, CryptoError, Key, Nonce, AUTH_TAG_SIZE, KEY_SIZE, NONCE_SIZE,
};

#[test]
fn test_encrypt_decrypt_roundtrip() {
    let key = Key::new([0x42u8; KEY_SIZE]);
    let plaintext = b"Hello, Violet!";

    let (nonce, ciphertext, auth_tag) = encrypt(&key, plaintext).unwrap();
    let decrypted = decrypt(&key, &nonce, &ciphertext, &auth_tag).unwrap();

    assert_eq!(decrypted, plaintext);
}

#[test]
fn test_encrypt_produces_different_nonces() {
    let key = Key::new([0x42u8; KEY_SIZE]);
    let plaintext = b"same plaintext";

    let (nonce1, ciphertext1, _) = encrypt(&key, plaintext).unwrap();
    let (nonce2, ciphertext2, _) = encrypt(&key, plaintext).unwrap();

    assert_ne!(nonce1.as_bytes(), nonce2.as_bytes());
    assert_ne!(ciphertext1, ciphertext2);
}

#[test]
fn test_encrypt_empty_plaintext() {
    let key = Key::new([0x42u8; KEY_SIZE]);
    let plaintext = b"";

    let (nonce, ciphertext, auth_tag) = encrypt(&key, plaintext).unwrap();
    let decrypted = decrypt(&key, &nonce, &ciphertext, &auth_tag).unwrap();

    assert_eq!(decrypted, plaintext);
}

#[test]
fn test_encrypt_large_plaintext() {
    let key = Key::new([0x42u8; KEY_SIZE]);
    let plaintext = vec![0x61u8; 1024 * 1024];

    let (nonce, ciphertext, auth_tag) = encrypt(&key, &plaintext).unwrap();
    let decrypted = decrypt(&key, &nonce, &ciphertext, &auth_tag).unwrap();

    assert_eq!(decrypted, plaintext);
}

#[test]
fn test_decrypt_wrong_key() {
    let key1 = Key::new([0x42u8; KEY_SIZE]);
    let key2 = Key::new([0x43u8; KEY_SIZE]);
    let plaintext = b"secret message";

    let (nonce, ciphertext, auth_tag) = encrypt(&key1, plaintext).unwrap();
    let result = decrypt(&key2, &nonce, &ciphertext, &auth_tag);

    assert!(matches!(result, Err(CryptoError::AuthenticationFailed)));
}

#[test]
fn test_decrypt_corrupted_ciphertext() {
    let key = Key::new([0x42u8; KEY_SIZE]);
    let plaintext = b"original message";

    let (nonce, mut ciphertext, auth_tag) = encrypt(&key, plaintext).unwrap();
    if !ciphertext.is_empty() {
        ciphertext[0] ^= 0xFF;
    }

    let result = decrypt(&key, &nonce, &ciphertext, &auth_tag);
    assert!(matches!(result, Err(CryptoError::AuthenticationFailed)));
}

#[test]
fn test_decrypt_corrupted_auth_tag() {
    let key = Key::new([0x42u8; KEY_SIZE]);
    let plaintext = b"authenticated message";

    let (nonce, ciphertext, auth_tag) = encrypt(&key, plaintext).unwrap();
    let mut corrupted_tag_bytes = [0u8; AUTH_TAG_SIZE];
    corrupted_tag_bytes.copy_from_slice(auth_tag.as_bytes());
    corrupted_tag_bytes[0] ^= 0xFF;
    let corrupted_tag = AuthTag::new(corrupted_tag_bytes);

    let result = decrypt(&key, &nonce, &ciphertext, &corrupted_tag);
    assert!(matches!(result, Err(CryptoError::AuthenticationFailed)));
}

#[test]
fn test_decrypt_corrupted_nonce() {
    let key = Key::new([0x42u8; KEY_SIZE]);
    let plaintext = b"nonce test";

    let (nonce, ciphertext, auth_tag) = encrypt(&key, plaintext).unwrap();
    let mut corrupted_nonce_bytes = [0u8; NONCE_SIZE];
    corrupted_nonce_bytes.copy_from_slice(nonce.as_bytes());
    corrupted_nonce_bytes[0] ^= 0xFF;
    let corrupted_nonce = Nonce::new(corrupted_nonce_bytes);

    let result = decrypt(&key, &corrupted_nonce, &ciphertext, &auth_tag);
    assert!(matches!(result, Err(CryptoError::AuthenticationFailed)));
}

#[test]
fn test_encrypt_unicode_plaintext() {
    let key = Key::new([0x42u8; KEY_SIZE]);
    let plaintext = "你好，Violet！🔐".as_bytes();

    let (nonce, ciphertext, auth_tag) = encrypt(&key, plaintext).unwrap();
    let decrypted = decrypt(&key, &nonce, &ciphertext, &auth_tag).unwrap();

    assert_eq!(decrypted, plaintext);
}

#[test]
fn test_encrypted_data_structure() {
    let key = Key::new([0x42u8; KEY_SIZE]);
    let plaintext = b"structure test";

    let (nonce, ciphertext, auth_tag) = encrypt(&key, plaintext).unwrap();

    assert_eq!(nonce.as_bytes().len(), NONCE_SIZE);
    assert_eq!(auth_tag.as_bytes().len(), AUTH_TAG_SIZE);
    assert!(!ciphertext.is_empty());
}

#[test]
fn test_key_from_slice_valid() {
    let bytes = [0x42u8; KEY_SIZE];
    let result = Key::from_slice(&bytes);
    assert!(result.is_ok());
}

#[test]
fn test_key_from_slice_invalid() {
    let bytes = [0x42u8; 16];
    let result = Key::from_slice(&bytes);
    assert!(matches!(result, Err(CryptoError::InvalidKeySize { .. })));
}
