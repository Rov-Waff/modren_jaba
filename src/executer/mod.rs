use crate::command::COMMANDS;
use chrono::Local;
use gtk4::prelude::{ApplicationExt, ApplicationExtManual, ButtonExt, GridExt, GtkWindowExt};
use gtk4::{Application, ApplicationWindow, Grid, Image};

mod info_window;
mod main_window;

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
                println!("已将{}转换为二进制", self.command[4..].to_string());
                print!("结果:");
                let mut res = String::new();
                for i in self.command[4..].chars() {
                    res.push_str(&format!("{:032b}", i as u32));
                    res.push_str(" ")
                }
                println!("{}", res);
                println!("--------------------------------")
            }
            COMMANDS::HELP => {
                match webbrowser::open("https://rov-waff.github.io/modren_jaba_docs/") {
                    Ok(_) => println!("已在浏览器打开"),
                    Err(_) => println!("无法打开！"),
                }
            }
            COMMANDS::INFO => {
                info_window::show_info_window(&self.command);
            }
            COMMANDS::ONUI => {
                main_window::show_main_window();
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
