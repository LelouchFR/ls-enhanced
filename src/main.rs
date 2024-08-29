pub mod arguments;
pub mod config;
pub mod formatting;
pub mod types;

use crate::{arguments::Arguments, config::Config, formatting::format::format_ls};
use std::{env, fs, path::Path, process::exit};
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

    let possible_args = arguments::create_arg();
    let mut args: Vec<&Arguments> = Vec::new();
    let mut path: Option<String> = Some(".".to_string());

    for arg in env::args() {
        let possible_path = Path::new(&arg);
        if possible_path.exists() && possible_path.is_dir() {
            path = Some(possible_path.to_string_lossy().to_string());
        } else {
            for possible_arg in &possible_args {
                if let Some(ref short) = possible_arg.short {
                    if arg == *short && !args.contains(&possible_arg.get_arg_type()) {
                        args.push(possible_arg.get_arg_type());
                    }
                }

                if let Some(ref long) = possible_arg.long {
                    if arg == *long && !args.contains(&possible_arg.get_arg_type()) {
                        args.push(possible_arg.get_arg_type());
                    }
                }
            }
        }
    }

    format_ls(config, path.unwrap(), args)
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

        let _ = format_ls(config, temp_dir.to_string(), vec![]);
    }
}
