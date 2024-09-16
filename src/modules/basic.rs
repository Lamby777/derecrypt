use super::*;

#[derive(Clone, Default)]
pub struct Append {
    string: String,
    prepend: bool,
}

impl DcMod for Append {
    fn op_display_name(&self) -> &'static str {
        "Append"
    }

    fn run(&self, input: &str) -> String {
        match self.prepend {
            true => format!("{}{}", self.string, input),
            false => format!("{}{}", input, self.string),
        }
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
