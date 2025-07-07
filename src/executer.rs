use chrono::Local;

use crate::command::COMMANDS;

const VERSION: &str = env!("CARGO_PKG_VERSION");
pub struct Executer {
    command_type: COMMANDS,
    command: String,
}

impl Executer {
    pub fn exec(&self) {
        match self.command_type {
            COMMANDS::PRINT => todo!(),
            COMMANDS::VERSION => {
                println!("当前版本:{}", VERSION)
            }
            COMMANDS::TIME => {
                println!("当前时间:{}",Local::now().format("%Y-%m-%d %H:%M:%S"))
            },
            COMMANDS::EXIT => {
                std::process::exit(0);
            },
        }
    }
}
