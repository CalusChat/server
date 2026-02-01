use crate::usecase::UserRepository;
use sqlx::PgPool;

pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl UserRepository for PostgresUserRepository {
    async fn create_user(&self, username: &str, hashed_password: &str) -> Result<i64, ()> {
        let result = sqlx::query!(
            "INSERT INTO users (username, hashed_password) VALUES ($1, $2) RETURNING id",
            username,
            hashed_password
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|_| ())?;

        Ok(result.id)
    }
}
