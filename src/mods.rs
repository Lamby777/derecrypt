/*
**	There is a reasonably high chance that
**	I might know what I'm doing.
**
** - Dex		11/8/2022
*/

use strum_macros::{EnumIter, EnumDiscriminants};

pub trait DcMod {
	fn run(&self, input: &mut String) -> ();
}

pub mod common_ops {
	pub fn replace(s: &mut String, old: &str, new: &str) {
		*s = s.as_str().replace(old, new);
	}

	pub fn deflate(s: &mut String) {
		s.retain(|c| !c.is_whitespace());
	}
}

pub mod win_s {
    use super::{WindowTypes, DcMod, common_ops};

	#[derive(Clone, Default)]
	pub struct Caster	{
		pub	name:	String,
		pub	list:	Vec<Box<WindowTypes>>,
	}

	#[derive(Clone, Default)]
	pub struct Replace	{
		pub	from:	String,
		pub	to:		String,
		pub	regex:	bool,
	}

	impl DcMod for Replace {
		fn run(&self, input: &mut String) -> () {
			common_ops::replace(input, &self.from, &self.to);
		}
	}

	#[derive(Clone, Default)]
	pub struct ConvertBase	{
		pub	from:	u32,
	}

	#[derive(Clone, Default)]
	pub struct FromASCII	{
		pub	sep:	String,
	}

	pub struct Deflate;
	impl DcMod for Deflate {
		fn run(&self, input: &mut String) -> () {
			common_ops::deflate(input);
		}
	}
}

#[derive(Clone, EnumIter, EnumDiscriminants)]
#[strum_discriminants(name(WindowDiscriminants))]
#[strum_discriminants(derive(Hash, EnumIter))]
pub enum WindowTypes {
	// holds buttons to open all the complex shit
	ModContainer,

	// holds some saved casts
	Caster			(win_s::Caster),

	// simple modules
	Strip,
	Deflate,
	Length,

	// The actual modules with config pop-out windows
	ConvertBase		(win_s::ConvertBase),
	Replace			(win_s::Replace),
	FromASCII		(win_s::FromASCII),
}
