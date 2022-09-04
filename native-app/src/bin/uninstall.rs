use std::thread;
use std::{error::Error, time::Duration};

use windows_service::{
    service::{ServiceAccess, ServiceState},
    service_manager::{ServiceManager, ServiceManagerAccess},
};

use mystic_light_browser_cinema::{constants::SERVICE_NAME, log::start_logging};

const MAX_ATTEMPTS: u32 = 5;
const SERVICE_STOP_TIMEOUT: u64 = 5;

fn main() {
    start_logging();

    if let Err(error) = uninstall_service() {
        tracing::error!("Failed to uninstall service because of error: {error}");
    }
}

fn uninstall_service() -> Result<(), Box<dyn Error>> {
    let service_manager =
        ServiceManager::local_computer(None::<&str>, ServiceManagerAccess::CONNECT)?;

    let service = service_manager.open_service(
        SERVICE_NAME,
        ServiceAccess::QUERY_STATUS | ServiceAccess::STOP | ServiceAccess::DELETE,
    )?;

    for count in 0u32.. {
        if count >= MAX_ATTEMPTS {
            tracing::error!("Couldn't stop service after {MAX_ATTEMPTS} attempts");
            break;
        }

        let current_state = service.query_status()?.current_state;

        tracing::info!("Current state is {current_state:?}");

        if current_state == ServiceState::Stopped {
            break;
        }

        tracing::info!("Trying to stop service");

        service.stop()?;

        thread::sleep(Duration::from_secs(SERVICE_STOP_TIMEOUT));
    }

    tracing::info!("Deleting service");

    service.delete()?;

    Ok(())
}
