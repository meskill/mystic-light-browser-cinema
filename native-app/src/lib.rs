mod app;
mod config;
pub mod constants;
mod graphql;
pub mod log;
mod props;
mod sdk;
pub mod service;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::thread;
use std::time::Duration;

use axum::Router;
use mystic_light_sdk::{CommonError, MysticLightSDK};
use tokio::sync::mpsc;

use app::create_app;
use props::Props;
use sdk::create_sdk;

const STATIC_PORT: u16 = 5000;
const MAX_ATTEMPTS_SDK_INIT: u8 = 1;
const ATTEMPTS_TIMEOUT: u64 = 5;

fn resolve_random_port() -> Option<u16> {
    portpicker::pick_unused_port()
}

pub struct AppBuilder {
    random_port: bool,
    shutdown_rx: Option<mpsc::Receiver<()>>,
    max_attempts_sdk_init: u8,
}

impl Default for AppBuilder {
    fn default() -> Self {
        Self {
            random_port: false,
            shutdown_rx: None,
            max_attempts_sdk_init: MAX_ATTEMPTS_SDK_INIT,
        }
    }
}

impl AppBuilder {
    pub fn new() -> Self {
        AppBuilder::default()
    }

    pub fn with_random_port(&mut self, random_port: bool) -> &mut Self {
        self.random_port = random_port;

        self
    }

    pub fn with_channel_rx(&mut self, shutdown_rx: mpsc::Receiver<()>) -> &mut Self {
        self.shutdown_rx = Some(shutdown_rx);

        self
    }

    pub fn with_max_attempts_sdk_init(&mut self, max_attempts: u8) -> &mut Self {
        assert!(max_attempts > 0, "max_attempts should be more than 0");

        self.max_attempts_sdk_init = max_attempts;

        self
    }

    pub fn build(&mut self) -> Result<App, CommonError> {
        let port = if self.random_port {
            resolve_random_port().expect("Cannot resolve port")
        } else {
            STATIC_PORT
        };

        let mut attempts = 0u8;

        let sdk = loop {
            attempts += 1;

            tracing::info!("trying to resolve sdk for {attempts} attempt");

            let sdk = create_sdk();

            if sdk.is_ok() || attempts == self.max_attempts_sdk_init {
                break sdk;
            }

            thread::sleep(Duration::from_secs(ATTEMPTS_TIMEOUT));
        }?;

        App::new(sdk, port, self.shutdown_rx.take())
    }
}

pub struct App {
    port: u16,
    router: Router,
    shutdown_rx: Option<mpsc::Receiver<()>>,
}

impl App {
    fn new(
        sdk: MysticLightSDK,
        port: u16,
        shutdown_rx: Option<mpsc::Receiver<()>>,
    ) -> Result<Self, CommonError> {
        let router = create_app(sdk);

        Ok(Self {
            router,
            port,
            shutdown_rx,
        })
    }

    pub fn listen(self) -> Result<(), Box<dyn std::error::Error>> {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), self.port);

        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .max_blocking_threads(1)
            .build()?;

        runtime.block_on(async {
            let server = axum::Server::bind(&addr).serve(self.router.into_make_service());

            let local_address = server.local_addr();

            tracing::info!("Server started at {}", local_address);

            let props = Props::new(local_address);

            props.save()?;

            if let Some(mut shutdown_rx) = self.shutdown_rx {
                let graceful = server.with_graceful_shutdown(async {
                    shutdown_rx.recv().await;

                    tracing::info!("Shutting down the server");
                });

                graceful.await?;
            } else {
                server.await?;
            }

            Ok(())
        })
    }
}
