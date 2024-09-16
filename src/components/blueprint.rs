use adw::prelude::*;
use gtk::{Align, Label, Orientation};

use crate::modules::DcMod;

#[derive(Clone)]
pub struct BlueprintBox {
    pub component: gtk::Box,
}

impl BlueprintBox {
    pub fn new() -> Self {
        let component = gtk::Box::builder()
            .orientation(Orientation::Vertical)
            .hexpand(true)
            .build();

        Self { component }
    }

    pub fn push_entry(&self, op: &dyn DcMod) {
        let entry = BlueprintEntryBox::new(op);
        self.component.append(&entry.component);
    }
}

pub struct BlueprintEntryBox<'a> {
    op: &'a dyn DcMod,
    component: gtk::Box,
}

impl<'a> BlueprintEntryBox<'a> {
    pub fn new(op: &'a dyn DcMod) -> Self {
        let component = Self::build_component(op);
        Self { op, component }
    }

    fn build_component(op: &dyn DcMod) -> gtk::Box {
        let entry_box = gtk::Box::builder()
            .orientation(Orientation::Vertical)
            .hexpand(true)
            .build();
        let label = Label::builder()
            .label(op.op_display_name())
            .halign(Align::Center)
            .build();
        entry_box.append(&label);
        entry_box
    }

    fn build_empty_component() -> gtk::Box {
        let blueprint_box = gtk::Box::builder()
            .orientation(Orientation::Horizontal)
            .hexpand(true)
            .vexpand(true)
            .build();

        let label = Label::builder()
            .label("Blueprint goes here")
            .halign(Align::Center)
            .build();

        blueprint_box.append(&label);
        blueprint_box
    }
}
