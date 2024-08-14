use ring::pbkdf2;
use std::num::NonZeroU32;

const ITERATIONS: u32 = 10000;

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