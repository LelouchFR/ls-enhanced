pub mod config;
pub mod format;
pub mod types;

use crate::{config::Config, format::format_ls};
use std::{env, fs, process::exit};
use toml;

fn main() -> std::io::Result<()> {
    let config_file: String = format!(
        "{}/.config/lse/config.toml",
        env::var("HOME").expect("HOME environment variable is not set")
    );

    let contents = match fs::read_to_string(&config_file) {
        Ok(c) => c,
        Err(_) => {
            let _ = config::generate_config(format!(
                "{}/.config/lse/config.toml",
                env::var("HOME").expect("HOME environment variable is not set")
            ));
            exit(1);
        }
    };

    let config: Config = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Unable to load config from {}\n{}", config_file, e);
            exit(1);
        }
    };

    format_ls(config, ".".to_string())
}
