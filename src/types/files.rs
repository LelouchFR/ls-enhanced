use std::path::Path;
use colored::Colorize;

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
    None,
}

pub fn get_file_type(file: String) -> FileType {
    let path = Path::new(&file);

    if let Some(dotfile) = path.file_name().and_then(|file_name| file_name.to_str()) {
        match dotfile {
            ".gitignore" | ".gitmodules" => return FileType::Git,
            "LICENSE" => return FileType::License,
            _ => {}
        }
    }

    match path.extension() {
        Some(ext) => {
            match ext.to_string_lossy().as_ref() {
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
                _  => FileType::Config
            }
        },
        None => FileType::None,
    }
}

pub fn get_file_type_icon(file_type: FileType) -> String {
    match file_type {
        FileType::Rust => format!("{}", "".truecolor(206, 66, 43)),
        FileType::Config => format!("{}", "".truecolor(128, 128, 128)),
        FileType::C => format!("{}", "".truecolor(57, 74, 171)),
        FileType::CPP => format!("{}", "".truecolor(0, 89, 156)),
        FileType::CS => format!("{}", "󰌛".truecolor(149, 60, 173)),
        FileType::Zig => format!("{}", "".truecolor(247, 164, 29)),
        FileType::Python => format!("{}", "".truecolor(255, 224, 82)),
        FileType::JavaScript => format!("{}", "".truecolor(240, 219, 79)),
        FileType::TypeScript => format!("{}", "".truecolor(0, 122, 204)),
        FileType::Html => format!("{}", "".truecolor(225, 78, 29)),
        FileType::Css => format!("{}", "".truecolor(2, 119, 189)),
        FileType::Scss => format!("{}", "".truecolor(205, 103, 153)),
        FileType::React => format!("{}", "".truecolor(0, 216, 255)),
        FileType::Git => format!("{}", "".truecolor(241, 80, 47)),
        FileType::Lock => format!("{}", "".truecolor(244, 244, 244)),
        FileType::Toml => format!("{}", "".truecolor(156, 66, 33)),
        FileType::License => format!("{}", "".truecolor(249, 252, 33)),
        _ => "".to_string()
    }
}

pub fn render_file(file_name: String, file_type: FileType) -> String {
    format!("{} {}", get_file_type_icon(file_type), file_name)
}
