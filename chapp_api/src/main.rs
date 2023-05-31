use std::{convert::Infallible, net::SocketAddr};

use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    Schema,
};
use async_graphql_warp::{graphql_subscription, GraphQLResponse};
use graphql::schema::{Mutation, Query, Subscription};
use storage::storage::Storage;
use ulid::Ulid;
use warp::{http::Response as HttpResponse, Filter};

use crate::graphql::schema::UserID;

mod graphql;
mod storage;

async fn warp_server(schema: Schema<Query, Mutation, Subscription>) {
    let cors_filter = warp::cors()
        .allow_any_origin()
        .allow_headers(["user_id", "content-type"])
        .allow_methods(["GET", "POST", "OPTIONS"])
        .allow_credentials(true);

    let graphql_post = warp::header::optional::<String>("user_id")
        .and(async_graphql_warp::graphql(schema.clone()))
        .and_then(
            |user_id: Option<String>,
             (schema, request): (
                Schema<Query, Mutation, Subscription>,
                async_graphql::Request,
            )| async move {
                let user_id = user_id
                    .map(|v: String| Ulid::from_string(&v).ok())
                    .flatten();
                let request = if let Some(user_id) = user_id {
                    request.data(UserID(user_id))
                } else {
                    request
                };
                Ok::<_, Infallible>(GraphQLResponse::from(schema.execute(request).await))
            },
        )
        .with(warp::log("graphql_post"));
    let playground = warp::path::end().and(warp::get()).map(|| {
        HttpResponse::builder()
            .header("content-type", "text/html")
            .body(playground_source(
                GraphQLPlaygroundConfig::new("/")
                    .subscription_endpoint("/")
                    .title("Chapp"),
            ))
    });
    let routes = graphql_subscription(schema)
        .with(warp::log("graphql_subscription"))
        .or(playground)
        .or(graphql_post)
        .with(cors_filter);
    let addr: SocketAddr = "[::]:8000".parse().unwrap();
    tracing::info!(message = "Starting server.", %addr);
    warp::serve(routes).run(addr).await;
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    let schema = Schema::build(Query, Mutation, Subscription)
        .data(Storage::default())
        .finish();
    warp_server(schema).await;
}
