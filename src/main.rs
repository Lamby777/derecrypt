/*
** I still have no idea what I'm doing.
**
** - RC 		9/11/2024
*/

#![feature(int_roundings)]
#![feature(slice_as_chunks)]

mod consts;
mod modules;

fn main() {}

use std::sync::{Arc, Mutex};

/// Program state
pub struct Derecrypt {
    pub open_modals: Vec<()>, // TODO

    pub outfile: Option<String>,
    pub string: Arc<Mutex<String>>,
}

impl Derecrypt {
    pub fn new() -> Self {
        Derecrypt {
            open_modals: vec![],
            outfile: None,
            string: Arc::new(Mutex::new(String::new())),
        }
    }
}
