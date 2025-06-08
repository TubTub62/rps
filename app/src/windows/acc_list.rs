use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};
use gtk::Button;

pub fn window_acc_list(app : &Application) -> ApplicationWindow {

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Account List")
        .build();

    let button_change_to_home = Button::builder()
        .label("Home")
        .build();

    window.set_child(Some(&button_change_to_home));

    return window;
}