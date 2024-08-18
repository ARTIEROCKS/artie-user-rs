use ring::pbkdf2;
use std::num::NonZeroU32;
use rand::RngCore;
use rand::rngs::OsRng;
use num_bigint::BigUint;
use std::fmt::Write;

const ITERATIONS: u32 = 1000;

pub fn get_salt() -> [u8; 16] {
    let mut salt = [0u8; 16];
    let mut rng = OsRng;
    rng.fill_bytes(&mut salt);
    salt
}

pub fn to_hex(array: &[u8]) -> String {
    let bi = BigUint::from_bytes_be(array);
    let hex = bi.to_str_radix(16);

    let padding_length = (array.len() * 2).saturating_sub(hex.len());
    if padding_length > 0 {
        let mut padded_hex = String::with_capacity(array.len() * 2);
        write!(padded_hex, "{:0>width$}", 0, width = padding_length).unwrap();
        padded_hex.push_str(&hex);
        padded_hex
    } else {
        hex
    }
}

pub fn generate_password_hash(password: &str) -> String {

    let salt = get_salt();
    let mut hash = vec![0u8; 64]; // 64 bytes = 512 bits

    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA1,
        NonZeroU32::new(ITERATIONS).unwrap(),
        &salt,
        password.as_bytes(),
        &mut hash,
    );

    to_hex(&hash)
}

fn from_hex(hex: &str) -> Vec<u8> {
    hex::decode(hex).expect("Decoding hex string failed")
}

pub fn validate_password(original_password: &str, stored_password: &str, stored_salt: &str) -> bool {
    let salt = from_hex(stored_salt);
    let stored_hash = from_hex(stored_password);

    let mut test_hash = vec![0u8; stored_hash.len()];

    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA1,
        NonZeroU32::new(ITERATIONS).unwrap(),
        &salt,
        original_password.as_bytes(),
        &mut test_hash,
    );

    let mut diff = stored_hash.len() ^ test_hash.len();
    for (a, b) in stored_hash.iter().zip(test_hash.iter()) {
        diff |= usize::from(*a) ^ usize::from(*b);
    }

    diff == 0
}