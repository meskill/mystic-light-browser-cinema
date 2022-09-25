use std::panic;

use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::fmt::format::FmtSpan;

const LOG_ENV_NAME: &str = "MYSTIC_LIGHT_LOG";
const LOG_DEFAULT_DIRECTIVES: &str =
    "error,mystic_light_sdk=debug,mystic_light_browser_cinema=debug,install=debug,uninstall=debug,service=debug";

pub fn start_logging() {
    let env_filter = EnvFilter::try_from_env(LOG_ENV_NAME)
        .unwrap_or_else(|_| EnvFilter::new(LOG_DEFAULT_DIRECTIVES));

    #[cfg(debug_assertions)]
    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter(env_filter)
        .with_span_events(FmtSpan::FULL)
        .init();

    #[cfg(not(debug_assertions))]
    {
        use crate::config;
        use file_rotate::{
            compression::Compression, suffix::AppendCount, ContentLimit, FileRotate,
        };

        use std::sync::Mutex;

        const LOG_FILES_LIMIT: usize = 5;
        const LOG_FILE_BYTES_LIMIT: usize = 20 * 1024 * 1024;
        const LOG_DIR_NAME: &str = "logs";
        const LOG_FILE_NAME: &str = "log.txt";

        let mut log_dir = config::resolve_config_dir();

        log_dir.push(LOG_DIR_NAME);
        log_dir.push(LOG_FILE_NAME);

        let log_rotation = FileRotate::new(
            log_dir,
            AppendCount::new(LOG_FILES_LIMIT),
            ContentLimit::Bytes(LOG_FILE_BYTES_LIMIT),
            Compression::OnRotate(0),
        );

        tracing_subscriber::fmt()
            .pretty()
            .with_env_filter(env_filter)
            .with_ansi(false)
            .with_span_events(FmtSpan::FULL)
            .with_writer(Mutex::new(log_rotation))
            .init();
    }

    panic::set_hook(Box::new(|panic_info| {
        let payload = panic_info.payload().downcast_ref::<&str>();
        let location = panic_info.location();

        tracing::error!("Panic at {location:?} with\n{payload:?}");
    }))
}
