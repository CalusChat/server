use mockall::automock;

#[automock]
pub trait PasswordHasher {
    fn hash_password(&self, password: &str) -> Result<String, ()>;
}

#[automock]
pub trait UserRepository {
    fn create_user(
        &self,
        username: &str,
        hashed_password: &str,
    ) -> impl std::future::Future<Output = Result<i64, ()>> + Send;
}

pub struct RegistrationUsecase<P: PasswordHasher, U: UserRepository> {
    password_hasher: P,
    user_repository: U,
}

impl<P: PasswordHasher, U: UserRepository> RegistrationUsecase<P, U> {
    pub fn new(password_hasher: P, user_repository: U) -> Self {
        Self {
            password_hasher,
            user_repository,
        }
    }

    pub async fn register(&self, username: &str, password: &str) -> Result<i64, ()> {
        let hashed_password = self.password_hasher.hash_password(password)?;
        self.user_repository
            .create_user(username, &hashed_password)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_registration_username() {
        let mut mock_hasher = MockPasswordHasher::new();
        mock_hasher
            .expect_hash_password()
            .withf(|password| password == "12345")
            .returning(|_| Ok("hashed_password".to_string()));

        let mut mock_repo = MockUserRepository::new();
        mock_repo
            .expect_create_user()
            .withf(|username, hashed_password| {
                username == "flygrounder" && hashed_password == "hashed_password"
            })
            .returning(|_, _| Box::pin(async { Ok(1) }));

        let usecase = RegistrationUsecase::new(mock_hasher, mock_repo);
        let result = usecase.register("flygrounder", "12345").await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
    }
}
