use std::error::Error;
use std::ffi::OsString;
use std::{env, ffi::OsStr};

use windows_service::service::ServiceDependency;
use windows_service::{
    service::{ServiceAccess, ServiceErrorControl, ServiceInfo, ServiceStartType, ServiceType},
    service_manager::{ServiceManager, ServiceManagerAccess},
};

use mystic_light_browser_cinema::{
    constants::{SERVICE_DISPLAY_NAME, SERVICE_NAME},
    log::start_logging,
};

fn main() {
    start_logging();

    let span = tracing::info_span!("service_handler");

    let _enter = span.enter();

    if let Err(error) = install_service() {
        tracing::error!("Failed to install service because of error: {error}");
    }
}

fn install_service() -> Result<(), Box<dyn Error>> {
    let service_manager = ServiceManager::local_computer(
        None::<&str>,
        ServiceManagerAccess::CONNECT | ServiceManagerAccess::CREATE_SERVICE,
    )?;

    let executable_path = env::current_exe()?.with_file_name("service.exe");

    tracing::info!("Running executable with path {executable_path:?}");

    let service_info = ServiceInfo {
        name: OsString::from(SERVICE_NAME),
        display_name: OsString::from(SERVICE_DISPLAY_NAME),
        executable_path,
        service_type: ServiceType::OWN_PROCESS,
        start_type: ServiceStartType::AutoStart,
        error_control: ServiceErrorControl::Normal,
        launch_arguments: vec![],
        dependencies: vec![ServiceDependency::Service(OsString::from(
            "Mystic_Light_Service",
        ))],
        account_name: None,
        account_password: None,
    };

    let service = service_manager.create_service(
        &service_info,
        ServiceAccess::CHANGE_CONFIG | ServiceAccess::START,
    )?;

    service.set_description("Companion service for Mystic Light Browser Cinema extension")?;

    tracing::info!("Installing successful");

    service.start::<&OsStr>(&[])?;

    Ok(())
}
