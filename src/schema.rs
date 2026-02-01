use std::marker::PhantomData;

use async_graphql::{Context, InputObject, Object, Result, SimpleObject};

use crate::usecase::{PasswordHasher, RegistrationUsecase, UserRepository};

pub struct AppContext<P: PasswordHasher, U: UserRepository> {
    registration: RegistrationUsecase<P, U>,
}

#[derive(SimpleObject)]
pub struct QueryRoot {
    id: i32,
}

impl QueryRoot {
    pub fn new(id: i32) -> Self {
        Self {
            id
        }
    }
}

impl<P: PasswordHasher, U: UserRepository> AppContext<P, U> {
    pub fn new(registration: RegistrationUsecase<P, U>) -> Self {
        Self { registration }
    }
}

pub struct MutationRoot<P, U> {
    phantom_hasher: PhantomData<P>,
    phantom_user_repository: PhantomData<U>,
}

impl<P, U> MutationRoot<P, U> {
    pub fn new() -> Self {
        Self {
            phantom_hasher: PhantomData,
            phantom_user_repository: PhantomData,
        }
    }
}

#[derive(InputObject)]
struct CreateAccountInput {
    username: String,
    password: String,
}

#[Object]
impl<P: PasswordHasher + Send + Sync + 'static, U: UserRepository + Send + Sync + 'static>
    MutationRoot<P, U>
{
    async fn create_account(&self, ctx: &Context<'_>, input: CreateAccountInput) -> Result<i64> {
        let res = ctx
            .data::<AppContext<P, U>>()?
            .registration
            .register(&input.username, &input.password)
            .await
            .map_err(|_| "failed to register user")?;
        Ok(res)
    }
}
