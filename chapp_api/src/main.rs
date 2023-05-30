use std::net::SocketAddr;

use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    Schema,
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use axum::{
    extract::Extension,
    http::HeaderMap,
    response::{self, IntoResponse},
    routing::get,
    Router, Server,
};
use graphql::schema::{ChappSchema, Mutation, Query, Subscription};
use storage::storage::Storage;
use tower_http::{
    cors::CorsLayer,
    trace::{self, TraceLayer},
};
use ulid::Ulid;

use crate::graphql::schema::UserID;

mod graphql;
mod storage;

async fn graphql_handler(
    headers: HeaderMap,
    schema: Extension<ChappSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let user_id = headers
        .get("user_id")
        .and_then(|val| val.to_str().ok().map(Ulid::from_string).map(Result::ok))
        .flatten();
    let request = if let Some(user_id) = user_id {
        req.0.data(UserID(user_id))
    } else {
        req.into_inner()
    };
    schema.execute(request).await.into()
}

async fn gql_playground() -> impl IntoResponse {
    let config = GraphQLPlaygroundConfig::new("/")
        .subscription_endpoint("/subscriptions")
        .title("Chapp");
    response::Html(playground_source(config))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    let schema = Schema::build(Query, Mutation, Subscription)
        .data(Storage::default())
        .finish();

    let cors = CorsLayer::permissive();

    let trace = TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(tracing::Level::INFO))
        .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO));

    let app = Router::new()
        .route("/", get(gql_playground).post(graphql_handler))
        .route_service("/subscriptions", GraphQLSubscription::new(schema.clone()))
        .layer(Extension(schema))
        .layer(cors)
        .layer(trace);

    let addr: SocketAddr = "[::]:10000".parse().unwrap();
    tracing::info!(message = "Starting server.", %addr);
    Server::bind(&"[::]:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
