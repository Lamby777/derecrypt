/*
** I still have no idea what I'm doing.
**
** - RC 		9/11/2024
*/

#![feature(int_roundings)]
#![feature(slice_as_chunks)]

use std::sync::{Arc, Mutex};

use adw::gtk::{ApplicationWindow, CssProvider};
use adw::prelude::*;
use adw::{glib, Application};

mod consts;
use consts::{APP_ID, DC_VERSION};
use gtk::gio;
mod modules;

/// Program state
pub struct Derecrypt {
    pub open_modals: Vec<()>, // TODO

    pub outfile: Option<String>,
    pub string: Arc<Mutex<String>>,
}

impl Default for Derecrypt {
    fn default() -> Self {
        Self {
            open_modals: vec![],
            outfile: None,
            string: Arc::new(Mutex::new(String::new())),
        }
    }
}

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_window);
    app.connect_startup(|_| load_css());
    app.run()
}

fn load_css() {
    let css = CssProvider::new();
    css.load_from_string(include_str!("style.css"));

    gtk::style_context_add_provider_for_display(
        &gtk::gdk::Display::default().expect("Could not connect to a display."),
        &css,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_window(app: &Application) {
    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .width_request(800)
        .height_request(600)
        .title(format!("derecrypt v{}", DC_VERSION))
        .build()
        .clone();

    // Present window
    build_main_ui(&window);
    window.present();

    window.connect_close_request(|_| glib::Propagation::Proceed);
}

fn build_main_ui(window: &ApplicationWindow) {
    let main_box = gtk::Box::builder()
        .hexpand(true)
        .orientation(gtk::Orientation::Vertical)
        .build();

    let top_row = top_row(window);

    let text_view =
        gtk::TextView::builder().hexpand(true).vexpand(true).build();

    main_box.append(&top_row);
    main_box.append(&text_view);

    window.set_child(Some(&main_box));
}

fn top_row(window: &ApplicationWindow) -> gtk::Box {
    let top_row = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .build();

    let cast_button = gtk::Button::builder().label("Cast").build();
    let open_button = gtk::Button::builder().label("Open").build();
    let save_button = gtk::Button::builder().label("Save").build();

    let window2 = window.clone();
    open_button.connect_clicked(move |_| {
        let dialog = gtk::FileDialog::builder().build();
        let cancellable = gio::Cancellable::new();

        dialog.open(Some(&window2), Some(&cancellable), |file| {
            let _ = dbg!(file);
        });
    });

    top_row.append(&cast_button);
    top_row.append(&open_button);
    top_row.append(&save_button);

    top_row
}
