use std::fs;
use std::path::Path;
use std::process::exit;

use chrono::Local;

pub fn setup_logger(log_to_file: bool, log_path: &Path) {
    let mut dispatch = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout());

    if log_to_file {
        let log_file = format!(
            "{}/{}.log",
            log_path.display(),
            Local::now().format("%Y-%m-%d_%H-%M-%S")
        );

        if !log_path.exists() {
            fs::create_dir(log_path).unwrap_or_else(|err| {
                eprintln!("An unexpected Error occurred while trying to create the logs directory at {log_path:?}, Err: {err:?}");
                exit(1)
            });
        }

        fs::write(&log_file, []).unwrap_or_else(|err| {
            eprintln!("An unexpected Error occurred while trying to create a log file at {log_file:?}, Err: {err:?}");
            exit(1)
        });

        dispatch = dispatch.chain(fern::log_file(&log_file).unwrap_or_else(|err| {
            eprintln!("An unexpected Error occurred while trying to add a log file at {log_file:?} to the logger, Err: {err:?}");
            exit(1)
        }));
    }

    dispatch.apply().unwrap_or_else(|err| {
        eprintln!("An unexpected Error occurred while trying to setup the logger, Err: {err:?}");
        exit(1);
    });
}
