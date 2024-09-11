use std::cell::RefCell;
// use fancy_regex::Regex;
use std::collections::VecDeque;
use std::rc::Rc;

pub trait DcMod {
    fn run(&mut self, input: &str) -> String;
}

/// Something like FL Studio's "Patcher"
///
/// Basically combines multiple operations into one, however you like
#[derive(Clone, Default)]
pub struct Caster {
    /// The name given to the list of operations by the user
    pub _name: String,

    /// The list of operations to run on the input
    pub list: VecDeque<Rc<RefCell<dyn DcMod>>>,
}

impl DcMod for Caster {
    fn run(&mut self, input: &str) -> String {
        let mut output = input.to_owned();
        for cast in self.list.iter_mut() {
            output = cast.borrow_mut().run(&output);
        }

        output
    }
}

#[derive(Clone, Default)]
pub struct Deflate;
impl DcMod for Deflate {
    fn run(&mut self, input: &str) -> String {
        let mut out = input.to_owned();
        out.retain(|c| !c.is_whitespace());
        out
    }
}

#[derive(Clone, Default)]
pub struct Strip;
impl DcMod for Strip {
    fn run(&mut self, input: &str) -> String {
        input.trim().to_string()
    }
}

#[derive(Clone, Default)]
pub struct Length;
impl DcMod for Length {
    fn run(&mut self, input: &str) -> String {
        input.len().to_string()
    }
}
