use crate::config::RAstraConfig;
use bedrockrs::proto::login::login_to_server;
use bedrockrs::proto::login::provider::DefaultLoginProvider;
use bedrockrs::proto::{connection, listener};
use log::{debug, error, info};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::process::exit;
use std::str::FromStr;
use std::time::Duration;

mod config;
mod logger;

fn main() {
    let config = config::setup_config();

    logger::setup_logger(config.log_to_file, &config.logs_directory);

    debug!("Initialization started!");

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(config.threads)
        .enable_all()
        .build()
        .unwrap_or_else(|err| {
            error!("An unexpected Error occurred while trying to create the Tokio-Runtime, Err: {err:?}");
            exit(1)
        });

    runtime.block_on(async {
        debug!("Tokio-Runtime initialized!");
        rastra_main(config).await;
    });
}

async fn rastra_main(_config: RAstraConfig) {
    info!("Hello World!");

    let mut listener = listener::Listener::new_raknet(
        _config.display_name,
        _config.display_sub_name,
        100,
        0,
        SocketAddr::V4(SocketAddrV4::new(
            Ipv4Addr::from_str("127.0.0.1").unwrap(),
            19132,
        )),
        false,
    )
    .await
    .unwrap();

    listener.start().await.unwrap();

    loop {
        let conn = listener.accept().await.unwrap();

        tokio::spawn(handle_conn(conn));
    }
}

async fn handle_conn(connection: connection::Connection) {
    let mut shard = connection
        .into_shard(Duration::from_millis(1000 / 20), 256)
        .await;

    login_to_server(&mut shard, DefaultLoginProvider::new())
        .await
        .unwrap();

    loop {
        let pk = shard.recv().await;

        info!("{pk:#?}");

        if pk.is_err() {
            break;
        }
    }
}
