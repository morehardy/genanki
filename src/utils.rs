use chrono::Utc;
use sha2::{Digest, Sha256};
pub fn generate_id() -> i64 {
    Utc::now().timestamp_millis()
}
const BASE91: &[u8] =
    b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!#$%&()*+,-./:;<=>?@[]^_`{|}~";
pub fn generate_guid(fields: &Vec<String>) -> String {
    let hash_str = fields.join("__");
    let mut n = u64::from_be_bytes(Sha256::digest(hash_str.as_bytes())[..8].try_into().unwrap());
    let mut result = Vec::with_capacity(12);

    while n > 0 {
        result.push(BASE91[(n % 91) as usize]);
        n /= 91;
    }
    String::from_utf8(result.into_iter().rev().collect()).unwrap()
}
