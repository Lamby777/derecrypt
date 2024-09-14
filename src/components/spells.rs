use std::cell::RefCell;
use std::rc::Rc;

use adw::prelude::*;
use gtk::{glib, Align, Button, Label, Orientation, Separator, Window};

use crate::SpellsMap;

pub fn build_spells_window(app_window: &impl IsA<Window>) -> Window {
    Window::builder().transient_for(app_window).build()
}

pub fn build_spells_box(spells: &'static Rc<RefCell<SpellsMap>>) -> gtk::Box {
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

    for (spell_name, spell) in spells.borrow().iter() {
        let button = Button::builder().label(spell_name).build();
        spells_box.append(&button);

        button.connect_clicked(glib::clone!(
            #[strong]
            spell,
            move |_| {
                spell.window.present();
            }
        ));
    }

    spells_box
}
