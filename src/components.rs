use std::path::PathBuf;

use adw::gtk::ApplicationWindow;
use adw::prelude::*;
use gtk::gio::Cancellable;
use gtk::{Label, Overflow, TextView};

use crate::{outfile_fmt, save_to_outfile, set_outfile, DC};

pub fn build_main_ui(window: &ApplicationWindow) -> (TextView, Label) {
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
pub fn open_file_dialog(
    window: &ApplicationWindow,
    textbox: &TextView,
    outfile_label: &Label,
    update_outfile_path: bool,
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

        if update_outfile_path {
            set_outfile(path, &outfile_label2)
        }
    });
}

/// Open a file dialog to select a file to open
pub fn update_outfile_dialog(
    window: &ApplicationWindow,
    outfile_label: &Label,
    textbox: &TextView,
) {
    let dialog = gtk::FileDialog::builder().build();

    let outfile_label2 = outfile_label.clone();
    let textbox2 = textbox.clone();
    dialog.save(Some(window), None::<&Cancellable>, move |file| {
        let Ok(file) = file else {
            println!("No file selected.");
            return;
        };

        let path = file.path().unwrap();
        println!("Updating outfile path to {:?}", path);

        set_outfile(path, &outfile_label2);
        save_to_outfile(&textbox2);
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
