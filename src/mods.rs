/*
**	There is a reasonably high chance that
**	I might know what I'm doing.
**
** - Dex		11/8/2022
*/

use num::*;
use strum_macros::{EnumIter, EnumDiscriminants};
use enum_dispatch::*;

#[enum_dispatch]
pub trait DcMod {
	fn run(&mut self, input: &mut String) -> ();
}

pub mod common_ops {
	pub fn replace(s: &mut String, old: &str, new: &str) {
		*s = s.as_str().replace(old, new);
	}

	pub fn deflate(s: &mut String) {
		s.retain(|c| !c.is_whitespace());
	}
}

#[derive(Clone, EnumIter, EnumDiscriminants)]
#[strum_discriminants(name(WindowDiscriminants))]
#[strum_discriminants(derive(Hash, EnumIter))]
#[enum_dispatch(DcMod)]
pub enum WindowTypes {
	// holds buttons to open all the complex shit
	ModContainer		(win_s::ModContainer),

	// holds some saved casts
	Caster				(win_s::Caster),

	// simple modules
	Strip				(win_s::Strip),
	Deflate				(win_s::Deflate),
	Length				(win_s::Length),

	// The actual modules with config pop-out windows
	ConvertBase			(win_s::ConvertBase),
	Replace				(win_s::Replace),
	FromEscapedASCII	(win_s::FromEscapedASCII),
	FromASCII			(win_s::FromASCII),
}



pub mod win_s {
	use std::collections::VecDeque;
	use crate::tfd;
    use crate::consts::*;
    use super::{WindowTypes, DcMod, common_ops};

	#[derive(Clone, Default)]
	pub struct Caster	{
		pub	name:	String,
		pub	list:	VecDeque<Box<WindowTypes>>,
	}

	impl DcMod for Caster {
		fn run(&mut self, _input: &mut String) -> () {
			panic!("This module does not run!");
		}
	}


	#[derive(Clone, Default)]
	pub struct Replace	{
		pub	from:	String,
		pub	to:		String,
		pub	regex:	bool,
	}

	impl DcMod for Replace {
		fn run(&mut self, input: &mut String) -> () {
			common_ops::replace(input, &self.from, &self.to);
		}
	}


	#[derive(Clone, Default)]
	pub struct ConvertBase	{
		pub	from:	u32,
	}

	impl DcMod for ConvertBase {
		fn run(&mut self, input: &mut String) -> () {
			// If "from" not in range, set to binary
			if !(2..=36).contains(&self.from) {
				self.from = 2;
			}

			// Deflate accidental whitespace
			common_ops::deflate(input);

			let res = u128::from_str_radix(
				input, self.from
			);

			match res {
				Ok(v) => {
					*input = v.to_string();
				},

				Err(v) => match v.kind() {
					core::num::IntErrorKind::PosOverflow => {
						tfd::message_box_ok(
							APP_NAME_STR,
							"Attempting to calculate this caused \
							a positive integer overflow.",
							tfd::MessageBoxIcon::Error
						);
					},

					_ => {
						tfd::message_box_ok(
							APP_NAME_STR,
							"Number is invalid for this base!\n\
							Did you mean to use the ASCII module?",
							tfd::MessageBoxIcon::Error
						);
					}
				}
			}
		}
	}


	#[derive(Clone, Default)]
	pub struct FromEscapedASCII	{
		pub	sep:	String,
	}

	impl DcMod for FromEscapedASCII {
		fn run(&mut self, input: &mut String) -> () {
			let rsep = if self.sep.len() > 0 { self.sep.as_str() } else {
				// If no separator is specified, assume there is nothing
				// between each escape sequence, so replace each "\"
				// with " \" and use " " as the separator.
				
				common_ops::replace(
					input,
					"\\", " \\"
				);

				" "
			};

			// Split string by delim
			let bytes: Vec<&str> = input.split(rsep).collect();

			let mut res = String::new();

			// For each piece, either decode it, OR if it's not
			// encoded, keep it the same.
			for b in bytes {
				if b.len() < 2 {
					res = format!("{res}{b}");
					continue;
				}

				// example: with \u0000, these bindings would be:
				let slice	= &b[2..];						// "0000"
				let mode	= b.chars().nth(1).unwrap();	// 'u'

				let charcode =
					u32::from_str_radix(slice,
						match mode {
							'x' | 'u'	=> 16,
							'0'			=> 8,

							// non-standard, but might
							// be useful for some people
							'd'			=> 10,


							_			=> {
								res = format!("{res}{b}");
								continue;
							}
						}
					);
				
				if charcode.is_err() {
					res = format!("{res}{b}");
					continue;
				}
				
				let nchar = char::from_u32(charcode.unwrap())
								.unwrap_or('?');

				res.push(nchar);
			}

			*input = res;
		}
	}


	#[derive(Clone)]
	pub struct FromASCII	{
		pub	sep:	String,
		pub mode:	ASCIIBases
	}

	impl Default for FromASCII {
		fn default() -> Self {
			FromASCII {
				sep:	String::from(" "),
				mode:	ASCIIBases::Hexadecimal
			}
		}
	}

	impl DcMod for FromASCII {
		fn run(&mut self, input: &mut String) -> () {
			// Length of byte representations for the current mode
			// (ex. this will be 2 in hex mode, 8 in binary mode, etc.)
			let byte_repr_len	= 16u8.div_ceil(self.mode.into());

			let rsep = if self.sep.len() > 0 { self.sep.as_str() } else {
				// If no separator is specified, assume there is nothing
				// between each escape sequence, so replace each "\"
				// with " \" and use " " as the separator.
				
				common_ops::replace(
					input,
					"\\", " \\"
				);

				" "
			};

			// Split string by delim
			let bytes: Vec<&str> = input.split(rsep).collect();

			let mut res = String::new();

			// For each piece, either decode it, OR if it's not
			// encoded, keep it the same.
			for b in bytes {
				if b.len() < 2 {
					res = format!("{res}{b}");
					continue;
				}

				// example: with \u0000, these bindings would be:
				let slice	= &b[2..];						// "0000"
				let mode	= b.chars().nth(1).unwrap();	// 'u'

				let charcode =
					u32::from_str_radix(slice,
						match mode {
							'x' | 'u'	=> 16,
							'0'			=> 8,

							// non-standard, but might
							// be useful for some people
							'd'			=> 10,


							_			=> {
								res = format!("{res}{b}");
								continue;
							}
						}
					);
				
				if charcode.is_err() {
					res = format!("{res}{b}");
					continue;
				}
				
				let nchar = char::from_u32(charcode.unwrap())
								.unwrap_or('?');

				res.push(nchar);
			}

			*input = res;
		}
	}


	#[derive(Clone, Default)]
	pub struct Deflate;
	impl DcMod for Deflate {
		fn run(&mut self, input: &mut String) -> () {
			common_ops::deflate(input);
		}
	}

	#[derive(Clone, Default)]
	pub struct Strip;
	impl DcMod for Strip {
		fn run(&mut self, input: &mut String) -> () {
			*input = input.trim().to_string();
		}
	}

	#[derive(Clone, Default)]
	pub struct Length;
	impl DcMod for Length {
		fn run(&mut self, input: &mut String) -> () {
			*input = input.len().to_string();
		}
	}

	#[derive(Clone, Default)]
	pub struct ModContainer;
	impl DcMod for ModContainer {
		fn run(&mut self, _input: &mut String) -> () {
			panic!("This module does not run!");
		}
	}
}
