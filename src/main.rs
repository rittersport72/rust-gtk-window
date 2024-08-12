use gtk::prelude::*;
use gtk::{glib, Align, Application, ApplicationWindow, Button, Orientation, TextView};
use gtk4 as gtk;

use glib::clone;

const APP_ID: &str = "org.rittersport72.UpperLower";
const WIDTH: i32 = 300;
const HEIGHT: i32 = 200;
const MARGIN: i32 = 10;

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // Create text view
    let text_view = TextView::builder()
        .vexpand(true)
        .margin_top(MARGIN)
        .margin_bottom(MARGIN)
        .margin_start(MARGIN)
        .margin_end(MARGIN)
        .build();

    // Create upper button
    let upper_button = Button::builder()
        .label("Upper case")
        .margin_top(MARGIN)
        .margin_bottom(MARGIN)
        .margin_start(MARGIN)
        .margin_end(MARGIN)
        .build();

    // Create lower button
    let lower_button = Button::builder()
        .label("Lower case")
        .margin_top(MARGIN)
        .margin_bottom(MARGIN)
        .margin_start(MARGIN)
        .margin_end(MARGIN)
        .build();

    // Connect to "clicked" signal
    upper_button.connect_clicked(clone!(
        #[weak]
        text_view,
        move |_button| {
            let bounds = text_view.buffer().bounds();
            let text = text_view.buffer().text(&bounds.0, &bounds.1, true);
            let string = text.to_string().to_uppercase();
            text_view.buffer().set_text(&string);
        }
    ));

    // Connect to "clicked" signal
    lower_button.connect_clicked(clone!(
        #[weak]
        text_view,
        move |_button| {
            let bounds = text_view.buffer().bounds();
            let text = text_view.buffer().text(&bounds.0, &bounds.1, true);
            let string = text.to_string().to_lowercase();
            text_view.buffer().set_text(&string);
        }
    ));

    // Add buttons to hbox
    let h_box = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .halign(Align::Center)
        .build();
    h_box.append(&upper_button);
    h_box.append(&lower_button);

    // Add text view and hbox to vbox
    let v_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();
    v_box.append(&text_view);
    v_box.append(&h_box);

    // Create a window with vbox
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Change upper or lower case")
        .child(&v_box)
        .default_height(HEIGHT)
        .default_width(WIDTH)
        .build();

    // Present window
    window.present();
}
