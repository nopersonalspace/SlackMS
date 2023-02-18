use magic_crypt::{new_magic_crypt, MagicCryptTrait};

/// Encrypt a string (url)
pub fn encrypt_url(url: &str) -> String {
    let mc = new_magic_crypt!("magickey", 256);
    mc.encrypt_str_to_base64(url)
}

/// Decrypt a string (url)
pub fn decrypt_url(encrypted_url: String) -> Result<String, String> {
    let mc = new_magic_crypt!("magickey", 256);
    let url = mc.decrypt_base64_to_string(&encrypted_url);
    match url {
        Ok(valid_url) => Ok(valid_url),
        Err(e) => Err(e.to_string()),
    }
}
