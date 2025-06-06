use anyhow::{Context, Result, anyhow};
use chacha20poly1305::{
    ChaCha20Poly1305, Key, Nonce,
    aead::{Aead, AeadCore, KeyInit, OsRng},
};
use std::fs::{File, read};
use std::io::prelude::*;
use xdg::BaseDirectories;

pub trait Encryption {
    fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>>;
    fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>>;
}

pub struct ChaCha20Poly1305Encryption {
    cipher: ChaCha20Poly1305,
}

impl ChaCha20Poly1305Encryption {
    /// Loads a ChaCha20 key from disk or generates and stores a new one if it doesn't exist.
    fn load_or_generate_key() -> Result<Key> {
        let xdg = BaseDirectories::with_prefix("dotsec");
        let key_file_path = xdg.place_config_file("private/dotsec.key")?;

        if key_file_path.exists() {
            let bytes = read(key_file_path)?;
            Ok(Key::clone_from_slice(&bytes))
        } else {
            let new_key = ChaCha20Poly1305::generate_key(&mut OsRng);
            let mut file = File::create(&key_file_path)?;
            file.write_all(&new_key)?;
            Ok(new_key)
        }
    }

    pub fn new() -> Result<ChaCha20Poly1305Encryption> {
        let key = Self::load_or_generate_key()?;
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
