use crate::command::COMMANDS;
use crate::ide_os::OS;
use chrono::Local;
use std::fs::File;
use std::io::{BufRead, BufReader};

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
            COMMANDS::RUN => {
                let file;
                match File::open(self.command[4..].to_string()) {
                    Ok(_file) => file = _file,
                    Err(_) => {
                        println!("该URL无法访问");
                        return;
                    }
                }
                let reader = BufReader::new(file);
                for _command_line in reader.lines() {
                    match _command_line {
                        Ok(_line) => {
                            let command_type = match COMMANDS::new(&_line.to_string()) {
                                Some(_l) => _l,
                                None => {
                                    println!("指令:{} 有误！", _line);
                                    break;
                                }
                            };
                            Executer::new(command_type, &_line).exec()
                        }
                        Err(_) => {}
                    }
                }
            }
            COMMANDS::IDE => match OS::get_os() {
                None => {
                    println!("还没有适配这个系统！")
                }
                Some(os) => match os {
                    OS::WINDOWS => {
                        println!("正在为你打开Windows第一IDE！");
                        match std::process::Command::new("notepad").output() {
                            Ok(_) => {
                                println!("没错！这就是Windows 第一IDE！")
                            }
                            Err(_) => {
                                println!("无法打开IDE！")
                            }
                        }
                    }
                    OS::LINUX => {
                        println!("正在为你打开Linux第一IDE！");
                        match std::process::Command::new("gedit").output() {
                            Ok(_) => {
                                println!("没错！这就是Linux第一IDE！")
                            }
                            Err(_) => {
                                println!(
                                    "无法打开IDE 提示:sudo apt install gedit或sudo dnf install gedit或sudo pacman -S gedit"
                                )
                            }
                        }
                    }
                },
            },
            COMMANDS::IdeOpensFile => match OS::get_os() {
                None => {}
                Some(os) => match os {
                    OS::WINDOWS => {
                        println!("正在为你打开Windows第一IDE！");
                        match std::process::Command::new("notepad")
                            .arg(self.command[4..].to_string())
                            .output()
                        {
                            Ok(_) => {
                                println!("没错！这就是Windows 第一IDE！")
                            }
                            Err(_) => {
                                println!("无法打开IDE！")
                            }
                        }
                    }
                    OS::LINUX => {
                        println!("正在为你打开Linux第一IDE！");
                        match std::process::Command::new("gedit")
                            .arg(self.command[4..].to_string())
                            .output()
                        {
                            Ok(_) => {
                                println!("没错！这就是Linux第一IDE！")
                            }
                            Err(_) => {
                                println!(
                                    "无法打开IDE 提示:sudo apt install gedit或sudo dnf install gedit或sudo pacman -S gedit"
                                )
                            }
                        }
                    }
                },
            },
            COMMANDS::Egg1 => {
                print!(
                    "
       　  　▃▆█▇▄▖
　 　 　 ▟◤▖　　　◥█▎
   　 ◢◤　 ▐　　　 　▐▉
　 ▗◤　　　▂　▗▖　　▕ █▎
　◤　▗▅▖◥▄　  ▀◣　 █▊
▐　▕▎◥▖◣◤　　　　◢██
█◣　◥▅█▀　　　　▐██◤
▐█▙▂　　     　◢██◤
◥██◣　　　◢▄◤
 　　▀██▅▇▀
"
                )
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
