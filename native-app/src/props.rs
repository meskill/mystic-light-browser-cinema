use std::env;
use std::fs;
use std::net::SocketAddr;

use serde::Serialize;

use crate::config;

#[derive(Serialize)]
pub struct Props {
    local_address: SocketAddr,
    version: &'static str,
}

impl Props {
    pub fn new(local_address: SocketAddr) -> Self {
        Self {
            local_address,
            version: env!("CARGO_PKG_VERSION"),
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_dir = config::resolve_config_dir();

        fs::create_dir_all(&config_dir)?;

        fs::write(config_dir.join("props.json"), serde_json::to_string(&self)?)?;

        Ok(())
    }
}
