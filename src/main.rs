/*
** I still have no idea what I'm doing.
**
** - RC 		9/11/2024
*/

use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;

use adw::gtk::{ApplicationWindow, CssProvider};
use adw::prelude::*;
use adw::{glib, Application};
use gtk::glib::Propagation;
use gtk::{gdk, EventControllerKey, Label, Overflow, TextView};

mod consts;
use consts::{APP_ID, DC_VERSION};

mod modules;
use modules::{DcMod, Spell};

mod components;
use components::{build_main_ui, open_file_dialog, update_outfile_dialog};

type SpellsMap = HashMap<String, Spell>;

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

thread_local! {
    static DC: RefCell<Derecrypt> = RefCell::new(Derecrypt::default());

    /// List of all modules with their default settings.
    /// Meant to be copied out for use in spells.
    static MODULE_REGISTRY: HashMap<&'static str, &'static dyn DcMod> = {
        let mut registry: HashMap<_, &dyn DcMod> = HashMap::new();

        use modules::*;
        insert_modules!(registry; Deflate, Strip, Length);

        registry
    };

    static SPELLS: RefCell<SpellsMap> = default_spells();
}

/// Program state
pub struct Derecrypt {
    /// Path to save the buffer to
    pub outfile: Option<PathBuf>,

    /// The main textview containing the buffer
    pub textbox: TextView,
}

impl Default for Derecrypt {
    fn default() -> Self {
        Self {
            outfile: None,
            textbox: TextView::builder()
                .hexpand(true)
                .vexpand(true)
                .monospace(true)
                .overflow(Overflow::Hidden)
                .name("buffer")
                .build(),
        }
    }
}

fn main() -> glib::ExitCode {
    // start the gtk app
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(move |app| build_window(app));
    app.connect_startup(|_| load_css());
    app.run()
}

/// List of spells the user starts out with
fn default_spells() -> RefCell<SpellsMap> {
    let mut res = SpellsMap::new();
    let mut length = Spell::new();

    length
        .ops
        .push(dyn_clone::clone_box(MODULE_REGISTRY.with(|v| v["Length"])));
    res.insert("Length (Default)".into(), length);

    RefCell::new(res)
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
    let outfile_label = build_main_ui(&window);

    let key_controller = EventControllerKey::new();
    let window2 = window.clone();
    key_controller.connect_key_pressed(move |_, keyval, _keycode, state| {
        let ctrl_down = state.contains(gdk::ModifierType::CONTROL_MASK);
        let shift_down = state.contains(gdk::ModifierType::SHIFT_MASK);

        if ctrl_down && keyval.to_lower() == gdk::Key::o {
            open_file_dialog(&window2, &outfile_label, !shift_down);
            return Propagation::Stop;
        }

        let textbox = DC.with_borrow(|dc| dc.textbox.clone());
        if ctrl_down && keyval.to_lower() == gdk::Key::s {
            if shift_down {
                // Updates the output location and then saves the buffer to disk.
                update_outfile_dialog(&window2, &outfile_label, &textbox);
            } else {
                // Saves the buffer to disk.
                save_to_outfile(&textbox);
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
    let buffer = textview.buffer();
    let text = buffer.text(&buffer.start_iter(), &buffer.end_iter(), false);

    let outfile = DC.with_borrow(|dc| dc.outfile.clone());

    let Some(outfile) = outfile.as_ref() else {
        return;
    };

    println!("Saving buffer to {:?}", &outfile);
    std::fs::write(outfile, text).unwrap();
}

fn set_outfile(outfile: impl Into<PathBuf>, outfile_label: &Label) {
    let outfile = DC.with_borrow_mut(|v| {
        v.outfile = Some(outfile.into());
        v.outfile.clone()
    });

    outfile_label.set_label(&outfile_fmt(&outfile));
}

fn outfile_fmt(outfile: &Option<PathBuf>) -> String {
    let path = outfile
        .as_ref()
        .map(|v| v.to_string_lossy())
        .unwrap_or("<none>".into());

    format!("Output Path: {}", path)
}
