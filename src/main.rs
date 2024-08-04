use std::process::exit;
use log::{error, info};

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
            error!("An unexpected Error occurred while trying to create the tokio runtime, Err: {err:?}");
            exit(1)
        });

    runtime.block_on(async {
        info!("Tokio-Runtime started!");
        rastra_main(config).await;
    });
}

async fn rastra_main(_config: RAstraConfig) {
    info!("Hello World!")
}
