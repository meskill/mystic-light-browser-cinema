use std::env;
use std::path::PathBuf;

pub fn resolve_config_dir() -> PathBuf {
    let mut path = if let Ok(dir) = env::var("ProgramData") {
        PathBuf::from(dir)
    } else {
        PathBuf::from("C:\\\\ProgramData")
    };

    path.push("Mystic Light");
    path.push("Mystic Light Browser Cinema");

    path
}
