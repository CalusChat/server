use crate::registration::application::CreateUserRepository;
use argon2::{
    Argon2,
    password_hash::{PasswordHasher as _, SaltString, rand_core::OsRng},
};

use crate::registration::application::PasswordHasher;

pub struct Argon2PasswordHasher;

impl Default for Argon2PasswordHasher {
    fn default() -> Self {
        Self::new()
    }
}

impl Argon2PasswordHasher {
    pub fn new() -> Self {
        Self
    }
}

impl PasswordHasher for Argon2PasswordHasher {
    fn hash_password(&self, password: &str) -> Result<String, ()> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        Ok(argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| ())?
            .to_string())
    }
}

use sqlx::PgPool;

pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl CreateUserRepository for PostgresUserRepository {
    async fn create_user(&self, username: &str, hashed_password: &str) -> Result<i64, ()> {
        let result = sqlx::query!(
            "insert into users (username, password_hash) values ($1, $2) returning id",
            username,
            hashed_password
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|_| ())?;

        Ok(result.id)
    }
}
