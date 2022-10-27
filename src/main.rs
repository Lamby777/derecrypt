/*
** I only somewhat know what I'm doing.
** - Dex		10/25/2022
*/

// Hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs, path::Path};
use tinyfiledialogs as tfd;
use eframe::egui::*;

const TITLEBAR_HEIGHT: f32 = 24.0;

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

struct MyApp {
	outfile:	Option<String>,
	string:		String,
	args:		Vec<String>
}

impl MyApp {
	pub fn new() -> Self {
		MyApp {
			outfile:	None,
			string:		String::new(),
			args:		vec![String::new()],
		}
	}

	pub fn filename(&self) -> Option<String> {
		let outfile = &self.outfile.as_ref();

		if let None = outfile {
			return None;
		}

		Some(Path::new(outfile.unwrap()).file_name().unwrap().to_str().unwrap().to_string())
	}
}

impl eframe::App for MyApp {
	fn clear_color(&self, _visuals: &Visuals) -> Rgba {
		Rgba::TRANSPARENT // Make sure we don't paint anything behind the rounded corners
	}

	fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
		let titlebar_text = match self.filename() {
			None	=> String::from("StringSuite"),
			_		=> format!("StringSuite: {}", self.filename().unwrap())
		};

		custom_window_frame(ctx, frame, titlebar_text.as_str(), |ui| {
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
			});

			ui.with_layout(Layout::centered_and_justified(Direction::TopDown),
			|ui| {

				// Render this stuff in the center
				let writebox = TextEdit::multiline(&mut self.string)
					.font(TextStyle::Monospace) // for cursor height
					.code_editor()
					.desired_rows(10)
					.lock_focus(true)
					.desired_width(f32::INFINITY);
				
				ui.add(writebox);
			});

			if ui.input_mut().consume_key(Modifiers::COMMAND, Key::O) {
				// Open a text file
				let fname = tfd::open_file_dialog(
					"Load String From File", "", None
				).unwrap();

				let fcontent = fs::read_to_string(&fname).unwrap();

				if tfd::message_box_yes_no(
					"String Loader",
					format!("Replace the current working path with {}?", &fname[..]).as_str(),
					tfd::MessageBoxIcon::Question,
					tfd::YesNo::Yes
				) == tfd::YesNo::Yes {
					self.outfile = Some(fname);
				}

				self.string = fcontent;
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
	let text_color = ctx.style().visuals.text_color();

	CentralPanel::default()
		.frame(Frame::none())
		.show(ctx, |ui| {
			let rect = ui.max_rect();
			let painter = ui.painter();

			// Paint the frame:
			painter.rect(
				rect.shrink(1.0),
				10.0,
				ctx.style().visuals.window_fill(),
				Stroke::new(1.0, text_color),
			);

			// Paint the title:
			painter.text(
				rect.center_top() + vec2(0.0, TITLEBAR_HEIGHT / 2.0),
				Align2::CENTER_CENTER,
				title,
				FontId::proportional(TITLEBAR_HEIGHT * 0.8),
				text_color,
			);

			//widgets::global_dark_light_mode_buttons(ui);

			// Paint the line under the title:
			painter.line_segment(
				[
					rect.left_top() + vec2(2.0, TITLEBAR_HEIGHT),
					rect.right_top() + vec2(-2.0, TITLEBAR_HEIGHT),
				],
				Stroke::new(1.0, text_color),
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
