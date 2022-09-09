use anyhow::Result;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

pub fn hash(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(password.as_bytes(), &salt)?
        .to_string();
    Ok(hashed_password)
}

pub fn verify(password: &str, hashed_password: &str) -> Result<(), argon2::password_hash::Error> {
    let hashed_password = PasswordHash::new(&hashed_password)?;
    Argon2::default().verify_password(password.as_bytes(), &hashed_password)
}
