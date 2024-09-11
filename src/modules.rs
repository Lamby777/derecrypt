/*
** I still have no idea what I'm doing.
**
** - RC 		11/8/2022
*/

pub trait DcMod {
    fn run(&mut self, input: &str) -> String;
}

/// Operations used in multiple modules
pub mod common_ops {
    pub fn deflate(s: &mut String) {
        s.retain(|c| !c.is_whitespace());
    }
}

pub mod win_s {
    use super::{common_ops, DcMod};
    use crate::consts::*;
    use crate::tfd;
    use fancy_regex::Regex;
    use std::collections::VecDeque;

    #[derive(Clone, Default)]
    pub struct Caster {
        pub name: String,
        pub list: VecDeque<Box<dyn DcMod>>,
    }

    impl DcMod for Caster {
        fn run(&mut self, input: &str) -> String {
            let mut output = input.to_owned();
            for cast in self.list.iter_mut() {
                output = cast.run(&output);
            }

            output
        }
    }

    #[derive(Clone, Default)]
    pub struct Replace {
        pub from: String,
        pub to: String,
        pub regex: bool,
    }

    impl DcMod for Replace {
        fn run(&mut self, input: &str) -> String {
            if !self.regex {
                // Replace strings
                common_ops::replace(input, &self.from, &self.to);
            } else {
                // Replace strings via RegEx
                let regex = Regex::new(self.from.as_str()).unwrap();
                *input = regex.replace_all(input, self.to.as_str()).to_string();
            }
        }
    }

    #[derive(Clone, Default)]
    pub struct ConvertBase {
        pub from: u32,
    }

    impl DcMod for ConvertBase {
        fn run(&mut self, input: &mut String) {
            // If "from" not in range, set to binary
            if !(2..=36).contains(&self.from) {
                self.from = 2;
            }

            // Deflate accidental whitespace
            common_ops::deflate(input);

            let res = u128::from_str_radix(input, self.from);

            match res {
                Ok(v) => {
                    *input = v.to_string();
                }

                Err(v) => match v.kind() {
                    core::num::IntErrorKind::PosOverflow => {
                        tfd::message_box_ok(
                            APP_NAME_STR,
                            "Attempting to calculate this caused \
							a positive integer overflow.",
                            tfd::MessageBoxIcon::Error,
                        );
                    }

                    _ => {
                        tfd::message_box_ok(
                            APP_NAME_STR,
                            "Number is invalid for this base!\n\
							Did you mean to use the ASCII module?",
                            tfd::MessageBoxIcon::Error,
                        );
                    }
                },
            }
        }
    }

    #[derive(Clone, Default)]
    pub struct FromEscapedASCII {
        pub sep: String,
    }

    impl DcMod for FromEscapedASCII {
        fn run(&mut self, input: &mut String) {
            let rsep = if !self.sep.is_empty() {
                self.sep.as_str()
            } else {
                // If no separator is specified, assume there is nothing
                // between each escape sequence, so replace each "\"
                // with " \" and use " " as the separator.

                common_ops::replace(input, "\\", " \\");

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
                let slice = &b[2..]; // "0000"
                let mode = b.chars().nth(1).unwrap(); // 'u'

                let charcode = u32::from_str_radix(slice, match mode {
                    'x' | 'u' => 16,
                    '0' => 8,

                    // non-standard, but might
                    // be useful for some people
                    'd' => 10,

                    _ => {
                        res = format!("{res}{b}");
                        continue;
                    }
                });

                if charcode.is_err() {
                    res = format!("{res}{b}");
                    continue;
                }

                let nchar = char::from_u32(charcode.unwrap()).unwrap_or('?');

                res.push(nchar);
            }

            *input = res;
        }
    }

    #[derive(Clone)]
    pub struct FromASCII {
        pub sep: String,
        pub mode: ASCIIBases,
    }

    impl Default for FromASCII {
        fn default() -> Self {
            FromASCII {
                sep: String::from(" "),
                mode: ASCIIBases::Hexadecimal,
            }
        }
    }

    impl DcMod for FromASCII {
        fn run(&mut self, input: &mut String) {
            // Number of digits in the current specified base (ex. Hexadecimal = 16)
            let base_numct: u8 = self.mode.into();

            // Length of byte representations for the current mode
            // (ex. this will be 2 in hex mode, 8 in binary mode, etc.)
            let byte_repr_len = 256u16.ilog(base_numct as u16) as usize;
            let total_chunks = input.len().div_ceil(byte_repr_len);

            // Split string by delim
            let mut bytes: Vec<&str> = vec![];
            for i in 0..total_chunks {
                let offset = i * (byte_repr_len);

                let res = &input[offset..(offset + (byte_repr_len))];
                bytes.push(res);
            }

            let mut res = String::new();

            // For each piece, either decode it, OR if it's not
            // encoded, keep it the same.
            for slice in bytes {
                let charcode = u32::from_str_radix(
                    slice,
                    Into::<u8>::into(self.mode) as u32,
                );

                //dbg!(&charcode);

                if charcode.is_err() {
                    res = format!("{res}{slice}");
                    continue;
                }

                let nchar = char::from_u32(charcode.unwrap()).unwrap_or('?');

                res.push(nchar);
            }

            *input = res;
        }
    }

    #[derive(Clone, Default)]
    pub struct Deflate;
    impl DcMod for Deflate {
        fn run(&mut self, input: &mut String) {
            common_ops::deflate(input);
        }
    }

    #[derive(Clone, Default)]
    pub struct Strip;
    impl DcMod for Strip {
        fn run(&mut self, input: &mut String) {
            *input = input.trim().to_string();
        }
    }

    #[derive(Clone, Default)]
    pub struct Length;
    impl DcMod for Length {
        fn run(&mut self, input: &mut String) {
            *input = input.len().to_string();
        }
    }

    #[derive(Clone, Default)]
    pub struct ModContainer;
    impl DcMod for ModContainer {
        fn run(&mut self, _input: &mut String) {
            panic!("This module does not run!");
        }
    }
}
