mod impl_service;
mod log_buffer;
mod pool;

use daemonize::Daemonize;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::pool::Pool;

fn main() -> Result<(), Box<dyn Error>> {
    Daemonize::new().start()?;

    // Build tokio runtime after daemonizing. It causes a problem that client tries to
    // connect to the server while the runtime or the server didn't not spawn yet.
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            start_server().await.unwrap();
        });

    Ok(())
}

async fn start_server() -> Result<(), Box<dyn Error>> {
    use futures::{future, prelude::*};
    use impl_service::Pm3Server;
    use pm3::rpc::Pm3;
    use std::net::{IpAddr, Ipv6Addr, SocketAddr};
    use tarpc::{
        server::{self, incoming::Incoming, Channel},
        tokio_serde::formats::Json,
    };
    let server_addr = SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), 9876);
    let transport = tarpc::serde_transport::tcp::listen(server_addr, Json::default);
    let mut listener = transport.await?;
    listener.config_mut().max_frame_length(usize::MAX);
    let pool = Arc::new(Mutex::new(Pool::new()));
    listener
        .filter_map(|r| future::ready(r.ok()))
        .map(server::BaseChannel::with_defaults)
        .max_channels_per_key(1, |t| t.transport().peer_addr().unwrap().ip())
        .map(|channel| {
            let addr = channel.transport().peer_addr().unwrap();
            let server = Pm3Server::new(addr, Arc::clone(&pool));
            channel.execute(server.serve())
        })
        .buffer_unordered(10)
        .for_each(|_| async {})
        .await;

    Ok(())
}
