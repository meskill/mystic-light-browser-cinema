use mystic_light_browser_cinema::{log::start_logging, service::uninstall_service};

fn main() {
    start_logging();

    tracing::info!("Uninstalling service");

    if let Err(error) = uninstall_service() {
        tracing::error!("Failed to uninstall service because of error: {error}");
    }
}
