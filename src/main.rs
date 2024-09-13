use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use log::{error, info};
use std::process::exit;
use std::str::FromStr;
use std::time::Duration;
use bedrock_rs::proto::login::login_to_server;
use bedrock_rs::proto::login::provider::DefaultLoginProvider;
use crate::config::RAstraConfig;

mod config;
mod logger;

fn main() {
    let config = config::setup_config();

    logger::setup_logger(config.log_to_file, &config.logs_directory);

    info!("Initialization started!");

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(config.threads)
        .enable_all()
        .build()
        .unwrap_or_else(|err| {
            error!("An unexpected Error occurred while trying to create the Tokio-Runtime, Err: {err:?}");
            exit(1)
        });

    runtime.block_on(async {
        info!("Tokio-Runtime initialized!");
        rastra_main(config).await;
    });
}

async fn rastra_main(_config: RAstraConfig) {
    info!("Hello World!");

    let mut listener = ::bedrock_rs::proto::listener::Listener::new_raknet(
        _config.display_name,
        _config.display_sub_name,
        100,
        0,
        SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::from_str("127.0.0.1").unwrap(), 19132)),
        false
    ).await.unwrap();

    listener.start().await.unwrap();

    loop {
        let conn = listener.accept().await.unwrap();

        tokio::spawn(handle_conn(conn));
    }
}

async fn handle_conn(connection: bedrock_rs::proto::connection::Connection) {
    let mut shard = connection.into_shard(Duration::from_millis(1000 / 20), 256).await;

    login_to_server(&mut shard, DefaultLoginProvider::new()).await.unwrap();

    loop {
        let pk = shard.recv().await;

        info!("{pk:#?}");

        if pk.is_err() {
            break;
        }
    }
}
