use colored::Colorize;
use std::path::Path;

use crate::config::{get_config_colors, set_truecolor, Config};

pub enum FileType {
    Config,
    Rust,
    C,
    CPP,
    CS,
    Zig,
    Python,
    JavaScript,
    TypeScript,
    Html,
    Css,
    Scss,
    Less,
    React,
    Git,
    Lock,
    Toml,
    License,
    Markdown,
    Golang,
    Svg,
    Photo,
    Audio,
    Video,
    Blender,
    Lua,
    Vim,
    Gleam,
    Php,
    Json,
    Yaml,
    Kotlin,
    Java,
    Assembly,
    Twig,
    None,
}

pub fn get_file_type(file: String) -> FileType {
    let path = Path::new(&file);

    // for specific files/dotfiles
    if let Some(dotfile) = path.file_name().and_then(|file_name| file_name.to_str()) {
        match dotfile {
            ".gitignore" | ".gitmodules" | ".gitattributes" => return FileType::Git,
            "LICENSE" => return FileType::License,
            "go.mod" | "go.sum" => return FileType::Golang,
            ".vimrc" => return FileType::Vim,
            _ => {}
        }
    }

    // for files with normal extensions
    match path.extension() {
        Some(ext) => match ext.to_string_lossy().as_ref() {
            "rs" => FileType::Rust,
            "c" => FileType::C,
            "cpp" | "cc" | "cxx" => FileType::CPP,
            "cs" => FileType::CS,
            "zig" => FileType::Zig,
            "py" => FileType::Python,
            "js" => FileType::JavaScript,
            "ts" => FileType::TypeScript,
            "htm" | "html" => FileType::Html,
            "css" => FileType::Css,
            "scss" | "sass" => FileType::Scss,
            "less" => FileType::Less,
            "jsx" | "tsx" => FileType::React,
            "lock" => FileType::Lock,
            "toml" => FileType::Toml,
            "md" => FileType::Markdown,
            "go" => FileType::Golang,
            "svg" => FileType::Svg,
            "png" | "jpg" | "jpeg" => FileType::Photo,
            "mp3" | "wma" | "wav" | "voc" | "tta" | "opus" | "mogg" | "oga" | "nmf" | "movpkg"
            | "mmf" | "m4b" | "m4a" | "iklax" | "flac" | "au" | "aiff" | "aax" | "aac" => {
                FileType::Audio
            }
            "webm" | "mkv" | "flv" | "vob" | "ogv" | "ogg" | "rrc" | "gifv" | "mng" | "mov"
            | "avi" | "qt" | "wmv" | "yuv" | "rm" | "asf" | "amv" | "mp4" | "m4p" | "mpg"
            | "mp2" | "mpeg" | "mpe" | "mpv" | "m4v" | "svi" | "3gp" | "3g2" | "mxf" | "roq"
            | "nsv" | "f4v" | "f4p" | "f4a" | "f4b" => FileType::Video,
            "blend" => FileType::Blender,
            "lua" => FileType::Lua,
            "vim" => FileType::Vim,
            "gleam" => FileType::Gleam,
            "php" => FileType::Php,
            "json" => FileType::Json,
            "yml" | "yaml" => FileType::Yaml,
            "kot" | "kt" | "kts" => FileType::Kotlin,
            "jar" | "war" | "ear" | "aar" | "java" | "class" | "properties" => FileType::Java,
            "asm" | "s" | "S" => FileType::Assembly,
            "twig" => FileType::Twig,
            _ => FileType::Config,
        },
        None => FileType::None,
    }
}

pub fn get_file_type_icon(file_type: FileType, config: &Config) -> String {
    let fti: &str = match file_type {
        FileType::Rust => "",
        FileType::Config => "",
        FileType::C => "",
        FileType::CPP => "",
        FileType::CS => "󰌛",
        FileType::Zig => "",
        FileType::Python => "",
        FileType::JavaScript => "",
        FileType::TypeScript => "",
        FileType::Html => "",
        FileType::Css => "",
        FileType::Scss => "",
        FileType::Less => "",
        FileType::React => "",
        FileType::Git => "",
        FileType::Lock => "",
        FileType::Toml => "",
        FileType::License => "",
        FileType::Markdown => "",
        FileType::Golang => "󰟓",
        FileType::Svg => "󰜡",
        FileType::Photo => "",
        FileType::Audio => "󰝚",
        FileType::Video => "",
        FileType::Blender => "󰂫",
        FileType::Lua => "󰢱",
        FileType::Vim => "",
        FileType::Gleam => "",
        FileType::Php => "",
        FileType::Json => "",
        FileType::Yaml => "",
        FileType::Kotlin => "",
        FileType::Java => "",
        FileType::Assembly => "",
        FileType::Twig => "",
        FileType::None => "".into(),
    };
    if !config.format.icons {
        return "".to_string();
    } else {
        let fti_colored = if config.format.colors {
            let config_colors = get_config_colors(&config);

            match file_type {
                FileType::Rust => set_truecolor(fti, config_colors.get("rust").unwrap()),
                FileType::Config => set_truecolor(fti, config_colors.get("config").unwrap()),
                FileType::C => set_truecolor(fti, config_colors.get("c").unwrap()),
                FileType::CPP => set_truecolor(fti, config_colors.get("cpp").unwrap()),
                FileType::CS => set_truecolor(fti, config_colors.get("cs").unwrap()),
                FileType::Zig => set_truecolor(fti, config_colors.get("zig").unwrap()),
                FileType::Python => set_truecolor(fti, config_colors.get("python").unwrap()),
                FileType::JavaScript => {
                    set_truecolor(fti, config_colors.get("javascript").unwrap())
                }
                FileType::TypeScript => {
                    set_truecolor(fti, config_colors.get("typescript").unwrap())
                }
                FileType::Html => set_truecolor(fti, config_colors.get("html").unwrap()),
                FileType::Css => set_truecolor(fti, config_colors.get("css").unwrap()),
                FileType::Scss => set_truecolor(fti, config_colors.get("scss").unwrap()),
                FileType::Less => set_truecolor(fti, config_colors.get("less").unwrap()),
                FileType::React => set_truecolor(fti, config_colors.get("react").unwrap()),
                FileType::Git => set_truecolor(fti, config_colors.get("git").unwrap()),
                FileType::Lock => set_truecolor(fti, config_colors.get("lock").unwrap()),
                FileType::Toml => set_truecolor(fti, config_colors.get("toml").unwrap()),
                FileType::License => set_truecolor(fti, config_colors.get("license").unwrap()),
                FileType::Markdown => set_truecolor(fti, config_colors.get("markdown").unwrap()),
                FileType::Golang => set_truecolor(fti, config_colors.get("golang").unwrap()),
                FileType::Svg => set_truecolor(fti, config_colors.get("svg").unwrap()),
                FileType::Photo => set_truecolor(fti, config_colors.get("photo").unwrap()),
                FileType::Audio => set_truecolor(fti, config_colors.get("audio").unwrap()),
                FileType::Video => set_truecolor(fti, config_colors.get("video").unwrap()),
                FileType::Blender => set_truecolor(fti, config_colors.get("blender").unwrap()),
                FileType::Lua => set_truecolor(fti, config_colors.get("lua").unwrap()),
                FileType::Vim => set_truecolor(fti, config_colors.get("vim").unwrap()),
                FileType::Gleam => set_truecolor(fti, config_colors.get("gleam").unwrap()),
                FileType::Php => set_truecolor(fti, config_colors.get("php").unwrap()),
                FileType::Json => set_truecolor(fti, config_colors.get("json").unwrap()),
                FileType::Yaml => set_truecolor(fti, config_colors.get("yaml").unwrap()),
                FileType::Kotlin => set_truecolor(fti, config_colors.get("kotlin").unwrap()),
                FileType::Java => set_truecolor(fti, config_colors.get("java").unwrap()),
                FileType::Assembly => set_truecolor(fti, config_colors.get("assembly").unwrap()),
                FileType::Twig => set_truecolor(fti, config_colors.get("twig").unwrap()),
                _ => fti.white(),
            }
        } else {
            fti.white()
        };

        fti_colored.to_string()
    }
}

pub fn render_file(file_name: String, file_type: FileType, config: &Config) -> String {
    format!("{} {}", get_file_type_icon(file_type, config), file_name)
}
