use anyhow::Result;
use dotsec::encryption::{ChaCha20Poly1305Encryption, Encryption};

#[test]
fn test_round_trip() -> Result<()> {
    let encryptor = ChaCha20Poly1305Encryption::new()?;

    let input = b"Ashby Santoso";

    let encrypted_input = encryptor.encrypt(input)?;
    let decrypted_input = encryptor.decrypt(&encrypted_input)?;

    assert_eq!(input.to_vec(), decrypted_input);
    Ok(())
}

#[test]
fn test_different_inputs() -> Result<()> {
    let encryptor = ChaCha20Poly1305Encryption::new()?;

    let input_a = b"Jenks";
    let input_b = b"Kizzy";

    assert_ne!(input_a, input_b); // for sanity

    let encrypted_a = encryptor.encrypt(input_a)?;
    let encrypted_b = encryptor.encrypt(input_b)?;

    let decrypted_a = encryptor.decrypt(&encrypted_a)?;
    let decrypted_b = encryptor.decrypt(&encrypted_b)?;

    assert_ne!(decrypted_a, decrypted_b);
    Ok(())
}

#[test]
fn test_same_input_encrypted_twice() -> Result<()> {
    let encryptor = ChaCha20Poly1305Encryption::new()?;

    let input = b"Rosemary";

    let first = encryptor.encrypt(input)?;
    let second = encryptor.encrypt(input)?;

    // first and second are encrypted with different nonces, so they should be different
    assert_ne!(first, second);

    let decrypted_first = encryptor.decrypt(&first)?;
    let decrypted_second = encryptor.decrypt(&second)?;

    assert_eq!(decrypted_first, decrypted_second);
    Ok(())
}

#[test]
fn test_decryption_fails_with_wrong_ciphertext() -> Result<()> {
    let encryptor = ChaCha20Poly1305Encryption::new()?;

    let input = b"Dr Chef";

    let mut ciphertext = encryptor.encrypt(input)?;
    ciphertext[0] ^= 0xFF;

    assert!(encryptor.decrypt(&ciphertext).is_err());

    Ok(())
}

#[test]
fn test_decrypting_garbage_fails() -> Result<()> {
    let encryptor = ChaCha20Poly1305Encryption::new()?;

    let garbage = [0u8; 5];
    assert!(encryptor.decrypt(&garbage).is_err());

    Ok(())
}

#[test]
fn test_empty_input() -> Result<()> {
    let encryptor = ChaCha20Poly1305Encryption::new()?;

    let input = b"";

    let ciphertext = encryptor.encrypt(input)?;
    let output = encryptor.decrypt(&ciphertext)?;

    assert_eq!(output, b"");
    Ok(())
}
