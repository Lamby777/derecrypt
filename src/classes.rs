/*
**OOP stuff to be used in main.rs
*/

use tinyfiledialogs as tfd;
use eframe::{egui::*};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use std::{path::Path, collections::HashMap};

use super::consts::*;

pub struct ThemeColors;

impl ThemeColors {
	pub const BG_PURPLE:		Color32	= Color32::from_rgb(79,	0,	148);
	pub const BG_PURPLE_DEEP:	Color32	= Color32::from_rgb(42,	0,	79);
	pub const BG_PURPLE_LIGHT:	Color32	= Color32::from_rgb(142,	24,	240);
	pub const BG_PURPLE_DARK:	Color32	= Color32::from_rgb(18,	0,	33);
	pub const TEXT:				Color32 = Color32::WHITE;
}

#[derive(Eq, Hash, PartialEq, Clone, EnumIter)]
pub enum WindowTypes {
	ConvertBase,
	Replace,
}

pub mod dc_modules {
	use super::{DcModBase, DcMod};

	pub struct ConvertBase {
		pub module:	DcModBase,
		pub	from:	u64,
		pub	to:		u64,
	}

	

	impl DcMod for ConvertBase {
		fn module(&self) -> &DcModBase {
			&self.module
		}
	}
}

pub trait DcMod {
	fn module(&self) -> &DcModBase;
}

pub struct DcModBase {
	pub	active:		bool,
}

pub struct Derecrypt {
	pub	open_modals:	HashMap<WindowTypes, DcModBase>,
	pub	outfile:		Option<String>,
	pub	string:			String,
}

impl Derecrypt {
	pub fn new() -> Self {
		let mut modals_map = HashMap::new();

		// initialize all the modules
		for wintype in WindowTypes::iter() {
			modals_map.insert(wintype, DcModBase {
				active:		false,
			});
		}

		Derecrypt {
			open_modals:	modals_map,
			outfile:		None,
			string:			String::new(),
		}
	}

	pub fn filename(&self) -> Option<String> {
		let outfile = &self.outfile.as_ref();

		if let Some(v) = outfile {
			Some(Path::new(v).file_name().unwrap().to_str().unwrap().to_string())
		} else {
			None
		}
	}

	
	pub fn toggle_module_visibility(&mut self, wintype: WindowTypes) {
		self.open_modals.entry(wintype)
			.and_modify(|val| val.active = !val.active);
	}

	// Pops up a dialog to open a new file, and then asks
	// if the selected path should be the new output path
	pub fn get_desired_path(&mut self, save: bool, force_overwrite: bool) -> String {
		let fname;

		loop {
			let chosen = if save {
				tfd::save_file_dialog(
					"Save To File", ""
				)
			} else {
				tfd::open_file_dialog(
					"Load String From File", "", None
				)
			};

			if chosen.is_some() {
				fname = chosen.unwrap();
				break;
			} else {
				tfd::message_box_ok(
					APP_NAME_STR,
					"Invalid File! Please specify a file to open.",
					tfd::MessageBoxIcon::Error
				);
			};
		}

		if self.outfile.is_none() || force_overwrite || tfd::message_box_yes_no(
			APP_NAME_STR,
			format!("Replace the current working path with {}?", &fname[..]).as_str(),
			tfd::MessageBoxIcon::Question,
			tfd::YesNo::Yes
		) == tfd::YesNo::Yes {
			self.outfile = Some(fname.clone());
		};

		fname
	}
}