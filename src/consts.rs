// Constants for the app

use eframe::epaint::Color32;

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

#[derive(Clone)]
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
