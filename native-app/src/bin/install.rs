use std::env;
use std::error::Error;

use mystic_light_browser_cinema::{log::start_logging, service::install_service};

fn main() {
    start_logging();

    tracing::info!("Installing service");

    if let Err(error) = run() {
        tracing::error!("Failed to install service because of error: {error}");
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let executable_path = env::current_exe()?.with_file_name("service.exe");

    install_service(executable_path)?;

    Ok(())
}
