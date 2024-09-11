//! Constants for the app

use std::fmt::Display;

use strum_macros::EnumIter;

pub const APP_ID: &str = "org.sparklet.derecrypt";
pub const DC_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Clone, Copy, Default, PartialEq, EnumIter)]
pub enum ASCIIBases {
    Binary,
    Octal,
    Decimal,

    #[default]
    Hexadecimal,
}

impl Display for ASCIIBases {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ASCIIBases::Binary => write!(f, "Binary"),
            ASCIIBases::Octal => write!(f, "Octal"),
            ASCIIBases::Decimal => write!(f, "Decimal"),
            ASCIIBases::Hexadecimal => write!(f, "Hexadecimal"),
        }
    }
}

impl From<ASCIIBases> for u8 {
    fn from(value: ASCIIBases) -> Self {
        match value {
            ASCIIBases::Binary => 2,
            ASCIIBases::Octal => 8,
            ASCIIBases::Decimal => 10,
            ASCIIBases::Hexadecimal => 16,
        }
    }
}
