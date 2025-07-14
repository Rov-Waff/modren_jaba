use regex::Regex;

#[derive(Debug)]
pub enum COMMANDS {
    PRINT,
    VERSION,
    TIME,
    EJZ,
    EXIT,
    HELP,
    INFO,
    ONUI,
    RUN,
    IDE,
    IdeOpensFile,
    Egg1,
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
            COMMANDS::INFO => Regex::new(r"^info\?").expect("Err!"),
            COMMANDS::ONUI => Regex::new(r"^OnUI$").expect("Err!"),
            COMMANDS::RUN => Regex::new(r"^run\?").expect("Err!"),
            COMMANDS::IDE => Regex::new(r"^IDE$").expect("Err!"),
            COMMANDS::IdeOpensFile => Regex::new(r"^IDE\?").expect("Err!"),
            COMMANDS::Egg1 => Regex::new(r"^114514$").expect("ERR"),
        }
    }
    pub fn new(command: &String) -> Option<COMMANDS> {
        if COMMANDS::get_regex(COMMANDS::PRINT).is_match(command) {
            Some(COMMANDS::PRINT)
        } else if COMMANDS::get_regex(COMMANDS::VERSION).is_match(command) {
            Some(COMMANDS::VERSION)
        } else if COMMANDS::get_regex(COMMANDS::TIME).is_match(command) {
            Some(COMMANDS::TIME)
        } else if COMMANDS::get_regex(COMMANDS::EXIT).is_match(command) {
            Some(COMMANDS::EXIT)
        } else if COMMANDS::get_regex(COMMANDS::EJZ).is_match(command) {
            Some(COMMANDS::EJZ)
        } else if COMMANDS::get_regex(COMMANDS::HELP).is_match(&command) {
            Some(COMMANDS::HELP)
        } else if COMMANDS::get_regex(COMMANDS::INFO).is_match(&command) {
            Some(COMMANDS::INFO)
        } else if COMMANDS::get_regex(COMMANDS::ONUI).is_match(&command) {
            Some(COMMANDS::ONUI)
        } else if COMMANDS::get_regex(COMMANDS::RUN).is_match(command) {
            Some(COMMANDS::RUN)
        } else if COMMANDS::get_regex(COMMANDS::IDE).is_match(command) {
            Some(COMMANDS::IDE)
        } else if COMMANDS::get_regex(COMMANDS::IdeOpensFile).is_match(command) {
            Some(COMMANDS::IdeOpensFile)
        } else if COMMANDS::get_regex(COMMANDS::Egg1).is_match(command) {
            Some(COMMANDS::Egg1)
        } else {
            None
        }
    }
}
