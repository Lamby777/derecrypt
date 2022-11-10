/*
** I only somewhat know what I'm doing.
** - Dex		10/25/2022
*/

// Hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use eframe::{egui::{*, style::Widgets}};
use strum::IntoEnumIterator;
use tinyfiledialogs as tfd;

// OOP definitions and constants
mod consts;
use consts::*;

mod classes;
use classes::*;

mod mods;
use mods::*;


fn main() {
	let options = eframe::NativeOptions {
		always_on_top:	true,
		decorated:		false,
		transparent:	true,
		vsync:			true,
		..Default::default()
	};

	eframe::run_native(
		format!("{} Editor", APP_NAME_STR).as_str(),
		options,
		Box::new(|_cc| Box::new(Derecrypt::new())),
	);
}

impl eframe::App for Derecrypt {
	fn clear_color(&self, _visuals: &Visuals) -> Rgba {
		Rgba::TRANSPARENT
	}

	fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
		let titlebar_text = if let Some(v) = self.filename() {
			format!("{}: {}", APP_NAME_STR, v)
		} else {
			format!("{} v{}", APP_NAME_STR, DC_VERSION)
		};
		
		ctx.set_visuals(Visuals {
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
		});

		custom_window_frame(ctx, frame, titlebar_text.as_str(), |ui| {
			for i_disc in WindowDiscriminants::iter() {
				let dcmod: &mut DcModBase = &mut self.open_modals.get_mut(&i_disc).unwrap();
				let params = &mut dcmod.params;

				if !(dcmod.active) {continue};

				match params {
					WindowTypes::Caster(o) => {

						Window::new("String Casting")
							.show(ctx, |ui| {
								// Draw current caster info
								let cname =
									if o.name.len() > 0 {
										format!("Cast \"{}\"", o.name)
									} else {
										String::from("New Cast")
									};
								
								let title = format!("{} ({} elements)", cname, o.list.len());
								ui.heading(title);

								//
						});
					},

					WindowTypes::ModContainer	=> {
						Window::new("The Toolbox")
							.show(ctx, |ui| {
								self.popout_button(ui, "Length",
									WindowDiscriminants::Length);

								self.popout_button(ui, "Conv Base",
									WindowDiscriminants::ConvertBase);

								self.popout_button(ui, "From ASCII",
									WindowDiscriminants::FromASCII);
						});
					},

					WindowTypes::Deflate(ref mut args) => {
						args.run(&mut self.string);
						dcmod.active = false;
					},

					WindowTypes::Strip(ref mut args) => {
						args.run(&mut self.string);
						dcmod.active = false;
					},

					WindowTypes::Length(ref mut args) => {
						args.run(&mut self.string);
						dcmod.active = false;
					},

					WindowTypes::FromASCII(ref mut args) => {
						Window::new("ASCII Sequence -> Plaintext")
							.show(ctx, |ui| {

								ui.add(
									TextEdit::singleline(&mut args.sep)
										.hint_text("String Delimiter")
								);

								if dcm_run(ui).0 {
									args.run(&mut self.string);
								}
							});
					},

					WindowTypes::ConvertBase(ref mut args) => {
						Window::new("Convert Base")
							.show(ctx, |ui| {
								ui.add(
									Slider::new(&mut args.from, 2..=36)
										.prefix("From ")
								);

								// Run module
								if dcm_run(ui).0 {
									args.run(&mut self.string)
								}
							});
					},



					WindowTypes::Replace(args)	=> {
						Window::new("Replace / Remove")
							.show(ctx, |ui| {
								ui.add(
									TextEdit::singleline(&mut args.from)
										.hint_text("Replace This...")
								);

								ui.add(
									TextEdit::singleline(&mut args.to)
										.hint_text("With This!")
								);

								ui.checkbox(&mut args.regex, "Match via RegEx");
						
								if dcm_run(ui).0 {
									args.run(&mut self.string);
								}
							});
					},
				}
			}

			ui.horizontal(|ui| {
				self.popout_button(ui, "TOOLBOX",
					WindowDiscriminants::ModContainer);

				self.popout_button(ui, "CASTER",
					WindowDiscriminants::Caster);

				self.popout_button(ui, "Strip",
					WindowDiscriminants::Strip);

				self.popout_button(ui, "Deflate",
					WindowDiscriminants::Deflate);

				self.popout_button(ui, "Replace",
					WindowDiscriminants::Replace);
			});

			ui.with_layout(Layout::centered_and_justified(Direction::TopDown),
			|ui| {
				ScrollArea::vertical().always_show_scroll(true).show(ui, |ui| {
					let writebox = TextEdit::multiline(&mut self.string)
						.font(TextStyle::Monospace)
						.code_editor()
						.lock_focus(true)
						.desired_width(f32::INFINITY);
					
					ui.add(writebox);
				});
			});

			// If CTRL O
			if ui.input_mut().consume_key(Modifiers::COMMAND, Key::O) {
				// Open a text file
				let fname_o = self.get_desired_path(false, false);

				if let Some(fname) = fname_o {
					let fcontent = fs::read_to_string(&fname);
					
					self.string =
						if fcontent.is_ok() {
							fcontent.unwrap()
						} else {
							tfd::message_box_ok(
								APP_NAME_STR,
								"Importing as binary (could not load as string)",
								tfd::MessageBoxIcon::Info
							);
							
							let bytes: Vec<u8>	= fs::read(&fname).unwrap();
							let mut res: String	= String::new();
							
							for b in bytes {
								res = format!("{}{:08b}", res, b);
							}

							res
						};
				}
			}




			// If managing files
			let ctrl_shift_s =
				ui.input_mut().consume_key(Modifiers::COMMAND | Modifiers::SHIFT, Key::S);

			let ctrl_n =
				ui.input_mut().consume_key(Modifiers::COMMAND, Key::N);

			let is_setting_path =
				ctrl_shift_s || ctrl_n;

			let is_saving =
				ctrl_shift_s ||
				ui.input_mut().consume_key(Modifiers::COMMAND, Key::S);

				
			let must_save_new = is_saving && self.outfile.is_none();

			let mut npath = None;

			if is_setting_path || must_save_new {
				// Ask where to save to
				npath = self.get_desired_path(true, ctrl_n);
			}

			if is_saving {
				if must_save_new && npath.is_none() {
					// User attempted to write to nothing
					tfd::message_box_ok(
						APP_NAME_STR,
						"Your file was not saved because the output path is empty.",
						tfd::MessageBoxIcon::Warning
					);
				} else {
					// Save to output file
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
	let vis				= &ctx.style().visuals;
	let text_color		= vis.text_color();
	let window_stroke	= vis.window_stroke();
	let window_fill		= vis.window_fill();

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
				Button::new(RichText::new("❌")
					.size(TITLEBAR_HEIGHT - 4.0)).frame(false),
			);

			if close_button.clicked() {
				frame.close();
			}

			// Draggable title bar
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

// Create and check for click on a module's main run button
fn dcm_run(ui: &mut Ui) -> (bool, bool) {
	let b = ui.button("Run");
	
	(
		b.clicked(),
		b.secondary_clicked(),
	)
}
