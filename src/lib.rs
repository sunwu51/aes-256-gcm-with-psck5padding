use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Key, Nonce,
};
use base64::{engine::general_purpose, Engine as _};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn aes_256_gcm_with_psck5padding_encrypt(key: &str, plaintext: &str) -> String {
    let k = Key::<Aes256Gcm>::from_slice(key.as_bytes());
    let nonce = Nonce::from_slice(key[..12].as_bytes());
    let cipher = Aes256Gcm::new(k);
    let plaintext = psck5_padding(&plaintext.as_bytes().to_vec());
    let cipher_vec = cipher.encrypt(&nonce, plaintext.as_bytes()).unwrap();
    let base64_str = general_purpose::STANDARD.encode(cipher_vec);
    return base64_str;
}

#[wasm_bindgen]
pub fn aes_256_gcm_with_psck5padding_decrypt(key: &str, base64_str: &str) -> String {
    let origin_u8 = general_purpose::STANDARD
        .decode(base64_str.as_bytes())
        .unwrap();
    let k = Key::<Aes256Gcm>::from_slice(key.as_bytes());
    let nonce = Nonce::from_slice(key[..12].as_bytes());
    let cipher = Aes256Gcm::new(k);
    let cipher_vec = cipher.decrypt(&nonce, origin_u8.as_ref()).unwrap();
    return psck5_unpadding(&cipher_vec);
}

fn psck5_padding(s: &Vec<u8>) -> String {
    let padding = (16 - s.len() % 16) as u8;
    let mut res = s.clone();
    for _ in 0..padding {
        res.push(padding);
    }
    return String::from_utf8(res).unwrap();
}

fn psck5_unpadding(s_vec: &Vec<u8>) -> String {
    let padding = s_vec[s_vec.len() - 1];
    let res = String::from_utf8(s_vec[0..(s_vec.len() - padding as usize)].to_vec()).unwrap();
    return res;
}

