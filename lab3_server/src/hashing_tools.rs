use rand::RngCore;
use argon2::{self, Config};

/// Used to generate salt
pub fn generate_random_16_bytes(bytes: &mut [u8; 16]) {
    let mut rng = rand::thread_rng();
    rng.fill_bytes(bytes);
}

/// We assume that the hash function will always works
pub fn hash_argon2(data: &str, salt: &[u8]) -> String {
    argon2::hash_encoded(data.as_bytes(), salt, &Config::default()).unwrap()
}