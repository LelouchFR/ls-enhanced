use colored::Colorize;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

#[derive(Debug, Serialize, Deserialize)]
pub struct Format {
    pub icons: bool,
    pub inline: bool,
    pub dotfiles: bool,
    pub colors: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Plugin {
    pub icons: HashMap<String, String>,
    pub colors: HashMap<String, Color>,
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
    pub plugins: HashMap<String, Plugin>,
    pub colors: HashMap<String, Color>,
}

impl Format {
    pub fn new(icons: bool, inline: bool, dotfiles: bool, colors: bool) -> Format {
        Format {
            icons,
            inline,
            dotfiles,
            colors,
        }
    }
}

impl Plugin {
    pub fn new(icons: HashMap<String, String>, colors: HashMap<String, Color>) -> Plugin {
        Plugin { icons, colors }
    }
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Color {
        Color { red, green, blue }
    }
}

pub fn get_colors(config: &Config, config_color: &str) -> Color {
    Color {
        red: config
            .colors
            .get(config_color)
            .unwrap_or(create_config().colors.get(config_color).unwrap())
            .red,
        green: config
            .colors
            .get(config_color)
            .unwrap_or(create_config().colors.get(config_color).unwrap())
            .green,
        blue: config
            .colors
            .get(config_color)
            .unwrap_or(create_config().colors.get(config_color).unwrap())
            .blue,
    }
}

pub fn set_truecolor(file_type_icon: &str, color: &Color) -> colored::ColoredString {
    file_type_icon.truecolor(color.red, color.green, color.blue)
}

pub fn create_config() -> Config {
    let mut plugins = HashMap::new();

    let git_icons = HashMap::from([
        ("untracked".to_string(), "✗".to_string()),
        ("tracked".to_string(), "✓".to_string()),
    ]);

    let git_colors = HashMap::from([
        ("untracked".to_string(), Color::new(255, 255, 255)),
        ("tracked".to_string(), Color::new(255, 255, 255)),
    ]);

    plugins.insert("git".to_string(), Plugin::new(git_icons, git_colors));

    let colors = HashMap::from([
        ("rust".to_string(), Color::new(206, 66, 43)),
        ("config".to_string(), Color::new(128, 128, 128)),
        ("c".to_string(), Color::new(57, 74, 171)),
        ("cpp".to_string(), Color::new(0, 89, 156)),
        ("cs".to_string(), Color::new(149, 60, 173)),
        ("zig".to_string(), Color::new(247, 164, 29)),
        ("python".to_string(), Color::new(255, 224, 82)),
        ("javascript".to_string(), Color::new(240, 219, 79)),
        ("typescript".to_string(), Color::new(0, 122, 204)),
        ("html".to_string(), Color::new(225, 78, 29)),
        ("css".to_string(), Color::new(2, 119, 189)),
        ("scss".to_string(), Color::new(205, 103, 153)),
        ("react".to_string(), Color::new(0, 216, 255)),
        ("git".to_string(), Color::new(241, 80, 47)),
        ("lock".to_string(), Color::new(244, 244, 244)),
        ("toml".to_string(), Color::new(156, 66, 33)),
        ("license".to_string(), Color::new(249, 252, 33)),
        ("markdown".to_string(), Color::new(244, 244, 244)),
        ("golang".to_string(), Color::new(0, 180, 224)),
        ("svg".to_string(), Color::new(255, 177, 59)),
        ("photo".to_string(), Color::new(163, 76, 245)),
        ("audio".to_string(), Color::new(163, 76, 245)),
        ("video".to_string(), Color::new(163, 76, 245)),
        ("blender".to_string(), Color::new(234, 118, 0)),
        ("lua".to_string(), Color::new(0, 0, 128)),
        ("vim".to_string(), Color::new(1, 152, 51)),
    ]);

    Config {
        format: Format {
            icons: true,
            inline: true,
            dotfiles: true,
            colors: true,
        },
        plugins,
        colors,
    }
}

pub fn generate_config(path: String) -> std::io::Result<()> {
    let config: &Config = &create_config();
    let toml_string = toml::to_string(config).expect("Failed to serialize config");

    let mut file = File::create(&path)?;
    let _ = file.write_all(toml_string.as_bytes())?;

    Ok(())
}
