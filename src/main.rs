pub mod config;
pub mod format;
pub mod types;

use crate::{config::Config, format::format_ls};
use std::fs;
use std::process::exit;
use std::env;
use toml;

fn main() -> std::io::Result<()> {
    let config_file: String = format!("{}/.config/lse/config.toml", env::var("HOME").expect("HOME environment is not set"));

    let contents = match fs::read_to_string(&config_file) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Could not find config file {}", config_file);
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
