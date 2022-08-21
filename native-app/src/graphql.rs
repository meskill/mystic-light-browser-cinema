use std::fs;

use async_graphql::{extensions::Tracing, EmptySubscription, Schema};
use mystic_light_sdk::{
    build_graphql_schema, MysticLightGraphqlMutation, MysticLightGraphqlQuery, MysticLightSDK,
};

pub type MysticLightSchema =
    Schema<MysticLightGraphqlQuery, MysticLightGraphqlMutation, EmptySubscription>;

pub fn create_qraphql_schema(sdk: MysticLightSDK) -> MysticLightSchema {
    let (query, mutation) = build_graphql_schema(sdk);

    let schema = Schema::build(query, mutation, EmptySubscription)
        .extension(Tracing)
        .finish();

    if let Err(error) = fs::write("./schema.graphql", schema.sdl()) {
        tracing::warn!("{}", error);
    }

    schema
}
