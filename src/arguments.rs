#[derive(Debug, PartialEq)]
pub enum Arguments {
    All,
    List,
    Recursive,
    Icon,
    Help,
}

#[derive(Debug, PartialEq)]
pub struct Argument {
    pub name: String,
    pub description: Option<String>,
    pub short: Option<String>,
    pub long: Option<String>,
    pub default_value: Option<String>,
    pub arg_type: Option<Arguments>,
}

impl Argument {
    pub fn new(name: impl Into<String>) -> Argument {
        Argument {
            name: name.into(),
            description: None,
            short: None,
            long: None,
            default_value: None,
            arg_type: None,
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
        self.short = Some(format!("-{}", short.into()));
        self
    }

    pub fn set_long(mut self, long: impl Into<String>) -> Self {
        self.long = Some(format!("--{}", long.into()));
        self
    }

    pub fn set_default_value(mut self, default_value: impl Into<String>) -> Self {
        self.default_value = Some(default_value.into());
        self
    }

    pub fn set_arg_type(mut self, arg_type: impl Into<Arguments>) -> Self {
        self.arg_type = Some(arg_type.into());
        self
    }

    pub fn get_arg_type(self: &Self) -> &Arguments {
        if let Some(arg) = &self.arg_type {
            arg
        } else {
            &Arguments::Help
        }
    }
}

pub fn create_arg() -> Vec<Argument> {
    // -a or --a
    let all = Argument::new("all")
        .set_short("a")
        .set_long("all")
        .set_description("Show all files, including hidden ones")
        .set_arg_type(Arguments::All);

    // -l or --list
    let list = Argument::new("list")
        .set_short("l")
        .set_long("list")
        .set_description("List the files with their read and write properties")
        .set_arg_type(Arguments::List);

    // -h or --help
    let help = Argument::new("help")
        .set_short("h")
        .set_long("help")
        .set_description("Lists all possible arguments")
        .set_arg_type(Arguments::Help);

    // -i=bool or --icon=bool
    let icon = Argument::new("icon")
        .set_short("i")
        .set_long("icon")
        .set_description("show or not the icons")
        .set_arg_type(Arguments::Icon);

    // -r or --recursive
    let recursive = Argument::new("recursive")
        .set_short("r")
        .set_long("recursive")
        .set_description("See all the files as a tree")
        .set_arg_type(Arguments::Recursive);

    let possible_args: Vec<Argument> = vec![all, list, help, icon, recursive];

    possible_args
}
