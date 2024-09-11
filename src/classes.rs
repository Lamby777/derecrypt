use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};
use strum::IntoEnumIterator;
use tinyfiledialogs as tfd;

use super::consts::*;
use super::modules::*;

pub struct DcModBase {
    pub active: bool,
    pub params_state: WindowTypes,
}

pub struct Derecrypt {
    pub open_modals: HashMap<WindowDiscriminants, DcModBase>,

    pub outfile: Option<String>,
    pub string: Arc<Mutex<String>>,
}

impl Derecrypt {
    pub fn new() -> Self {
        let mut modals: HashMap<WindowDiscriminants, DcModBase> =
            HashMap::new();

        // initialize all the modules
        for wintype in WindowTypes::iter() {
            let disc = wintype.clone().into();
            modals.insert(disc, DcModBase {
                active: false,
                params_state: wintype,
            });
        }

        Derecrypt {
            open_modals: modals,
            outfile: None,
            string: Arc::new(Mutex::new(String::new())),
        }
    }

    pub fn filename(&self) -> Option<String> {
        let outfile = &self.outfile.as_ref();

        outfile.map(|v| {
            Path::new(v)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
        })
    }

    pub fn get_window_from_disc(
        &mut self,
        winid: WindowDiscriminants,
    ) -> Entry<WindowDiscriminants, DcModBase> {
        self.open_modals.entry(winid)
    }

    pub fn toggle_module_visibility(&mut self, winid: WindowDiscriminants) {
        self.get_window_from_disc(winid)
            .and_modify(|v| v.active = !v.active);
    }

    // Pops up a dialog to open a new file, and then asks
    // if the selected path should be the new output path
    pub fn get_desired_path(
        &mut self,
        save: bool,
        force_overwrite: bool,
    ) -> Option<String> {
        let fname;

        loop {
            let chosen = if save {
                tfd::save_file_dialog("Save To File", "")
            } else {
                tfd::open_file_dialog("Load String From File", "", None)
            };

            if chosen.is_some() {
                fname = chosen.unwrap();
                break;
            } else {
                let res = tfd::message_box_ok_cancel(
                    APP_NAME_STR,
                    "No file chosen! Try again?",
                    tfd::MessageBoxIcon::Error,
                    tfd::OkCancel::Ok,
                );

                if res == tfd::OkCancel::Cancel {
                    return None;
                }
            };
        }

        if self.outfile.is_none()
            || force_overwrite
            || tfd::message_box_yes_no(
                APP_NAME_STR,
                format!(
                    "Replace the current working path with {}?",
                    fname.as_str()
                )
                .as_str(),
                tfd::MessageBoxIcon::Question,
                tfd::YesNo::Yes,
            ) == tfd::YesNo::Yes
        {
            self.outfile = Some(fname.clone());
        };

        Some(fname)
    }
}
