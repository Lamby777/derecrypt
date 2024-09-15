use std::cell::RefCell;
use std::rc::Rc;

use adw::prelude::*;
use dyn_clone::DynClone;
use gtk::Window;

use crate::components::spells::build_spells_main_box;

// use fancy_regex::Regex;

pub trait DcMod: DynClone + Send + Sync {
    /// Display name of the operation
    fn op_display_name(&self) -> &'static str;

    /// Run the operation on the input
    fn run(&self, input: &str) -> String;

    /// Draw onto the GTK window
    fn draw(&self, _window: &Window) {}
}

dyn_clone::clone_trait_object!(DcMod);

/// This is not an actual module, but a list of operations.
/// There will later be a module which uses this list to apply the operations.
#[derive(Clone)]
pub struct Spell {
    pub ops: Rc<RefCell<Vec<Box<dyn DcMod>>>>,
    pub window: Window,
}

impl Spell {
    pub fn new() -> Self {
        let ops = Rc::new(RefCell::new(vec![]));
        let window = Window::builder()
            .width_request(800)
            .height_request(600)
            .title("Edit Spell")
            .build();

        let res = Self { ops, window };
        res.init_window();
        res
    }

    /// draw all the widgets onto the window
    pub fn init_window(&self) {
        let main_box = build_spells_main_box(self);
        self.window.set_child(Some(&main_box));
    }

    /// run every operation in the list on the input string
    pub fn run(&mut self, input: &str) -> String {
        self.ops
            .borrow()
            .iter()
            .fold(input.to_owned(), |acc, op| op.run(&acc))
    }
}

#[derive(Clone, Default)]
pub struct Deflate;
impl DcMod for Deflate {
    fn op_display_name(&self) -> &'static str {
        "Deflate"
    }

    fn run(&self, input: &str) -> String {
        let mut out = input.to_owned();
        out.retain(|c| !c.is_whitespace());
        out
    }
}

#[derive(Clone, Default)]
pub struct Strip;
impl DcMod for Strip {
    fn op_display_name(&self) -> &'static str {
        "Strip"
    }

    fn run(&self, input: &str) -> String {
        input.trim().to_string()
    }
}

#[derive(Clone, Default)]
pub struct Length;
impl DcMod for Length {
    fn op_display_name(&self) -> &'static str {
        "Length"
    }

    fn run(&self, input: &str) -> String {
        input.len().to_string()
    }
}
