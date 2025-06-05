use anyhow::{anyhow, Context, Result};
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305, Key, Nonce,
};

pub trait Encryption {
    fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>>;
    fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>>;
}

pub struct ChaCha20Poly1305Encryption {
    cipher: ChaCha20Poly1305,
}

impl ChaCha20Poly1305Encryption {
    pub fn new() -> Result<ChaCha20Poly1305Encryption> {
        // DEV: This is only for development, we MUST change this so that we generate/use a real key
        // TODO: change this code to:
        // - read key from decided location, L (TBD)
        // - if no key present, generate new one and store at L
        let raw_key = [0u8; 32];
        let key = Key::from_slice(&raw_key);
        let cipher = ChaCha20Poly1305::new(&key);
        Ok(ChaCha20Poly1305Encryption { cipher })
    }
}

impl Encryption for ChaCha20Poly1305Encryption {
    fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>> {
        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
        let ciphertext = self
            .cipher
            .encrypt(&nonce, plaintext.as_ref())
            .map_err(|e| anyhow::anyhow!(e))
            .context("encryption failed")?;
        let result_with_nonce = [nonce.as_slice(), ciphertext.as_slice()].concat();
        Ok(result_with_nonce)
    }

    fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>> {
        if ciphertext.len() < 12 {
            return Err(anyhow!("ciphertext is too short to contain a valid nonce"));
        }

        let (nonce_bytes, ciphertext_bytes) = ciphertext.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);
        let plaintext = self
            .cipher
            .decrypt(nonce, ciphertext_bytes)
            .map_err(|e| anyhow::anyhow!(e))
            .context("decryption failed")?;
        Ok(plaintext)
    }
}

