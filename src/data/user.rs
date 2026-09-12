use argon2::{Argon2, PasswordHasher, PasswordVerifier, PasswordHash};

use anyhow::Result;

pub struct User {
    pub id: uuid::Uuid,
    pub username: String,
}

pub struct UserAuth {
    pub id: uuid::Uuid,
    pub username: String,
    pub pass_hash: String,
    pub jwt_v: i32,
}

pub fn hash_password(pass: &str) -> Result<String> {
    let hash = Argon2::default()
        .hash_password(pass.as_bytes())
        .map_err(|e| anyhow::anyhow!("password hashing failed: {e}"))?
        .to_string();

    Ok(hash)
}

pub fn check_hash(pass: &str, user: &UserAuth) -> bool {
    let parsed_hash = match PasswordHash::new(&user.pass_hash) {
        Ok(h) => h,
        Err(_) => return false,
    };

    Argon2::default()
        .verify_password(pass.as_bytes(), &parsed_hash)
        .is_ok()
}
