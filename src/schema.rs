use async_graphql::{Context, InputObject, Object, Result, SimpleObject};

use crate::registration::{
    application::RegistrationUsecase,
    infrastructure::{Argon2PasswordHasher, PostgresUserRepository},
};

#[derive(SimpleObject)]
pub struct QueryRoot {
    id: i32,
}

impl QueryRoot {
    pub fn new(id: i32) -> Self {
        Self { id }
    }
}

pub struct MutationRoot;

impl MutationRoot {
    pub fn new() -> Self {
        Self
    }
}

#[derive(InputObject)]
struct CreateAccountInput {
    username: String,
    password: String,
}

#[Object]
impl MutationRoot {
    async fn create_account(&self, ctx: &Context<'_>, input: CreateAccountInput) -> Result<i64> {
        let res = ctx
            .data::<RegistrationUsecase<Argon2PasswordHasher, PostgresUserRepository>>()?
            .register(&input.username, &input.password)
            .await
            .map_err(|_| "failed to register user")?;
        Ok(res)
    }
}
