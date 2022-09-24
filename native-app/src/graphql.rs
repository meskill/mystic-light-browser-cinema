use async_graphql::{extensions::Tracing, EmptySubscription, Schema};
use mystic_light_sdk::{
    build_graphql_schema, MysticLightGraphqlMutation, MysticLightGraphqlQuery, MysticLightSDK,
};

pub type MysticLightSchema =
    Schema<MysticLightGraphqlQuery, MysticLightGraphqlMutation, EmptySubscription>;

pub fn create_qraphql_schema(sdk: MysticLightSDK) -> MysticLightSchema {
    let (query, mutation) = build_graphql_schema(sdk);

    Schema::build(query, mutation, EmptySubscription)
        .extension(Tracing)
        .finish()
}
