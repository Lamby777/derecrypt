use adw::gtk::ApplicationWindow;
use adw::prelude::*;
use gtk::gio::Cancellable;
use gtk::{
    glib, Align, Button, FileDialog, Label, Orientation, Overflow, Paned,
    ScrolledWindow, Separator,
};

pub mod spells;

use crate::{outfile_fmt, run_spell, save_to_outfile, set_outfile, DC, SPELLS};

pub fn build_main_ui(window: &ApplicationWindow) -> Label {
    let main_box = gtk::Box::builder()
        .hexpand(true)
        .orientation(Orientation::Vertical)
        .build();

    let (paned, spells_box) = build_main_paned();
    let (top_row, outfile_label) = build_top_row(window, &paned, &spells_box);

    main_box.append(&top_row);
    main_box.append(&Separator::new(Orientation::Horizontal));
    main_box.append(&paned);

    window.set_child(Some(&main_box));
    outfile_label
}

fn build_main_paned() -> (Paned, gtk::Box) {
    let pane = Paned::builder().build();

    let spells_box = build_spells_box();
    let textbox_scroll = build_text_box_scroll();

    // pane.set_start_child(Some(&toolbox));
    pane.set_end_child(Some(&textbox_scroll));

    (pane, spells_box)
}

/// Build the box containing a list of spells created.
pub fn build_spells_box() -> gtk::Box {
    let spells_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    let label = Label::builder()
        .label("Spells")
        .name("spells_box_label")
        .halign(Align::Center)
        .build();

    spells_box.append(&label);
    spells_box.append(&Separator::new(Orientation::Horizontal));

    let spells = SPELLS.with_borrow(|spells| spells.clone());

    for (spell_name, _) in spells.iter() {
        let button = Button::builder().label(spell_name).build();
        spells_box.append(&button);

        // if left click, open the editor
        button.connect_clicked(glib::clone!(
            #[strong]
            spell_name,
            move |_| {
                run_spell(&spell_name);
            }
        ));

        // if right click, run the spell
        let gesture = gtk::GestureClick::builder()
            .button(gtk::gdk::ffi::GDK_BUTTON_SECONDARY as u32)
            .build();

        gesture.connect_pressed(glib::clone!(
            #[strong]
            spell_name,
            move |gesture, _, _, _| {
                gesture.set_state(gtk::EventSequenceState::Claimed);
                SPELLS
                    .with_borrow(|spells| spells.clone())
                    .get(&spell_name)
                    .unwrap()
                    .window
                    .present();
            }
        ));
        button.add_controller(gesture);
    }

    spells_box
}

/// build the scrollable window containing the textview
fn build_text_box_scroll() -> ScrolledWindow {
    const SCROLL_MARGIN: i32 = 15;

    let textview = DC.with_borrow(|dc| dc.textbox.clone());

    ScrolledWindow::builder()
        .overflow(Overflow::Hidden)
        .width_request(600)
        .margin_top(SCROLL_MARGIN)
        .margin_bottom(SCROLL_MARGIN)
        .margin_start(SCROLL_MARGIN)
        .margin_end(SCROLL_MARGIN)
        .child(&textview)
        .build()
}

/// Open a file dialog to select a file to open
pub fn open_file_dialog(
    window: &ApplicationWindow,
    outfile_label: &Label,
    update_outfile_path: bool,
) {
    let dialog = FileDialog::builder().build();

    let outfile_label2 = outfile_label.clone();
    dialog.open(Some(window), None::<&Cancellable>, move |file| {
        let Ok(file) = file else {
            println!("No file selected.");
            return;
        };

        let path = file.path().unwrap();
        let content = std::fs::read_to_string(&path).unwrap();

        DC.with_borrow(|dc| dc.textbox.buffer().set_text(&content));

        if update_outfile_path {
            set_outfile(path, &outfile_label2)
        }
    });
}

/// Open a file dialog to select a file to open
pub fn update_outfile_dialog(
    window: &ApplicationWindow,
    outfile_label: &Label,
) {
    let dialog = FileDialog::builder().build();

    let outfile_label2 = outfile_label.clone();
    dialog.save(Some(window), None::<&Cancellable>, move |file| {
        let Ok(file) = file else {
            println!("No file selected.");
            return;
        };

        let path = file.path().unwrap();
        println!("Updating outfile path to {:?}", path);

        set_outfile(path, &outfile_label2);
        save_to_outfile();
    });
}

fn build_top_row(
    window: &ApplicationWindow,
    paned: &Paned,
    spells_box: &gtk::Box,
) -> (gtk::Box, Label) {
    let top_row = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .hexpand(true)
        .homogeneous(true)
        .build();

    // make the file menu buttons
    let buttons_box = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .margin_end(100)
        .build();

    let spells_box_button = Button::builder().label("Spells").build();
    let open_button = Button::builder().label("Open").build();
    let save_button = Button::builder().label("Save").build();

    buttons_box.append(&spells_box_button);
    buttons_box.append(&open_button);
    buttons_box.append(&save_button);

    // make the output path label
    let outfile_label = Label::builder()
        .label(outfile_fmt(&DC.with_borrow(|dc| dc.outfile.clone())))
        .name("outfile_label")
        .build();

    // connect the button signals
    let window2 = window.clone();
    let outfile_label2 = outfile_label.clone();
    open_button.connect_clicked(move |_| {
        open_file_dialog(&window2, &outfile_label2, true)
    });

    let paned2 = paned.clone();
    let toolbox2 = spells_box.clone();
    spells_box_button.connect_clicked(move |_| {
        // toggle the toolbox visibility
        let new_child = match paned2.start_child() {
            Some(_) => None,
            None => Some(&toolbox2),
        };

        paned2.set_start_child(new_child);
    });

    top_row.append(&buttons_box);
    top_row.append(&outfile_label);
    // top_row.append(&cast_box);

    (top_row, outfile_label)
}
