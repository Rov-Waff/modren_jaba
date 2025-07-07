use regex::Regex;

#[derive(Debug)]
pub enum COMMANDS {
    PRINT,
    VERSION,
    TIME,
    EJZ,
    EXIT,
    HELP,
}

impl COMMANDS {
    fn get_regex(command: COMMANDS) -> Regex {
        match command {
            COMMANDS::PRINT => Regex::new(r"^print\?").expect("Err!"),
            COMMANDS::VERSION => Regex::new(r"^version$").expect("Err!"),
            COMMANDS::TIME => Regex::new(r"^time$").expect("Err"),
            COMMANDS::EXIT => Regex::new(r"^exit$").expect("Err!"),
            COMMANDS::EJZ => Regex::new(r"^ejz\?").expect("Err!"),
            COMMANDS::HELP => Regex::new(r"^help$").expect("Err!"),
        }
    }
    pub fn new(command: &String) -> Option<COMMANDS> {
        if COMMANDS::get_regex(COMMANDS::PRINT).is_match(command) {
            return Some(COMMANDS::PRINT);
        } else if COMMANDS::get_regex(COMMANDS::VERSION).is_match(command) {
            return Some(COMMANDS::VERSION);
        } else if COMMANDS::get_regex(COMMANDS::TIME).is_match(command) {
            return Some(COMMANDS::TIME);
        } else if COMMANDS::get_regex(COMMANDS::EXIT).is_match(command) {
            return Some(COMMANDS::EXIT);
        } else if COMMANDS::get_regex(COMMANDS::EJZ).is_match(command) {
            return Some(COMMANDS::EJZ);
        } else if COMMANDS::get_regex(COMMANDS::HELP).is_match(&command) {
            return Some(COMMANDS::HELP);
        } else {
            return None;
        }
    }
}
