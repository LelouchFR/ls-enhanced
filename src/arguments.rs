#[derive(Debug)]
pub struct Argument {
    name: String,
    description: Option<String>,
    short: Option<String>,
    long: Option<String>,
    default_value: Option<String>,
}

impl Argument {
    pub fn new(name: impl Into<String>) -> Argument {
        Argument {
            name: name.into(),
            description: None,
            short: None,
            long: None,
            default_value: None,
        }
    }

    pub fn set_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub fn set_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn set_short(mut self, short: impl Into<String>) -> Self {
        self.short = Some(short.into());
        self
    }

    pub fn set_long(mut self, long: impl Into<String>) -> Self {
        self.long = Some(long.into());
        self
    }

    pub fn set_default_value(mut self, default_value: impl Into<String>) -> Self {
        self.default_value = Some(default_value.into());
        self
    }
}

pub fn create_arg() -> Vec<Argument> {
    // -a or --a
    let all = Argument::new("all")
        .set_short("a")
        .set_long("all")
        .set_description("Show all files, including hidden ones");

    // -l or --list
    let list = Argument::new("list")
        .set_short("l")
        .set_long("list")
        .set_description("List the files with their read and write properties");

    // -h or --help
    let help = Argument::new("help")
        .set_short("h")
        .set_long("help")
        .set_description("Lists all possible arguments");

    // -i=bool or --icon=bool
    let icon = Argument::new("icon")
        .set_short("i")
        .set_long("icon")
        .set_description("show or not the icons");

    // -R or --recursive
    let recursive = Argument::new("recursive")
        .set_short("R")
        .set_long("recursive")
        .set_description("See all the files as a tree");

    let possible_args: Vec<Argument> = vec![all, list, help, icon, recursive];

    possible_args
}
