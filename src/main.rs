use async_graphql::{EmptySubscription, Schema, http::GraphiQLSource};
use async_graphql_axum::GraphQL;
use axum::{Router, response::{self, IntoResponse}, routing::get};
use server::{
    hasher::Argon2PasswordHasher,
    repository::PostgresUserRepository,
    schema::{AppContext, MutationRoot, QueryRoot},
    usecase::RegistrationUsecase,
};
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;

async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

#[tokio::main]
async fn main() {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost:5432/postgres")
        .await
        .expect("failed to connect to postgres");
    let hasher = Argon2PasswordHasher::new();
    let user_repository = PostgresUserRepository::new(pool);
    let registration_usecase = RegistrationUsecase::new(hasher, user_repository);
    let context = AppContext::new(registration_usecase);
    let schema = Schema::build(
        QueryRoot::new(123),
        MutationRoot::<Argon2PasswordHasher, PostgresUserRepository>::new(),
        EmptySubscription,
    )
    .data(context)
    .finish();

    let app = Router::new().route("/", get(graphiql).post_service(GraphQL::new(schema)));

    axum::serve(TcpListener::bind("127.0.0.1:8000").await.unwrap(), app)
        .await
        .unwrap();
}
