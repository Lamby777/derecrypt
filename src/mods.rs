/*
**	There is a reasonably high chance that
**	I might know what I'm doing.
**
** - Dex		11/8/2022
*/

use strum_macros::{EnumIter, EnumDiscriminants};

pub mod win_s {
    use super::WindowTypes;

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

	#[derive(Clone, Default)]
	pub struct ConvertBase	{
		pub	from:	u32,
	}

	#[derive(Clone, Default)]
	pub struct FromASCII	{
		pub	sep:	String,
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

	// The actual modules with config pop-out windows
	ConvertBase		(win_s::ConvertBase),
	Replace			(win_s::Replace),
	FromASCII		(win_s::FromASCII),
}
