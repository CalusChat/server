use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher as _, SaltString
    },
    Argon2
};

use crate::usecase::PasswordHasher;

pub struct Argon2PasswordHasher;

impl Argon2PasswordHasher {
    pub fn new() -> Self {
        Self
    }
}

impl PasswordHasher for Argon2PasswordHasher {
    fn hash_password(&self, password: &str) -> Result<String, ()> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        return Ok(argon2.hash_password(password.as_bytes(), &salt).map_err(|_| ())?.to_string());
    }
}
