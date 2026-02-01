use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema, http::GraphiQLSource};
use async_graphql_axum::GraphQL;
use axum::{
    Router,
    response::{self, IntoResponse},
    routing::get,
};
use tokio::net::TcpListener;

async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

struct MyQuery;

#[Object]
impl MyQuery {
    async fn id(&self) -> i32 { 123 }
}

#[tokio::main]
async fn main() {
    let schema = Schema::build(MyQuery{}, EmptyMutation, EmptySubscription)
        .finish();

    let app = Router::new().route("/", get(graphiql).post_service(GraphQL::new(schema)));

    println!("GraphiQL IDE: http://localhost:8000");

    axum::serve(TcpListener::bind("127.0.0.1:8000").await.unwrap(), app)
        .await
        .unwrap();
}
