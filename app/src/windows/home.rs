use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};
use gtk::Button;

use super::acc_list::window_acc_list;

/* fn change_window(app : &Application) {
    app.connect_activate(|app| {
        let new_window = window_acc_list(app);
        app.connect_window_added(move |_| {
            new_window.show();
        });
    });
} */

pub fn window_home() -> ApplicationWindow {
    let window = ApplicationWindow::builder()
        //.application(app)
        .default_height(500)
        .default_width(500)
        .title("Home")
        .build();

    let button_change_to_acc_list = Button::builder()
        .label("Account List")
        //.border_width(10)
        .build();

    let wlist = app.windows();
    println!("{:?}", wlist);


    button_change_to_acc_list.connect_clicked(move |_| {
        let window_acc_list = window_acc_list();
        window_acc_list.show();
    });

    window.set_child(Some(&button_change_to_acc_list));

    return window
}