use dyn_clone::DynClone;
use gtk::Window;
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
#[derive(Default)]
pub struct Spell {
    pub ops: Vec<Box<dyn DcMod>>,
    pub window: Window,
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
