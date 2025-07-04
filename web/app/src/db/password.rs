use argon2::{
    password_hash::{
        // `OsRng` requires enabled `std` crate feature
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::try_from_rng(&mut OsRng)
        .map_err(|_| argon2::password_hash::Error::Crypto)?;
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(password_hash.to_string())
}

pub fn verify_password(password: &str, hash: &str) -> Result<(), argon2::password_hash::Error> {
    let argon2 = Argon2::default();
    let password_hash = PasswordHash::new(hash)?;
    argon2.verify_password(password.as_bytes(), &password_hash)
}
