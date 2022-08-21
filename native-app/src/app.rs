use async_graphql_axum::{GraphQLBatchRequest, GraphQLResponse};
use axum::{extract::Extension, routing::get, routing::post, Router};
use mystic_light_sdk::{CommonError, MysticLightSDK};
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

const LIB_PATH: &str = if cfg!(target_arch = "x86_64") {
    "./sdk/MysticLight_SDK_x64.dll"
} else {
    "./sdk/MysticLight_SDK.dll"
};

pub fn create_app() -> Result<Router, CommonError> {
    let sdk = MysticLightSDK::new(LIB_PATH)?;

    let schema = create_qraphql_schema(sdk);

    Ok(Router::new()
        .route("/mystic_light", post(graphql))
        .route("/healthz", get(healthz))
        .layer(TraceLayer::new_for_http())
        .layer(Extension(schema))
        .layer(
            CorsLayer::new()
                .allow_methods(Any)
                .allow_origin(Any)
                .allow_headers(Any),
        ))
}
