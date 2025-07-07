use regex::Regex;

#[derive(Debug)]
pub enum COMMANDS {
    PRINT,
    VERSION,
    TIME,
    EXIT,
}

impl COMMANDS {
    fn get_regex(command: COMMANDS) -> Regex {
        match command {
            COMMANDS::PRINT => Regex::new(r"^print\?").expect("Err!"),
            COMMANDS::VERSION => Regex::new(r"^version$").expect("Err!"),
            COMMANDS::TIME => Regex::new(r"^time$").expect("Err"),
            COMMANDS::EXIT => Regex::new(r"^exit$").expect("Err!"),
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
        } else {
            return None;
        }
    }
}
