use crate::config::Config;
use colored::{ColoredString, Colorize};
use std::fs;
use std::os::unix::fs::PermissionsExt;

pub fn read_permission(path: &str) -> u32 {
    let metadata = fs::metadata(path).expect("Unable to read metadata.");
    let permissions = metadata.permissions();
    permissions.mode()
}

pub fn grey(string: &str) -> ColoredString {
    string.truecolor(128, 128, 128)
}

pub fn format_permissions(config: &Config, path: &str, mode: u32) -> ColoredString {
    let metadata = fs::metadata(path).expect("Unable to read metadata.");
    let mut permissions: Vec<ColoredString> = Vec::new();
    let codes: Vec<u32> = vec![
        0o400, 0o200, 0o100, 0o040, 0o020, 0o010, 0o004, 0o002, 0o001,
    ];

    permissions.push(if metadata.is_dir() {
        "d".to_string().blue()
    } else {
        grey("-")
    });

    for code in &codes {
        permissions.push(if mode & code != 0 {
            match code {
                0o400 | 0o040 | 0o004 => "r".to_string().yellow(),
                0o200 | 0o020 | 0o002 => "w".to_string().red(),
                0o100 | 0o010 | 0o001 => "x".to_string().green(),
                _ => grey("-"),
            }
        } else {
            grey("-")
        })
    }

    let mut colored_permission: ColoredString = ColoredString::default();

    for permission in permissions {
        if !config.format.colors {
            colored_permission = format!("{}{}", colored_permission, permission.white()).into();
        } else {
            colored_permission = format!("{}{}", colored_permission, permission).into();
        }
    }

    colored_permission
}
