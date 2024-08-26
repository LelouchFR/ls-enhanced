pub mod arguments;
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

    // let _ = arguments::create_arg();

    format_ls(config, ".".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn check_format_ls() {
        let temp_dir: &str = "/tmp/lse/";
        let temp_files: Vec<&str> = vec!["test.rs", "test.py"];

        let _ = fs::create_dir(temp_dir);

        for temp_file in temp_files {
            File::create(format!("{}/{}", temp_dir, temp_file)).expect("");
        }

        let config = config::create_config();

        let _ = format_ls(config, temp_dir.to_string());
    }
}
