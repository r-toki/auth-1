use anyhow::Result;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

pub fn hash(password: String) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(password.as_bytes(), &salt)?
        .to_string();
    Ok(hashed_password)
}

pub fn verify(
    password: String,
    hashed_password: String,
) -> Result<(), argon2::password_hash::Error> {
    let hashed_password = PasswordHash::new(&hashed_password)?;
    Argon2::default().verify_password(password.as_bytes(), &hashed_password)
}
