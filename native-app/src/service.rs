use std::ffi::OsStr;
use std::ffi::OsString;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

use windows_service::service::ServiceAction;
use windows_service::service::ServiceActionType;
use windows_service::service::ServiceFailureActions;
use windows_service::service::ServiceFailureResetPeriod;
use windows_service::{
    service::{
        Service, ServiceAccess, ServiceDependency, ServiceErrorControl, ServiceInfo,
        ServiceStartType, ServiceState, ServiceType,
    },
    service_manager::{ServiceManager, ServiceManagerAccess},
    Result as ServiceResult,
};

use crate::constants::{SERVICE_DISPLAY_NAME, SERVICE_NAME};

const MAX_ATTEMPTS: u32 = 5;
const SERVICE_STOP_TIMEOUT: u64 = 5;

fn stop_service(service: &Service) -> ServiceResult<()> {
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

    Ok(())
}

pub fn install_service(executable_path: PathBuf) -> ServiceResult<()> {
    let service_manager = ServiceManager::local_computer(
        None::<&str>,
        ServiceManagerAccess::CONNECT | ServiceManagerAccess::CREATE_SERVICE,
    )?;

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

    let service = if let Ok(current_service) = service_manager.open_service(
        SERVICE_NAME,
        ServiceAccess::CHANGE_CONFIG
            | ServiceAccess::QUERY_STATUS
            | ServiceAccess::START
            | ServiceAccess::STOP,
    ) {
        tracing::info!("Service has already installed. Stop it if it is running");

        stop_service(&current_service)?;

        current_service
    } else {
        tracing::info!("Service is not installed. Installing now");

        service_manager.create_service(
            &service_info,
            ServiceAccess::CHANGE_CONFIG | ServiceAccess::START,
        )?
    };

    tracing::info!("Updating service config");

    service.change_config(&service_info)?;
    service.set_description("Companion service for Mystic Light Browser Cinema extension")?;

    let actions = vec![
        ServiceAction {
            action_type: ServiceActionType::Restart,
            delay: Duration::from_secs(60),
        },
        ServiceAction {
            action_type: ServiceActionType::Restart,
            delay: Duration::from_secs(300),
        },
        ServiceAction {
            action_type: ServiceActionType::None,
            delay: Duration::default(),
        },
    ];

    service.update_failure_actions(ServiceFailureActions {
        reset_period: ServiceFailureResetPeriod::After(Duration::from_secs(60)),
        actions: Some(actions),
        command: None,
        reboot_msg: None,
    })?;

    tracing::info!("Installing successful");

    tracing::info!("Starting the service");

    service.start::<&OsStr>(&[])?;

    Ok(())
}

pub fn uninstall_service() -> ServiceResult<()> {
    let service_manager =
        ServiceManager::local_computer(None::<&str>, ServiceManagerAccess::CONNECT)?;

    let service = service_manager.open_service(
        SERVICE_NAME,
        ServiceAccess::QUERY_STATUS | ServiceAccess::STOP | ServiceAccess::DELETE,
    )?;

    stop_service(&service)?;

    tracing::info!("Deleting service");

    service.delete()?;

    Ok(())
}
