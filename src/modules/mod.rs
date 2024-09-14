use dyn_clone::DynClone;
use gtk::{ApplicationWindow, Window};

use crate::components::spells::build_spells_window;

// use fancy_regex::Regex;

pub trait DcMod: DynClone + Send + Sync {
    /// Display name of the operation
    fn op_display_name(&self) -> &'static str;

    /// Run the operation on the input
    fn run(&mut self, input: &str) -> String;

    /// Draw onto the GTK window
    fn draw(&self, _window: &Window) {}
}

dyn_clone::clone_trait_object!(DcMod);

/// This is not an actual module, but a list of operations.
/// There will later be a module which uses this list to apply the operations.
#[derive(Clone)]
pub struct Spell {
    pub ops: Vec<Box<dyn DcMod>>,
    pub window: Window,
}

impl Spell {
    pub fn new(app_window: &ApplicationWindow) -> Self {
        Self {
            ops: vec![],
            window: build_spells_window(app_window),
        }
    }
}

#[derive(Clone, Default)]
pub struct Deflate;
impl DcMod for Deflate {
    fn op_display_name(&self) -> &'static str {
        "Deflate"
    }

    fn run(&mut self, input: &str) -> String {
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

    fn run(&mut self, input: &str) -> String {
        input.trim().to_string()
    }
}

#[derive(Clone, Default)]
pub struct Length;
impl DcMod for Length {
    fn op_display_name(&self) -> &'static str {
        "Length"
    }

    fn run(&mut self, input: &str) -> String {
        input.len().to_string()
    }
}
