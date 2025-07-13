use crate::command::COMMANDS;
use chrono::Local;
use gtk4::gdk_pixbuf::Pixbuf;
use gtk4::glib::BoolError;
use gtk4::prelude::{ApplicationExt, ApplicationExtManual, GridExt, GtkWindowExt};
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
                webbrowser::open("https://rov-waff.github.io/modren_jaba_docs/")
                    .expect("无法打开帮助文档！");
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
                    .label(
                        "Jaba 张浩扬与您同在\n当前Jaba版本:1.0.2\n如果只想关闭窗口就请右上角关闭",
                    )
                    .build();
                let btn_info = gtk4::Button::builder().label("更多信息").build();
                let btn_site = gtk4::Button::builder().label("打开官网").build();
                let btn_close = gtk4::Button::builder().label("关闭Jaba").build();
                let grid = Grid::builder().build();
                grid.attach(&banner, 0, 0, 1, 1);
                grid.attach(&info, 0, 1, 1, 1);
                grid.attach(&btn_info, 0, 2, 1, 1);
                grid.attach(&btn_site, 0, 3, 1, 1);
                grid.attach(&btn_close, 0, 4, 1, 1);
                app.connect_activate(move |app| {
                    let window = ApplicationWindow::builder()
                        .application(app)
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
