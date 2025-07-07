#[derive(Debug)]
pub enum COMMANDS {
    PRINT,
    VERSION,
    TIME,
    EXIT
}

impl COMMANDS{
    pub fn exec(&self){
        match self {
            COMMANDS::PRINT => todo!(),
            COMMANDS::VERSION => todo!(),
            COMMANDS::TIME => todo!(),
            COMMANDS::EXIT => todo!(),
        }
    }
}