struct RegistrationUsecase {}

struct User {
    username: String,
}

impl RegistrationUsecase {
    fn new() -> Self {
        Self {}
    }

    async fn register(&self, username: &str, password: &str) -> Result<User, ()> { 
        Ok(User { username: username.into(), }) 
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[tokio::test]
    async fn test_registration() {
        let usecase = RegistrationUsecase::new();
        let user = usecase.register("flygrounder", "12345").await.unwrap();
        assert_eq!(user.username, "flygrounder");
    }
}
