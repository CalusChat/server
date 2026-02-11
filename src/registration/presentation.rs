use async_graphql::{Context, InputObject, Object, Result};

use crate::registration::{
    application::RegistrationUsecase,
    infrastructure::{Argon2PasswordHasher, PostgresUserRepository},
};

#[derive(InputObject)]
struct CreateAccountInput {
    username: String,
    password: String,
}

pub struct RegistrationMutation;

impl Default for RegistrationMutation {
    fn default() -> Self {
        Self::new()
    }
}

impl RegistrationMutation {
    pub fn new() -> Self {
        Self
    }
}

#[Object]
impl RegistrationMutation {
    async fn create_account(&self, ctx: &Context<'_>, input: CreateAccountInput) -> Result<i64> {
        let res = ctx
            .data::<RegistrationUsecase<Argon2PasswordHasher, PostgresUserRepository>>()?
            .register(&input.username, &input.password)
            .await
            .map_err(|_| "failed to register user")?;
        Ok(res)
    }
}
