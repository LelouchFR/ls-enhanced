use std::fs;
use colored::Colorize;

mod types;

use crate::types::files;

fn main() -> std::io::Result<()> {
    let path = ".";
    
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();

        if metadata.is_dir() {
            if file_name_str == ".github" {
                println!("{} {}", "󰊤".blue(), file_name_str.blue().bold());
            } else {
                println!("{} {}", "".blue(), file_name_str.blue().bold());
            }
        } else if metadata.is_file() {
            println!("{}", files::render_file(file_name_str.to_string(), files::get_file_type(file_name_str.to_string())));
        } else if metadata.is_symlink() {
            println!("{}", file_name_str.green().bold());
        }
    }

    Ok(())
}
