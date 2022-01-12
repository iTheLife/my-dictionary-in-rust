#[path = "services/dictionary.rs"]
mod dictionary;
mod router;
mod state;

use crate::state::State;

use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    //let state = State::init();

    let addr = SocketAddr::from(([127, 0, 0, 1], 5000));

    let state = Arc::new(State::init());

    let make_svc = make_service_fn(|_conn| {
        //let dictionary = state.Dictionary.clone();

        let state = state.clone();

        async move {
            Ok::<_, Infallible>(service_fn(move |f| {
                let state = state.clone();
                router::router_service(f, state)
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    let graceful_shutdown = server.with_graceful_shutdown(shutdown_signal());

    // Run this server for... forever!
    if let Err(e) = graceful_shutdown.await {
        eprintln!("server error: {}", e);
    }
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Main.shutdown_signal - failed to install CTRL+C signal handler");

    println!("Server shutting down")
}
