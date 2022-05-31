use rand::RngCore;
use argon2::{self, Config};

/// Generate a random number of 16 bytes / 128 bits
/// Used to generate salt
/// # Arguments
/// * `bytes` - bytes to fill with random number
pub fn generate_random_16_bytes(bytes: &mut [u8; 16]) {
    let mut rng = rand::thread_rng();
    rng.fill_bytes(bytes);
}

/// Hash a given data (password) with salt using argon2 algorithme
/// We assume that the hash function will always works
/// # Arguments
/// * `data` - data to hash (mostly passwords)
/// * `salt` - salt used to hash the data
/// # Returns
/// * `String` - The hash generated
pub fn hash_argon2(data: &str, salt: &[u8]) -> String {
    argon2::hash_encoded(data.as_bytes(), salt, &Config::default()).unwrap()
}

/// Create a new hash and salt for a given password
/// # Arguments
/// * `password` - password to hash
/// # Returns
/// * `([u8;16], String)` - A tuple containing the salt and the hash
pub fn new_hash_password(password: &str) -> ([u8; 16], String) {
    let mut salt: [u8; 16] = [0; 16];
    generate_random_16_bytes(&mut salt);
    let hash_password = hash_argon2(password, &mut salt);
    (salt, hash_password)
}