// Constants for the app

use std::fmt::Display;

use eframe::epaint::Color32;
use strum_macros::EnumIter;

pub	const TITLEBAR_HEIGHT:	f32		= 24.0;
pub	const APP_NAME_STR:		&str	= "Derecrypt";
pub	const DC_VERSION:		&str	= env!("CARGO_PKG_VERSION");

pub struct ThemeColors;

impl ThemeColors {
	pub const BG_PURPLE:		Color32	= Color32::from_rgb(79,	0,	148);
	pub const BG_PURPLE_DEEP:	Color32	= Color32::from_rgb(42,	0,	79);
	pub const BG_PURPLE_LIGHT:	Color32	= Color32::from_rgb(142,	24,	240);
	pub const BG_PURPLE_DARK:	Color32	= Color32::from_rgb(18,	0,	33);
	pub const TEXT:				Color32 = Color32::WHITE;
}

#[derive(Clone, PartialEq, EnumIter)]
pub enum ASCIIBases {
	Binary,
	Octal,
	Decimal,
	Hexadecimal,
}

impl Default for ASCIIBases {
    fn default() -> Self {
		ASCIIBases::Hexadecimal
    }
}

impl Display for ASCIIBases {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			ASCIIBases::Binary		=> write!(f, "Binary"		),
			ASCIIBases::Octal		=> write!(f, "Octal"		),
			ASCIIBases::Decimal		=> write!(f, "Decimal"		),
			ASCIIBases::Hexadecimal	=> write!(f, "Hexadecimal"	),
		}
	}
}

impl From<ASCIIBases> for u8 {
	fn from(value: ASCIIBases) -> Self {
		match value {
			ASCIIBases::Binary		=> 2,
			ASCIIBases::Octal		=> 8,
			ASCIIBases::Decimal		=> 10,
			ASCIIBases::Hexadecimal	=> 16,
		}
	}
}
