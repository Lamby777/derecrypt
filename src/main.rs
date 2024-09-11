/*
** I still have no idea what I'm doing.
**
** - RC 		9/11/2024
*/

#![feature(int_roundings)]
#![feature(slice_as_chunks)]

use std::sync::{Arc, Mutex};

use adw::Application;
use gtk::prelude::*;
use gtk::{glib, ApplicationWindow, CssProvider};

mod consts;
use consts::{APP_ID, DC_VERSION};
mod modules;

/// Program state
pub struct Derecrypt {
    pub open_modals: Vec<()>, // TODO

    pub outfile: Option<String>,
    pub string: Arc<Mutex<String>>,
}

impl Derecrypt {
    pub fn new() -> Self {
        Derecrypt {
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
        .width_request(400)
        .height_request(300)
        .title(format!("derecrypt v{}", DC_VERSION))
        .build()
        .clone();

    // Present window
    build_main_ui(&window);
    window.present();

    window.connect_close_request(|_| glib::Propagation::Proceed);
}

fn build_main_ui(_window: &ApplicationWindow) {
    // TODO
}
