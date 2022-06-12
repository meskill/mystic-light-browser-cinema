mod app;
mod graphql;
mod props;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use app::create_app;
use props::Props;

#[cfg(debug_assertions)]
fn resolve_port() -> u16 {
    5000
}

#[cfg(not(debug_assertions))]
fn resolve_port() -> u16 {
    println!("random");
    portpicker::pick_unused_port().expect("Cannot resolve free port")
}

pub fn start_server() {
    let port = resolve_port();

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), port);

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .max_blocking_threads(1)
        .build()
        .unwrap();

    runtime.block_on(async {
        let app = create_app();

        let server = axum::Server::bind(&addr)
            .serve(app.expect("Cannot initialize sdk").into_make_service());

        let local_address = server.local_addr();

        log::info!("Server started at {}", local_address);

        let props = Props::new(local_address);

        props.save().unwrap();

        server.await.unwrap();
    })
}
