use crate::command::COMMANDS;

pub struct Executer{
    command_type:COMMANDS,
    command:String
}

impl Executer{
    pub fn exec(&self){
        match self.command_type{
            COMMANDS::PRINT => todo!(),
            COMMANDS::VERSION => todo!(),
            COMMANDS::TIME => todo!(),
            COMMANDS::EXIT => todo!(),
        }
    }
}