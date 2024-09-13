use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

// use gtk::Window;
// use fancy_regex::Regex;

pub trait DcMod {
    /// The `Default` trait requires the `Sized` bound, so we can't use that.
    fn default() -> Self
    where
        Self: Sized;

    /// Run the operation on the input
    fn run(&mut self, input: &str) -> String;

    // Draw onto the GTK window
    // fn draw(&self, window: &Window) {}
}

/// Something like FL Studio's "Patcher"
///
/// Basically combines multiple operations into one, however you like
#[derive(Clone)]
pub struct Caster {
    /// The name given to the list of operations by the user
    pub _name: String,

    /// The list of operations to run on the input
    pub list: VecDeque<Rc<RefCell<dyn DcMod>>>,
}

impl DcMod for Caster {
    fn default() -> Self {
        Self {
            _name: "Caster".to_string(),
            list: VecDeque::new(),
        }
    }

    fn run(&mut self, input: &str) -> String {
        let mut output = input.to_owned();
        for cast in self.list.iter_mut() {
            output = cast.borrow_mut().run(&output);
        }

        output
    }
}

#[derive(Clone)]
pub struct Deflate;
impl DcMod for Deflate {
    fn default() -> Self {
        Self
    }

    fn run(&mut self, input: &str) -> String {
        let mut out = input.to_owned();
        out.retain(|c| !c.is_whitespace());
        out
    }
}

#[derive(Clone)]
pub struct Strip;
impl DcMod for Strip {
    fn default() -> Self {
        Self
    }

    fn run(&mut self, input: &str) -> String {
        input.trim().to_string()
    }
}

#[derive(Clone)]
pub struct Length;
impl DcMod for Length {
    fn default() -> Self {
        Self
    }

    fn run(&mut self, input: &str) -> String {
        input.len().to_string()
    }
}
