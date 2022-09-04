use std::{ffi::OsString, time::Duration};

use tokio::sync::mpsc;
use windows_service::{
    define_windows_service,
    service::{
        ServiceControl, ServiceControlAccept, ServiceExitCode, ServiceState, ServiceStatus,
        ServiceType,
    },
    service_control_handler::{self, ServiceControlHandlerResult},
    service_dispatcher,
};

use mystic_light_browser_cinema::{constants::SERVICE_NAME, log::start_logging, AppBuilder};

const SERVICE_SDK_START_TIMEOUT: u64 = 60;
const SERVICE_LISTEN_START_TIMEOUT: u64 = 20;

const EXIT_CODE_OK: u32 = 0;
const EXIT_CODE_SDK_ERROR: u32 = 11;
const EXIT_CODE_SERVER_ERROR: u32 = 21;

define_windows_service!(ffi_service_main, service_main);

fn main() -> windows_service::Result<()> {
    service_dispatcher::start(SERVICE_NAME, ffi_service_main)?;
    Ok(())
}

fn service_main(_arguments: Vec<OsString>) {
    start_logging();

    if let Err(error) = run_service() {
        tracing::error!("Got error: {error} while running service");
    }
}

fn run_service() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx) = mpsc::channel::<()>(1);

    let status_handle =
        service_control_handler::register(
            SERVICE_NAME,
            move |control_event| match control_event {
                ServiceControl::Stop => {
                    if let Err(error) = tx.blocking_send(()) {
                        tracing::error!("Error while sending message: {error}");
                    }

                    ServiceControlHandlerResult::NoError
                }
                ServiceControl::Interrogate => ServiceControlHandlerResult::NoError,
                _ => ServiceControlHandlerResult::NotImplemented,
            },
        )?;

    let default_service_status: ServiceStatus = ServiceStatus {
        service_type: ServiceType::OWN_PROCESS,
        current_state: ServiceState::Stopped,
        controls_accepted: ServiceControlAccept::STOP,
        process_id: None,
        checkpoint: 0,
        exit_code: ServiceExitCode::ServiceSpecific(EXIT_CODE_OK),
        wait_hint: Duration::default(),
    };

    tracing::info!("Create a service app");

    status_handle.set_service_status(ServiceStatus {
        current_state: ServiceState::StartPending,
        checkpoint: 1,
        wait_hint: Duration::from_secs(SERVICE_SDK_START_TIMEOUT),
        ..default_service_status
    })?;

    let app = AppBuilder::new()
        .with_random_port(true)
        // on system login sdk cannot initialize for some reason
        // despite providing service dependencies for Mystic_Light_Service
        // and I'm not quite sure what the problem
        // so code below will try to initialize sdk several time before throwing an error
        .with_max_attempts_sdk_init(5)
        .with_channel_rx(rx)
        .build()
        .map_err(|error| {
            let _ = status_handle.set_service_status(ServiceStatus {
                current_state: ServiceState::Stopped,
                exit_code: ServiceExitCode::ServiceSpecific(EXIT_CODE_SDK_ERROR),
                ..default_service_status
            });

            error
        })?;

    tracing::info!("Run the underlying server");

    status_handle.set_service_status(ServiceStatus {
        current_state: ServiceState::Running,
        checkpoint: 5,
        wait_hint: Duration::from_secs(SERVICE_LISTEN_START_TIMEOUT),
        ..default_service_status
    })?;

    if let Err(error) = app.listen() {
        status_handle.set_service_status(ServiceStatus {
            current_state: ServiceState::Stopped,
            exit_code: ServiceExitCode::ServiceSpecific(EXIT_CODE_SERVER_ERROR),
            ..default_service_status
        })?;

        return Err(error);
    }

    tracing::info!("Mark service as stopped");

    status_handle.set_service_status(ServiceStatus {
        current_state: ServiceState::Stopped,
        checkpoint: 10,
        exit_code: ServiceExitCode::default(),
        ..default_service_status
    })?;

    Ok(())
}
