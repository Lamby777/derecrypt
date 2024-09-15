use std::cell::RefCell;
use std::rc::Rc;

use adw::prelude::*;
use gtk::{glib, Align, Button, Entry, Label, Orientation, Paned, Separator};

use crate::modules::{DcMod, Spell};
use crate::{run_spell_by_id, MODULE_REGISTRY};

pub fn build_spell_editor_main_box(spell: &Spell) -> gtk::Box {
    let main_box = gtk::Box::builder()
        .hexpand(true)
        .orientation(Orientation::Vertical)
        .build();

    let blueprint = build_blueprint_box();
    let paned = Paned::builder()
        .orientation(Orientation::Horizontal)
        .vexpand(true)
        .build();
    paned.set_start_child(Some(&build_toolbox(&blueprint, spell.ops.clone())));
    paned.set_end_child(Some(&blueprint));

    let (top_row, _spellname_label) = build_top_row();

    main_box.append(&top_row);
    main_box.append(&Separator::new(Orientation::Horizontal));
    main_box.append(&paned);

    main_box
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

fn build_toolbox(
    _blueprint: &gtk::Box,
    ops: Rc<RefCell<Vec<Box<dyn DcMod>>>>,
) -> gtk::Box {
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
        button.connect_clicked(glib::clone!(
            #[strong]
            ops,
            #[strong]
            module_default,
            move |_| {
                ops.borrow_mut().push(dyn_clone::clone_box(module_default));
            }
        ));
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
        run_spell_by_id("Length (Default)");
    }));

    top_row.append(&spellname_entry);
    top_row.append(&cast_button);

    (top_row, spellname_entry)
}
