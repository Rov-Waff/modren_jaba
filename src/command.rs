use regex::Regex;

#[derive(Debug)]
pub enum COMMANDS {
    PRINT,
    VERSION,
    TIME,
    EXIT,
}

impl COMMANDS {
    pub fn exec(&self) {
        match self {
            COMMANDS::PRINT => todo!(),
            COMMANDS::VERSION => todo!(),
            COMMANDS::TIME => todo!(),
            COMMANDS::EXIT => todo!(),
        }
    }

    fn get_regex(command: COMMANDS) -> Regex {
        match command {
            COMMANDS::PRINT => Regex::new(r"[print\?]").expect("Err!"),
            COMMANDS::VERSION => Regex::new(r"^version$").expect("Err!"),
            COMMANDS::TIME => Regex::new(r"^time$").expect("Err"),
            COMMANDS::EXIT => Regex::new(r"^exit$").expect("Err!"),
        }
    }

    pub fn new(command: String) {
        todo!()
    }
}
