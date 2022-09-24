use mystic_light_browser_cinema::{graphql, sdk};
use std::error;
use std::fs;

fn main() -> Result<(), Box<dyn error::Error>> {
    let sdk = sdk::create_sdk()?;
    let schema = graphql::create_qraphql_schema(sdk);

    if let Err(error) = fs::write("./schema.graphql", schema.sdl()) {
        tracing::warn!("{}", error);
    }

    Ok(())
}
