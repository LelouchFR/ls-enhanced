use crate::{
    arguments::{generate_help_text, Arguments},
    config::Config,
    formatting::permissions::{format_permissions, read_permission},
    types::files,
};
use colored::Colorize;
use std::cmp;
use std::fs;
use term_size;

pub fn format_ls(mut config: Config, path: String, args: Vec<&Arguments>) -> std::io::Result<()> {
    let mut show_permissions = false;

    for arg in args {
        let _ = match arg {
            &Arguments::All => {
                config.format.dotfiles = true;
            }
            &Arguments::List => {
                config.format.inline = false;
                show_permissions = true;
            }
            &Arguments::Icon => {
                config.format.icons = true;
            }
            &Arguments::Help => {
                let _ = generate_help_text();
            }
            &Arguments::Recursive => {
                todo!();
            }
        };
    }

    if config.format.inline {
        inline_format(config, path)
    } else {
        multi_line_format(config, path, show_permissions)
    }
}

pub fn inline_format(config: Config, path: String) -> std::io::Result<()> {
    let mut directories: Vec<String> = Vec::new();
    let mut files: Vec<String> = Vec::new();
    let mut symlinks: Vec<String> = Vec::new();
    let mut result: Vec<String> = Vec::new();

    let mut max_str_size: usize = 0;

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy().to_string();

        if file_name_str.len() > max_str_size {
            max_str_size = file_name_str.len();
        }

        if metadata.is_dir() {
            if !config.format.dotfiles && file_name_str.starts_with(".") {
                continue;
            } else {
                if file_name_str == ".github" {
                    if config.format.icons {
                        directories.push(format!("{} {}", "󰊤".blue(), file_name_str.blue().bold()))
                    } else {
                        directories.push(format!("{}", file_name_str.blue().bold()))
                    }
                } else {
                    if config.format.icons {
                        directories.push(format!("{} {}", "".blue(), file_name_str.blue().bold()))
                    } else {
                        directories.push(format!("{}", file_name_str.blue().bold()))
                    }
                }
            }
        } else if metadata.is_file() {
            if !config.format.dotfiles && file_name_str.starts_with(".") {
                continue;
            } else {
                files.push(format!(
                    "{}",
                    files::render_file(
                        file_name_str.to_string(),
                        files::get_file_type(file_name_str.to_string()),
                        &config
                    )
                ));
            }
        } else if metadata.is_symlink() {
            if !config.format.dotfiles && file_name_str.starts_with(".") {
                continue;
            } else {
                symlinks.push(format!("{}", file_name_str.green().bold()));
            }
        }
    }

    for directory in &directories {
        result.push(directory.to_string());
    }

    for file in &files {
        result.push(file.to_string());
    }

    for symlink in &symlinks {
        result.push(symlink.to_string());
    }

    let (term_width, _) = term_size::dimensions().unwrap_or((80, 25));
    let column_width = max_str_size + 2;
    let num_columns = cmp::max(1, term_width / column_width);

    for (i, directory) in result.iter().enumerate() {
        print!("{:<width$}  ", directory, width = column_width);
        if (i + 1) % num_columns == 0 {
            println!();
        }
    }

    if directories.len() % num_columns != 0 {
        println!();
    }

    Ok(())
}

pub fn multi_line_format(
    config: Config,
    path: String,
    show_permissions: bool,
) -> std::io::Result<()> {
    let mut result: Vec<String> = Vec::new();
    for entry in fs::read_dir(path.clone())? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();
        let permissions = {
            if show_permissions {
                format!(
                    "{} ",
                    format_permissions(&file_name_str, read_permission(&file_name_str))
                )
            } else {
                "".to_string()
            }
        };

        if metadata.is_dir() {
            if !config.format.dotfiles && file_name_str.starts_with(".") {
                continue;
            } else {
                if file_name_str == ".github" {
                    if config.format.icons {
                        result.push(format!(
                            "{}{} {}",
                            permissions,
                            "󰊤".blue(),
                            file_name_str.blue().bold()
                        ))
                    } else {
                        result.push(format!("{}{}", permissions, file_name_str.blue().bold()))
                    }
                } else {
                    if config.format.icons {
                        result.push(format!(
                            "{}{} {}",
                            permissions,
                            "".blue(),
                            file_name_str.blue().bold()
                        ))
                    } else {
                        result.push(format!("{}{}", permissions, file_name_str.blue().bold()))
                    }
                }
            }
        } else if metadata.is_file() {
            if !config.format.dotfiles && file_name_str.starts_with(".") {
                continue;
            } else {
                result.push(format!(
                    "{}{}",
                    permissions,
                    files::render_file(
                        file_name_str.to_string(),
                        files::get_file_type(file_name_str.to_string()),
                        &config
                    )
                ));
            }
        } else if metadata.is_symlink() {
            if !config.format.dotfiles && file_name_str.starts_with(".") {
                continue;
            } else {
                result.push(format!("{}{}", permissions, file_name_str.green().bold()));
            }
        }
    }

    for entry in &result {
        println!("{}", entry);
    }

    Ok(())
}
