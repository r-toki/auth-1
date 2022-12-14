use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

pub fn hash(password: &str) -> anyhow::Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| anyhow::anyhow!("could not hash"))?
        .to_string();
    Ok(hashed_password)
}

pub fn verify(password: &str, hashed_password: &str) -> anyhow::Result<()> {
    let hashed_password = PasswordHash::new(&hashed_password).map_err(|_| anyhow::anyhow!(""))?;
    Argon2::default()
        .verify_password(password.as_bytes(), &hashed_password)
        .map_err(|_| anyhow::anyhow!("could not verify"))
}
