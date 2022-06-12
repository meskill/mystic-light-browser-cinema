use std::env;
use std::fs;
use std::net::SocketAddr;
use std::path::PathBuf;

use serde::Serialize;

fn resolve_config_dir() -> PathBuf {
    let mut path = if let Ok(dir) = env::var("ProgramData") {
        PathBuf::from(dir)
    } else {
        PathBuf::from("C:\\\\ProgramData")
    };

    path.push("Mystic Light");
    path.push("Mystic Light Browser Cinema");

    path
}

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
        let config_dir = resolve_config_dir();

        fs::create_dir_all(&config_dir)?;

        fs::write(config_dir.join("props.json"), serde_json::to_string(&self)?)?;

        Ok(())
    }
}
