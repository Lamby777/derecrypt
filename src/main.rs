/*
** I still have no idea what I'm doing.
**
** - RC 		9/11/2024
*/

mod consts;
use consts::{APP_ID, DC_VERSION};
use gtk::glib::Propagation;
mod modules;

use std::sync::RwLock;

use adw::gtk::{ApplicationWindow, CssProvider};
use adw::prelude::*;
use adw::{glib, Application};
use gtk::gio::Cancellable;
use gtk::{gdk, EventControllerKey, Label, Overflow, TextView};

static DC: RwLock<Derecrypt> = RwLock::new(Derecrypt::new());

/// Program state
pub struct Derecrypt {
    pub outfile: Option<String>,
}

impl Derecrypt {
    const fn new() -> Self {
        Self { outfile: None }
    }
}

fn main() -> glib::ExitCode {
    // start the gtk app
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

        Propagation::Proceed
    });

    // Add the key event controller to the window
    window.add_controller(key_controller);

    window.present();
    window.connect_close_request(|_| glib::Propagation::Proceed);
}

fn build_main_ui(window: &ApplicationWindow) -> (TextView, Label) {
    let main_box = gtk::Box::builder()
        .hexpand(true)
        .orientation(gtk::Orientation::Vertical)
        .build();

    let (textbox, textview) = build_text_box();
    let (top_row, outfile_label) = build_top_row(window, &textview);

    main_box.append(&top_row);
    main_box.append(&gtk::Separator::new(gtk::Orientation::Horizontal));
    main_box.append(&textbox);

    window.set_child(Some(&main_box));
    (textview, outfile_label)
}

fn build_text_box() -> (gtk::ScrolledWindow, gtk::TextView) {
    const SCROLL_MARGIN: i32 = 15;
    let scroll = gtk::ScrolledWindow::builder()
        .overflow(Overflow::Hidden)
        .margin_top(SCROLL_MARGIN)
        .margin_bottom(SCROLL_MARGIN)
        .margin_start(SCROLL_MARGIN)
        .margin_end(SCROLL_MARGIN)
        .build();

    let textview = gtk::TextView::builder()
        .hexpand(true)
        .vexpand(true)
        .monospace(true)
        .overflow(Overflow::Hidden)
        .name("buffer")
        .build();

    scroll.set_child(Some(&textview));
    (scroll, textview)
}

/// Open a file dialog to select a file to open
fn open_file_dialog(
    window: &ApplicationWindow,
    textbox: &TextView,
    outfile_label: &Label,
    overwrite_outfile: bool,
) {
    let dialog = gtk::FileDialog::builder().build();

    let textbox2 = textbox.clone();
    let outfile_label2 = outfile_label.clone();
    dialog.open(Some(window), None::<&Cancellable>, move |file| {
        let Ok(file) = file else {
            println!("No file selected.");
            return;
        };

        let path = file.path().unwrap();
        let content = std::fs::read_to_string(&path).unwrap();

        textbox2.buffer().set_text(&content);
        if overwrite_outfile {
            let path_str = path.to_str().unwrap().to_string();

            let mut dc = DC.write().unwrap();
            dc.outfile = Some(path_str);
            outfile_label2.set_label(&outfile_fmt(&dc.outfile));
        }
    });
}

fn build_top_row(
    window: &ApplicationWindow,
    textbox: &TextView,
) -> (gtk::Box, Label) {
    let dc = DC.read().unwrap();

    let top_row = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        // .homogeneous(true)
        .build();

    // make the file menu buttons
    let buttons_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .margin_end(100)
        .build();

    let cast_button = gtk::Button::builder().label("Cast").build();
    let open_button = gtk::Button::builder().label("Open").build();
    let save_button = gtk::Button::builder().label("Save").build();

    buttons_box.append(&cast_button);
    buttons_box.append(&open_button);
    buttons_box.append(&save_button);

    // make the output path label
    let outfile_label = gtk::Label::builder()
        .label(outfile_fmt(&dc.outfile))
        .name("outfile_label")
        .build();

    // make the 3rd box (idk what it's used for yet)
    let box3 = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .build();

    // connect the button signals
    let window2 = window.clone();
    let textbox2 = textbox.clone();
    let outfile_label2 = outfile_label.clone();
    open_button.connect_clicked(move |_| {
        open_file_dialog(&window2, &textbox2, &outfile_label2, true)
    });

    top_row.append(&buttons_box);
    top_row.append(&outfile_label);
    top_row.append(&box3);

    (top_row, outfile_label)
}

fn outfile_fmt(outfile: &Option<String>) -> String {
    let path = outfile
        .as_ref()
        .map(|v| v.to_string())
        .unwrap_or("<none>".to_string());

    format!("Output Path: {}", path)
}
