use crate::command::COMMANDS;
use chrono::Local;
use gtk4::prelude::{ApplicationExt, ApplicationExtManual, ButtonExt, GridExt, GtkWindowExt};
use gtk4::{Application, ApplicationWindow, Grid, Image};

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
                let app = Application::builder()
                    .application_id("modrenjaba.mb")
                    .build();
                let command = self.command[5..].to_string();
                app.connect_activate(move |app| {
                    let lb = gtk4::Label::builder().label(&command).build();
                    let window = ApplicationWindow::builder()
                        .application(app)
                        .default_width(300)
                        .default_height(150)
                        .child(&lb)
                        .title("Modren Jaba Message")
                        .build();

                    window.present()
                });
                app.run();
            }
            COMMANDS::ONUI => {
                match gtk4::init() {
                    Ok(..) => {
                        println!("Successfully");
                    }
                    Err(..) => {
                        println!("Err!");
                        return;
                    }
                }
                let app = Application::builder()
                    .application_id("modrenjaba.ui")
                    .build();
                let banner = Image::builder()
                    .file("src/resources/banner.png")
                    .height_request(256)
                    .width_request(138)
                    .build();
                let info = gtk4::Label::builder()
                    .label(format!(
                        "Jaba 张浩扬与您同在\n当前Jaba版本:{}\n如果只想关闭窗口就请右上角关闭",
                        VERSION
                    ))
                    .build();
                let btn_info = gtk4::Button::builder().label("更多信息").build();
                let btn_site = gtk4::Button::builder().label("打开官网").build();
                let btn_close = gtk4::Button::builder().label("关闭Jaba").build();
                btn_info.connect_clicked(|_|{
                   Executer::new(COMMANDS::INFO,&String::from(format!("{}{}{}{}{}{}", "info?Jaba是jvav的改进版本，基于jvav\n从 2020 年 14 月 64 起的发行版更改了 张浩扬 Jaba 许可。\n", "新的适用于 张浩扬 Jaba SE 许可协议 与以前的 张浩扬 Jvav 许可有很大差异。\n 新许可允许某些免费使用（例如个人使用和开发使用），\n而根据以前的 张浩扬 Jvav 许可获得授权的其他使用可能会不再支持。 请在下载和使用此产品之前认真阅读条款。 可在此处查看常见问题解答。\n", "\n", "可以通过低成本的 Jaba SE 订阅 获得商业许可和技术支持。\n", "\n", "张浩扬 还在 jdk.Jaba.net 的开源 GPL 许可下提供了最新的 OpenBDK 发行版"))).exec();
                });
                btn_site.connect_clicked(|_| {
                    match webbrowser::open("https://github.com/Rov-Waff/modren_jaba") {
                        Ok(_) => println!("已在浏览器中打开"),
                        Err(_) => println!("无法打开！"),
                    }
                });
                btn_close.connect_clicked(|_| {
                    Executer::new(COMMANDS::EXIT, &String::from("exit")).exec();
                });
                let grid = Grid::builder().build();
                grid.attach(&banner, 0, 0, 1, 1);
                grid.attach(&info, 0, 1, 1, 1);
                grid.attach(&btn_info, 0, 2, 1, 1);
                grid.attach(&btn_site, 0, 3, 1, 1);
                grid.attach(&btn_close, 0, 4, 1, 1);
                app.connect_activate(move |app| {
                    let window = ApplicationWindow::builder()
                        .application(app)
                        .title("Modren Jaba")
                        .child(&grid)
                        .build();
                    window.present();
                });
                app.run();
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
