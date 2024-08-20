use colored::Colorize;
use std::path::Path;

use crate::config::{get_colors, set_truecolor, Config};

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
            _ => FileType::Config,
        },
        None => FileType::None,
    }
}

pub fn get_file_type_icon(file_type: FileType, config: &Config) -> String {
    let fti = match file_type {
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
        _ => "".into(),
    };
    if !config.format.icons {
        return "".to_string();
    } else {
        let fti_colored = if config.format.colors {
            let rust_colors = get_colors(&config, "rust");
            let config_colors = get_colors(&config, "config");
            let c_colors = get_colors(&config, "c");
            let cpp_colors = get_colors(&config, "cpp");
            let cs_colors = get_colors(&config, "cs");
            let zig_colors = get_colors(&config, "zig");
            let python_colors = get_colors(&config, "python");
            let js_colors = get_colors(&config, "javascript");
            let ts_colors = get_colors(&config, "typescript");
            let html_colors = get_colors(&config, "html");
            let css_colors = get_colors(&config, "css");
            let scss_colors = get_colors(&config, "scss");
            let react_colors = get_colors(&config, "react");
            let git_colors = get_colors(&config, "git");
            let lock_colors = get_colors(&config, "lock");
            let toml_colors = get_colors(&config, "toml");
            let license_colors = get_colors(&config, "license");
            let md_colors = get_colors(&config, "markdown");
            let go_colors = get_colors(&config, "golang");
            let svg_colors = get_colors(&config, "svg");
            let photo_colors = get_colors(&config, "photo");
            let audio_colors = get_colors(&config, "audio");
            let video_colors = get_colors(&config, "video");
            let blender_colors = get_colors(&config, "blender");
            let lua_colors = get_colors(&config, "lua");
            let vim_colors = get_colors(&config, "vim");

            match file_type {
                FileType::Rust => set_truecolor(fti, &rust_colors),
                FileType::Config => set_truecolor(fti, &config_colors),
                FileType::C => set_truecolor(fti, &c_colors),
                FileType::CPP => set_truecolor(fti, &cpp_colors),
                FileType::CS => set_truecolor(fti, &cs_colors),
                FileType::Zig => set_truecolor(fti, &zig_colors),
                FileType::Python => set_truecolor(fti, &python_colors),
                FileType::JavaScript => set_truecolor(fti, &js_colors),
                FileType::TypeScript => set_truecolor(fti, &ts_colors),
                FileType::Html => set_truecolor(fti, &html_colors),
                FileType::Css => set_truecolor(fti, &css_colors),
                FileType::Scss => set_truecolor(fti, &scss_colors),
                FileType::React => set_truecolor(fti, &react_colors),
                FileType::Git => set_truecolor(fti, &git_colors),
                FileType::Lock => set_truecolor(fti, &lock_colors),
                FileType::Toml => set_truecolor(fti, &toml_colors),
                FileType::License => set_truecolor(fti, &license_colors),
                FileType::Markdown => set_truecolor(fti, &md_colors),
                FileType::Golang => set_truecolor(fti, &go_colors),
                FileType::Svg => set_truecolor(fti, &svg_colors),
                FileType::Photo => set_truecolor(fti, &photo_colors),
                FileType::Audio => set_truecolor(fti, &audio_colors),
                FileType::Video => set_truecolor(fti, &video_colors),
                FileType::Blender => set_truecolor(fti, &blender_colors),
                FileType::Lua => set_truecolor(fti, &lua_colors),
                FileType::Vim => set_truecolor(fti, &vim_colors),
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
