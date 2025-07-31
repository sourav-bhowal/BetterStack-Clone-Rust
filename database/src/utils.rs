use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand_core::OsRng;

/// Hashes a plain text password and returns the hash as a String.
/// Uses Argon2 with a secure random salt.
pub fn hash_password(password: String) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|e| format!("Hashing failed: {}", e))
}

/// Verifies a password against a stored Argon2 hash string.
/// Returns true if it matches, false otherwise.
pub fn verify_password(password: String, hashed: &str) -> bool {
    let parsed_hash = match PasswordHash::new(hashed) {
        Ok(h) => h,
        Err(_) => return false,
    };

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}
