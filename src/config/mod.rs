use std::fs;
use std::path::PathBuf;
use std::process::exit;

use serde::{Deserialize, Serialize};

const CONFIG_PATH: &'static str = "server.properties";

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct RAstraConfig {
    pub display_name: String,
    pub display_sub_name: String,
    pub threads: usize,
    pub log_to_file: bool,
    pub logs_directory: PathBuf,
    pub level_name: String,
    pub level_seed: u64,
}

impl Default for RAstraConfig {
    fn default() -> Self {
        Self {
            display_name: String::from("Dedicated Server"),
            display_sub_name: String::from("RAstra"),
            threads: 4,
            log_to_file: true,
            logs_directory: PathBuf::from("logs"),
            level_name: String::from("Bedrock Level"),
            level_seed: 0,
        }
    }
}

pub fn setup_config() -> RAstraConfig {
    if PathBuf::from(CONFIG_PATH).exists() {
        let text = fs::read_to_string(CONFIG_PATH).unwrap_or_else(|err| {
            eprintln!(
                "An unexpected Error occurred while trying to read {CONFIG_PATH:?}, Err: {err:?}"
            );
            exit(1);
        });

        toml::from_str(&text).unwrap_or_else(|err| {
            eprintln!("An unexpected Error occurred while trying to deserialize {CONFIG_PATH:?}, Err: {err:?}");
            exit(1);
        })
    } else {
        let config = RAstraConfig::default();

        let text = toml::to_string(&config).unwrap_or_else(|err| {
            eprintln!(
                "An unexpected Error occurred while trying to serialize {config:?}, Err: {err:?}"
            );
            exit(1);
        });

        fs::write(CONFIG_PATH, text).unwrap_or_else(|err| {
            eprintln!("An unexpected Error occurred while trying to write the missing config to {CONFIG_PATH:?}, Err: {err:?}");
        });

        config
    }
}
