/*
** I still have no idea what I'm doing.
**
** - RC 		9/11/2024
*/

use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::{LazyLock, RwLock};

use adw::gtk::{ApplicationWindow, CssProvider};
use adw::prelude::*;
use adw::{glib, Application};
use gtk::glib::Propagation;
use gtk::{gdk, EventControllerKey, Label, TextView};

mod consts;
use consts::{APP_ID, DC_VERSION};

mod modules;
use modules::{DcMod, OperationList};

mod components;
use components::{build_main_ui, open_file_dialog, update_outfile_dialog};

type OpMap = HashMap<String, OperationList>;

// good grief, what an awesome amazing way to start off the source code...
// i promise this is only used for lazy init stuff
fn leak<T>(value: T) -> &'static T {
    Box::leak(Box::new(value))
}

macro_rules! insert_modules {
    ($registry:ident; $($module:ident),*) => {{
        $(
            $registry.insert(stringify!($module), $crate::leak(modules::$module::default()));
        )*
    }};
}

static DC: LazyLock<RwLock<Derecrypt>> =
    LazyLock::new(|| RwLock::new(Derecrypt::default()));

/// List of all modules with their default settings.
/// Meant to be copied out for use in individual instances.
static MODULE_REGISTRY: LazyLock<HashMap<&'static str, &'static dyn DcMod>> =
    LazyLock::new(|| {
        let mut registry: HashMap<&'static str, &'static dyn DcMod> =
            HashMap::new();

        use modules::*;
        insert_modules!(registry; Deflate, Strip, Length);

        registry
    });

/// Program state
pub struct Derecrypt {
    /// Path to save the buffer to
    pub outfile: Option<PathBuf>,
}

impl Default for Derecrypt {
    fn default() -> Self {
        Self { outfile: None }
    }
}

fn main() -> glib::ExitCode {
    let ops_list =
        Rc::new(RefCell::new(HashMap::<String, OperationList>::new()));

    // start the gtk app
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(move |app| build_window(app, ops_list.clone()));
    app.connect_startup(|_| load_css());
    app.run()
}

fn load_css() {
    let css = CssProvider::new();
    css.load_from_string(include_str!("style.css"));

    gtk::style_context_add_provider_for_display(
        &gdk::Display::default().expect("Could not connect to a display."),
        &css,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_window(app: &Application, ops_list: Rc<RefCell<OpMap>>) {
    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .width_request(800)
        .height_request(600)
        .title(format!("derecrypt v{}", DC_VERSION))
        .build()
        .clone();

    // Present window
    let (textview, outfile_label) = build_main_ui(&window);

    let key_controller = EventControllerKey::new();
    let window2 = window.clone();
    let textview2 = textview.clone();
    key_controller.connect_key_pressed(move |_, keyval, _keycode, state| {
        let ctrl_down = state.contains(gdk::ModifierType::CONTROL_MASK);
        let shift_down = state.contains(gdk::ModifierType::SHIFT_MASK);

        if ctrl_down && keyval.to_lower() == gdk::Key::o {
            open_file_dialog(&window2, &textview2, &outfile_label, !shift_down);
            return Propagation::Stop;
        }

        if ctrl_down && keyval.to_lower() == gdk::Key::s {
            if shift_down {
                // Updates the output location and then saves the buffer to disk.
                update_outfile_dialog(&window2, &outfile_label, &textview2);
            } else {
                // Saves the buffer to disk.
                save_to_outfile(&textview2);
            }

            return Propagation::Stop;
        }

        Propagation::Proceed
    });

    // Add the key event controller to the window
    window.add_controller(key_controller);

    window.present();
    window.connect_close_request(|_| glib::Propagation::Proceed);
}

pub fn save_to_outfile(textview: &TextView) {
    let dc = DC.read().unwrap();

    let buffer = textview.buffer();
    let text = buffer.text(&buffer.start_iter(), &buffer.end_iter(), false);

    let Some(outfile) = dc.outfile.as_ref() else {
        return;
    };

    println!("Saving buffer to {:?}", &outfile);
    std::fs::write(outfile, text).unwrap();
}

fn set_outfile(outfile: impl Into<PathBuf>, outfile_label: &Label) {
    let mut dc = DC.write().unwrap();
    dc.outfile = Some(outfile.into());
    outfile_label.set_label(&outfile_fmt(&dc.outfile));
}

fn outfile_fmt(outfile: &Option<PathBuf>) -> String {
    let path = outfile
        .as_ref()
        .map(|v| v.to_string_lossy())
        .unwrap_or("<none>".into());

    format!("Output Path: {}", path)
}
