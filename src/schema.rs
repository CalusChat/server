use async_graphql::{MergedObject, SimpleObject};

use crate::registration::presentation::RegistrationMutation;

#[derive(SimpleObject)]
pub struct QueryRoot {
    id: i32,
}

impl QueryRoot {
    pub fn new(id: i32) -> Self {
        Self { id }
    }
}

#[derive(MergedObject)]
pub struct MutationRoot(RegistrationMutation);

impl MutationRoot {
    pub fn new(registration: RegistrationMutation) -> Self {
        Self(registration)
    }
}
