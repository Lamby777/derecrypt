use std::cell::RefCell;
use std::rc::Rc;

use adw::prelude::*;
use dyn_clone::DynClone;
use gtk::Window;
// use fancy_regex::Regex;

use crate::components::blueprint::BlueprintBox;
use crate::components::spells::build_spell_editor_main_box;

mod basic;
pub use basic::{Deflate, Length, Strip};

/// The trait implemented by all string operations available in the program
pub trait DcMod: DynClone + Send + Sync {
    /// Display name of the operation
    fn op_display_name(&self) -> &'static str;

    /// Run the operation on the input
    fn run(&self, input: &str) -> String;

    /// Draw onto the box inside the blueprint editor
    fn draw(&self, _blueprint_entry: &gtk::Box) {}
}

dyn_clone::clone_trait_object!(DcMod);

/// This is not an actual module, but a list of operations.
/// There will later be a module which uses this list to apply the operations.
#[derive(Clone)]
pub struct Spell {
    pub ops: Rc<RefCell<Vec<Box<dyn DcMod>>>>,
    pub window: Window,
    pub blueprint: BlueprintBox,
}

impl Spell {
    pub fn new() -> Self {
        let ops = Rc::new(RefCell::new(vec![]));
        let window = Window::builder()
            .width_request(800)
            .height_request(600)
            .title("Edit Spell")
            .hide_on_close(true)
            .build();

        let blueprint = BlueprintBox::new();
        Self::init_window(&window, ops.clone(), blueprint.clone());
        Self {
            ops,
            window,
            blueprint,
        }
    }

    /// draw all the widgets onto the window
    pub fn init_window(
        window: &Window,
        ops: Rc<RefCell<Vec<Box<dyn DcMod>>>>,
        blueprint: BlueprintBox,
    ) {
        let main_box = build_spell_editor_main_box(ops.clone(), &blueprint);
        window.set_child(Some(&main_box));
    }

    /// run every operation in the list on the input string
    pub fn run(&mut self, input: &str) -> String {
        self.ops
            .borrow()
            .iter()
            .fold(input.to_owned(), |acc, op| op.run(&acc))
    }
}
