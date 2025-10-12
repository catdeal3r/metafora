use aes_gcm::aead::{Aead, KeyInit, OsRng};
use aes_gcm::AeadCore;
use aes_gcm::Aes256Gcm;
use hex::{self, FromHex};
use base64::prelude::*;

pub fn encode_to_base64(str: &mut String) {
    *str = BASE64_STANDARD.encode(str.as_bytes())
}

pub fn decode_from_base64(str: &mut String) {
    *str = String::from_utf8(BASE64_STANDARD.decode(str.as_bytes()).unwrap()).unwrap()
}

pub fn encrypt_str(str_to_encrypt: &String, encryption_key: &mut String) -> Vec<u8> {
    let key = Aes256Gcm::generate_key(OsRng);
    *encryption_key = hex::encode(&key);

    let cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let ciphertext = cipher.encrypt(&nonce, &*str_to_encrypt.as_bytes()).unwrap();

    [nonce.as_slice(), &ciphertext].concat().to_vec()
}

pub fn decrypt_str(str_to_decrypt: &mut Vec<u8>, encryption_key: &String) {
    let key_bytes = <Vec<u8>>::from_hex(encryption_key).unwrap();
    let cipher = Aes256Gcm::new_from_slice(&key_bytes).unwrap();

    let (nonce_slice, ciphertext) = str_to_decrypt.split_at(12); // nonce is 12 bytes
    let nonce = aes_gcm::Nonce::from_slice(nonce_slice);

    *str_to_decrypt = cipher.decrypt(&nonce, &*ciphertext).unwrap();
}
