use regex::Regex;

// #[derive(Default)]
pub(crate) struct Config {
    pub spliter: Option<String>,
    pub trim_each_line: bool,
    pub skip_empty_line: bool,
    pub not_math_put_all: bool,
    pub verbose: bool,
    pub left: char,
    pub right: char,
    pub regexp: Option<Regex>,
    pub commands: Vec<String>,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            spliter: None,
            verbose: false,
            trim_each_line: true,
            skip_empty_line: true,
            not_math_put_all: false,
            left: '{',
            right: '}',
            regexp: None,
            commands: Vec::new(),
        }
    }
}
