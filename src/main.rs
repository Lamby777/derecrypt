/*
** I only somewhat know what I'm doing.
** - Dex		10/25/2022
*/

// Hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs, path::Path, collections::HashMap};
use tinyfiledialogs as tfd;
use eframe::{egui::{*, style::Widgets}};
use strum::IntoEnumIterator; // 0.17.1
use strum_macros::EnumIter; // 0.17.1

const TITLEBAR_HEIGHT: f32 = 24.0;

struct ThemeColors;

impl ThemeColors {
	const BG_PURPLE:		Color32	= Color32::from_rgb(79, 0, 148);
	const BG_PURPLE_DEEP:	Color32	= Color32::from_rgb(42, 0, 79);
	const BG_PURPLE_LIGHT:	Color32	= Color32::from_rgb(142, 24, 240);
	const BG_PURPLE_DARK:	Color32	= Color32::from_rgb(18, 0, 33);
	const TEXT:				Color32 = Color32::WHITE;
}

fn main() {
	let options = eframe::NativeOptions {
		always_on_top:	true,
		decorated:		false,
		transparent:	true,
		vsync:			true,
		..Default::default()
	};

	eframe::run_native(
		"StringSuite Editor",
		options,
		Box::new(|_cc| Box::new(MyApp::new())),
	);
}

#[derive(Eq, Hash, PartialEq, Copy, Clone, EnumIter)]
enum WindowTypes {
	ConvertBase,
}

struct MyApp {
	open_modals:	HashMap<WindowTypes, bool>,
	outfile:		Option<String>,
	string:			String,
	args:			Vec<String>
}

impl MyApp {
	pub fn new() -> Self {
		let mut modals_map = HashMap::new();
		
		for wintype in WindowTypes::iter() {
			modals_map.insert(wintype, false);
		}

		MyApp {
			open_modals:	modals_map,
			outfile:		None,
			string:			String::new(),
			args:			vec![String::new()],
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

	// Pops up a dialog to open a new file, and then asks
	// if the selected path should be the new output path
	fn get_desired_path(&mut self, save: bool, force_overwrite: bool) -> String {
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
					"StringSuite",
					"Invalid File! Please specify a file to open.",
					tfd::MessageBoxIcon::Error
				);
			};
		}

		if self.outfile.is_none() || force_overwrite || tfd::message_box_yes_no(
			"StringSuite",
			format!("Replace the current working path with {}?", &fname[..]).as_str(),
			tfd::MessageBoxIcon::Question,
			tfd::YesNo::Yes
		) == tfd::YesNo::Yes {
			self.outfile = Some(fname.clone());
		};

		fname
	}
}

impl eframe::App for MyApp {
	fn clear_color(&self, _visuals: &Visuals) -> Rgba {
		Rgba::TRANSPARENT // Make sure we don't paint anything behind the rounded corners
	}

	fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
		let titlebar_text = if let Some(v) = self.filename() {
			format!("StringSuite: {}", v)
		} else {
			String::from("StringSuite")
		};

		let visuals = Visuals {
			resize_corner_size:		4.0,
			hyperlink_color:		ThemeColors::BG_PURPLE_LIGHT,
			faint_bg_color:			ThemeColors::BG_PURPLE_LIGHT,
			extreme_bg_color:		ThemeColors::BG_PURPLE_DARK,

			widgets: Widgets {
				noninteractive: {
					style::WidgetVisuals {
						bg_fill:	ThemeColors::BG_PURPLE_DEEP,
						bg_stroke:	Stroke::new(2.0, ThemeColors::BG_PURPLE),
						fg_stroke:	Stroke::new(1.0, ThemeColors::TEXT),
						rounding:	Rounding::same(4.0),
						expansion:	0.0,
					}
				},

				..Widgets::dark()
			},
		
			..Visuals::dark()
		};
		
		ctx.set_visuals(visuals);

		custom_window_frame(ctx, frame, titlebar_text.as_str(), |ui| {
			if *self.open_modals.get(&WindowTypes::ConvertBase).unwrap() {
				Window::new("Convert Base")
					.show(ctx, |ui| {
						ui.label("contents");
					});
			}
			
			ui.heading("Arguments");

			for arg_i in 0..self.args.len() {
				ui.add(TextEdit::singleline(&mut self.args[arg_i]));
			}

			ui.horizontal(|ui| {
				if ui.button("Strip").clicked() {
					self.string = self.string.trim().to_string();
				}

				if ui.button("Deflate").clicked() {
					self.string.retain(|c| !c.is_whitespace());
				}

				if ui.button("Conv Base").clicked() {
					// Toggle whether or not the array contains
					let val_o = *self.open_modals.get(&WindowTypes::ConvertBase).unwrap();

					self.open_modals.insert(WindowTypes::ConvertBase, !val_o);
				}
			});

			ui.with_layout(Layout::centered_and_justified(Direction::TopDown),
			|ui| {

				// Render this stuff in the center
				let writebox = TextEdit::multiline(&mut self.string)
					.font(TextStyle::Monospace) // for cursor height
					.code_editor()
					.lock_focus(true)
					.desired_width(f32::INFINITY);
				
				ui.add(writebox);
			});

			// If CTRL O
			if ui.input_mut().consume_key(Modifiers::COMMAND, Key::O) {
				// Open a text file
				let fname = self.get_desired_path(false, false);

				let fcontent = fs::read_to_string(&fname).expect("Failed to read file");

				self.string = fcontent;
			}

			// If managing files
			let ctrl_n =
				ui.input_mut().consume_key(Modifiers::COMMAND, Key::N);
			let ctrl_s =
				ui.input_mut().consume_key(Modifiers::COMMAND, Key::S);
			let ctrl_shift_s =
				ui.input_mut().consume_key(Modifiers::COMMAND | Modifiers::SHIFT, Key::S);

			if ctrl_n || ctrl_s || ctrl_shift_s {
				// If "Save As" OR output path not yet specified
				if ctrl_shift_s || ctrl_n || self.outfile.is_none() {
					// Ask where to save to
					self.get_desired_path(true, ctrl_n);
				}

				// Save to output file, unless using CTRL N to only edit path
				if ctrl_s || ctrl_shift_s {
					fs::write(self.outfile.as_ref().unwrap(), &self.string)
						.expect("Failed to write file");
				}
			}
		});
	}
}

fn custom_window_frame(
	ctx: &Context,
	frame: &mut eframe::Frame,
	title: &str,
	add_contents: impl FnOnce(&mut Ui),
) {
	let text_color		= ctx.style().visuals.text_color();
	let window_stroke	= ctx.style().visuals.window_stroke();
	let window_fill		= ctx.style().visuals.window_fill();

	CentralPanel::default()
		.frame(Frame::none())
		.show(ctx, |ui| {
			let rect = ui.max_rect();
			let painter = ui.painter();

			// Paint the frame:
			painter.rect(
				rect.shrink(1.0),
				10.0,
				window_fill,
				window_stroke,
			);

			// Paint the title:
			painter.text(
				rect.center_top() + vec2(0.0, TITLEBAR_HEIGHT / 2.0),
				Align2::CENTER_CENTER,
				title,
				FontId::proportional(TITLEBAR_HEIGHT * 0.8),
				text_color,
			);

			// Paint the line under the title:
			painter.line_segment(
				[
					rect.left_top() + vec2(2.0, TITLEBAR_HEIGHT),
					rect.right_top() + vec2(-2.0, TITLEBAR_HEIGHT),
				],
				window_stroke,
			);

			// Add the close button:
			let close_button = ui.put(
				Rect::from_min_size(
					rect.right_top() - vec2(32.0, 0.0),
					Vec2::splat(TITLEBAR_HEIGHT)
				),
				Button::new(RichText::new("❌").size(TITLEBAR_HEIGHT - 4.0)).frame(false),
			);

			if close_button.clicked() {
				frame.close();
			}

			// Interact with the title bar (drag to move window):
			let title_bar_rect = {
				let mut rect = rect;
				rect.max.y = rect.min.y + TITLEBAR_HEIGHT;
				rect
			};
			let title_bar_response =
				ui.interact(title_bar_rect, Id::new("title_bar"), Sense::click());
			if title_bar_response.is_pointer_button_down_on() {
				frame.drag_window();
			}

			// Add the contents:
			let content_rect = {
				let mut rect = rect;
				rect.min.y = title_bar_rect.max.y;
				rect
			}
			.shrink(4.0);
			let mut content_ui = ui.child_ui(content_rect, *ui.layout());
			add_contents(&mut content_ui);
		});
}
