use async_graphql_axum::{GraphQLBatchRequest, GraphQLResponse};
use axum::{extract::Extension, routing::get, routing::post, Router};
use mystic_light_sdk::MysticLightSDK;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

use crate::graphql::{create_qraphql_schema, MysticLightSchema};

async fn graphql(
    schema: Extension<MysticLightSchema>,
    req: GraphQLBatchRequest,
) -> GraphQLResponse {
    schema.execute_batch(req.into_inner()).await.into()
}

async fn healthz() -> &'static str {
    "Ok"
}

pub fn create_app(sdk: MysticLightSDK) -> Router {
    let schema = create_qraphql_schema(sdk);

    Router::new()
        .route("/mystic_light", post(graphql))
        .route("/healthz", get(healthz))
        .layer(TraceLayer::new_for_http())
        .layer(Extension(schema))
        .layer(
            CorsLayer::new()
                .allow_methods(Any)
                .allow_origin(Any)
                .allow_headers(Any),
        )
}
