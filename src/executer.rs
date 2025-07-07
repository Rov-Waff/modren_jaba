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
            COMMANDS::PRINT => {
                let mut _status = false;
                for i in self.command.chars() {
                    if i == '?' && _status == false {
                        _status = true;
                        continue;
                    }
                    if _status {
                        print!("{}", i);
                    }
                }
                print!("\n");
            }
            COMMANDS::VERSION => {
                println!("当前版本:{}", VERSION)
            }
            COMMANDS::TIME => {
                println!("当前时间:{}", Local::now().format("%Y-%m-%d %H:%M:%S"))
            }
            COMMANDS::EXIT => {
                println!("jaba已退出");
                std::process::exit(0);
            }
            COMMANDS::EJZ => {
                println!("已将{}转换为二进制",self.command[4..].to_string());
                print!("结果:");
                let mut res = String::new();
                for i in self.command[4..].chars() {
                    res.push_str(&format!("{:032b}", i as u32));
                    res.push_str(" ")
                }
                println!("{}", res);
                println!("--------------------------------")
            }
        }
    }
    pub fn new(_command_type: COMMANDS, _command: &String) -> Executer {
        Executer {
            command_type: _command_type,
            command: _command.to_string(),
        }
    }
}
