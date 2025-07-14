use crate::command::COMMANDS;
use crate::executer::{Executer, VERSION};
use gtk4::prelude::{ApplicationExt, EditableExt};
use gtk4::prelude::{ApplicationExtManual, ButtonExt, GridExt, GtkWindowExt};
use gtk4::{Application, ApplicationWindow, Grid, Image};
use std::cell::RefCell;
use std::rc::Rc;

pub fn show_main_window() {
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
    let btn_ide = gtk4::Button::builder().label("打开IDE").build();
    let input_file = Rc::new(RefCell::new(gtk4::Entry::builder().build()));
    let btn_run_script = gtk4::Button::builder().label("编译Jaba脚本").build();

    let input_file_clone = Rc::clone(&input_file);
    btn_run_script.connect_clicked(move |_| {
        let text = input_file_clone.borrow().text().to_string();
        Executer::new(COMMANDS::RUN, &format!("IDE?{}", text));
    });
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
    btn_ide.connect_clicked(|_| {
        Executer::new(COMMANDS::IDE, &String::from("IDE")).exec();
    });
    let grid = Grid::builder().build();
    grid.attach(&banner, 0, 0, 1, 1);
    grid.attach(&info, 0, 1, 1, 1);
    grid.attach(&btn_info, 0, 2, 1, 1);
    grid.attach(&btn_site, 0, 3, 1, 1);
    grid.attach(&btn_close, 0, 4, 1, 1);
    grid.attach(&btn_ide, 0, 5, 1, 1);
    grid.attach(&*input_file.borrow(), 0, 6, 1, 1);
    grid.attach(&btn_run_script,0,7,1,1);

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
