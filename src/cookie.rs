use openssl::symm::{encrypt, Cipher, decrypt};
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;
use crate::config::Config;
use openssl::sha::sha256;

fn gen_iv() -> Vec<u8> {
    let mut buf = [0x00; 16];
    let mut rng = ChaCha20Rng::from_entropy();
    rng.fill(&mut buf);
    buf.to_vec()
}

fn hash_secret(config: &Config) -> Vec<u8> {
    let sha = sha256(config.secret.as_bytes());
    let mut output = [0x00u8; 16];
    let mut idx = 0;
    for b in &sha {
        output[idx % 16] = output[idx % 16] ^ b;
        idx += 1;
    }

    Vec::from(output)
}

pub fn create(config: &Config, data: String) -> Option<String> {
    let cipher = Cipher::aes_128_cbc();
    let data = data.as_bytes();
    let iv = &*gen_iv();
    let encrypted = match encrypt(cipher, &hash_secret(config), Some(&iv), data) {
        Err(e) => {
            println!("{}", e);
            return None
        },
        Ok(v) => v
    };
    let mut result = vec![];
    result.extend_from_slice(iv);
    result.append(&mut vec![0x00]);
    result.extend(encrypted);
    Some(base64::encode_config(result, base64::URL_SAFE_NO_PAD))
}

pub fn decode(config: &Config, data: String) -> Option<String> {
    let bytes = match base64::decode_config(data, base64::URL_SAFE_NO_PAD) {
        Err(_) => return None,
        Ok(v) => v,
    };

    let arrs = bytes.split(|b| b == &0x00u8).collect::<Vec<&[u8]>>();
    if arrs.len() != 2 {
        return None;
    }

    let iv = arrs[0];
    let encrypted = arrs[1];
    let cipher = Cipher::aes_128_cbc();
    let decrypted = match decrypt(cipher, &hash_secret(config), Some(iv), encrypted) {
        Err(_) => return None,
        Ok(v) => v,
    };

    match String::from_utf8(decrypted) {
        Err(_) => None,
        Ok(v) => Some(v),
    }
}

#[cfg(test)]
mod test_cookie {
    use crate::config::Config;
    use crate::cookie::{create, decode};

    #[test]
    fn test_roundtrip() {
        let config = Config{
            allowed_users: vec![],
            github_secret: "".to_string(),
            github_key: "".to_string(),
            secret: "Test".to_string(),
        };

        let result = create(&config, "Hello World".to_string());
        assert_ne!(result, None);

        let decoded = decode(&config, result.unwrap());
        assert_ne!(decoded, None);
        assert_eq!(decoded.unwrap(), "Hello World");
    }
}
