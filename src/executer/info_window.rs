use gtk4::{Application, ApplicationWindow};
use gtk4::prelude::{ApplicationExt, ApplicationExtManual, GtkWindowExt};

pub fn show_info_window(command:&String){
    let app = Application::builder()
        .application_id("modrenjaba.mb")
        .build();
    let command = command[5..].to_string();
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