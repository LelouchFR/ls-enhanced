use colored::Colorize;
use std::path::Path;

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
            _ => FileType::Config,
        },
        None => FileType::None,
    }
}

pub fn get_file_type_icon(file_type: FileType) -> String {
    let file_type_icon = match file_type {
        FileType::Rust => "".truecolor(206, 66, 43),
        FileType::Config => "".truecolor(128, 128, 128),
        FileType::C => "".truecolor(57, 74, 171),
        FileType::CPP => "".truecolor(0, 89, 156),
        FileType::CS => "󰌛".truecolor(149, 60, 173),
        FileType::Zig => "".truecolor(247, 164, 29),
        FileType::Python => "".truecolor(255, 224, 82),
        FileType::JavaScript => "".truecolor(240, 219, 79),
        FileType::TypeScript => "".truecolor(0, 122, 204),
        FileType::Html => "".truecolor(225, 78, 29),
        FileType::Css => "".truecolor(2, 119, 189),
        FileType::Scss => "".truecolor(205, 103, 153),
        FileType::React => "".truecolor(0, 216, 255),
        FileType::Git => "".truecolor(241, 80, 47),
        FileType::Lock => "".truecolor(244, 244, 244),
        FileType::Toml => "".truecolor(156, 66, 33),
        FileType::License => "".truecolor(249, 252, 33),
        FileType::Markdown => "".truecolor(244, 244, 244),
        FileType::Golang => "󰟓".truecolor(0, 180, 224),
        _ => "".into(),
    };

    file_type_icon.to_string()
}

pub fn render_file(file_name: String, file_type: FileType) -> String {
    format!("{} {}", get_file_type_icon(file_type), file_name)
}
