use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Format {
    pub icons: bool,
    pub folders_first: bool,
    pub inline: bool,
    pub dotfiles: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub format: Format,
    pub colors: HashMap<String, Color>,
}

pub fn generate_config() {
    todo!();
}
