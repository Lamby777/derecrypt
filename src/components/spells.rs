use adw::prelude::*;
use gtk::{
    glib, Align, Button, Entry, Label, Orientation, Paned, Separator, Window,
};

use crate::MODULE_REGISTRY;

pub fn build_spells_window(app_window: &impl IsA<Window>) -> Window {
    let window = Window::builder()
        .width_request(400)
        .height_request(400)
        .transient_for(app_window)
        .build();

    let main_box = gtk::Box::builder()
        .hexpand(true)
        .orientation(Orientation::Vertical)
        .build();

    let blueprint = build_blueprint_box();
    let toolbox = build_toolbox(&blueprint);
    let paned = Paned::builder()
        .orientation(Orientation::Horizontal)
        .vexpand(true)
        .build();
    paned.set_start_child(Some(&toolbox));
    paned.set_end_child(Some(&blueprint));

    let (top_row, _spellname_label) = build_top_row();

    main_box.append(&top_row);
    main_box.append(&Separator::new(Orientation::Horizontal));
    main_box.append(&paned);

    window.set_child(Some(&main_box));

    window
}

fn build_blueprint_box() -> gtk::Box {
    let blueprint = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    blueprint.append(
        &Label::builder()
            .label("Blueprint goes here")
            .halign(Align::Center)
            .build(),
    );

    blueprint
}

fn build_toolbox(blueprint: &gtk::Box) -> gtk::Box {
    let toolbox = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    let label = Label::builder()
        .label("Toolbox")
        .name("toolbox_label")
        .halign(Align::Center)
        .build();

    toolbox.append(&label);
    toolbox.append(&Separator::new(Orientation::Horizontal));

    let registry = MODULE_REGISTRY.with(|v| v.clone());
    for (module_name, module_default) in registry.iter() {
        let button = Button::builder().label(module_name.to_owned()).build();
        toolbox.append(&button);

        // Add the module's default state to the current spell
        let module_default = dyn_clone::clone_box(*module_default);
        button.connect_clicked(glib::clone!(move |_| {
            // spell.window.present();
        }));
    }

    toolbox
}

fn build_top_row() -> (gtk::Box, Entry) {
    let top_row = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .hexpand(true)
        .build();

    // make the spell name box (can be edited)
    let spellname_entry = Entry::builder()
        .placeholder_text("Spell Name Here")
        .name("outfile_label")
        .hexpand(true)
        .build();

    let cast_button = Button::builder().label("Cast").build();
    cast_button.connect_clicked(glib::clone!(move |_| {
        // let dc = DC.read().unwrap();
    }));

    top_row.append(&spellname_entry);
    top_row.append(&cast_button);

    (top_row, spellname_entry)
}
