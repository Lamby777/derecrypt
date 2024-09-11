/*
** I only somewhat know what I'm doing.
** - Dex		10/25/2022
*/

// Hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(int_roundings)]
#![feature(slice_as_chunks)]

use tinyfiledialogs as tfd;

// OOP definitions and constants
mod classes;
mod consts;
mod modules;

fn main() {}
