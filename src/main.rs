/*
** I only somewhat know what I'm doing.
** - Dex		10/25/2022
*/

// Hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{mem, fs};
use eframe::{egui::{*, style::Widgets}};
use tinyfiledialogs as tfd;

// OOP definitions and constants
mod consts;
use consts::*;

mod classes;
use classes::*;


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

// code that has its own module, but might be reused in other modules
mod dcmod_scripts {
	pub fn deflate(s: &mut String) {
		s.retain(|c| !c.is_whitespace());
	}

	pub fn replace(s: &mut String, old: &str, new: &str) {
		*s = s.as_str().replace(old, new);
	}
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
			for pair in &mut self.open_modals.iter_mut() {
				let dcmod: &mut DcModBase = pair.1;
				let params = &mut dcmod.params;

				if !(dcmod.active) {continue};

				match params {
					WindowTypes::ModContainer	=> {/*
						Window::new("The Toolbox")
							.show(ctx, |ui| {

								if ui.button("Length").clicked() {
									self.string = self.string.len().to_string();
								}

								if ui.button("Conv Base").clicked() {
									self.toggle_module_visibility(
										mem::discriminant(
											&WindowTypes::ConvertBase {from: 0}
										)
									);
								}
						});*/
					},

					WindowTypes::ConvertBase	{
						ref	mut	from,
					}	=> {
						Window::new("Convert Base")
							.show(ctx, |ui| {
								ui.add(
									Slider::new(from, 2..=36)
										.prefix("From ")
								);

								// Run module
								if dcm_run(ui) {
									// If "from" not in range, set to binary
									if !(2..=36).contains(from) {
										*from = 2;
									}

									// Deflate accidental whitespace
									dcmod_scripts::deflate(&mut self.string);

									let res = u128::from_str_radix(
										&self.string, *from
									);

									match res {
										Ok(v) => {
											self.string = v.to_string();
										},

										Err(v) => match v.kind() {
											core::num::IntErrorKind::PosOverflow => {
												tfd::message_box_ok(
													APP_NAME_STR,
													"Attempting to calculate this caused \
													a positive integer overflow.",
													tfd::MessageBoxIcon::Error
												);
											},

											_ => {
												tfd::message_box_ok(
													APP_NAME_STR,
													"Number is invalid for this base!\n\
													Did you mean to use the ASCII module?",
													tfd::MessageBoxIcon::Error
												);
											}
										}
									}
								}
							});
					},



					WindowTypes::Replace		{
						ref	mut	from,
						ref	mut	to,
						ref	mut	regex
					}	=> {
						
						Window::new("Replace / Remove")
							.show(ctx, |ui| {
								ui.add(
									TextEdit::singleline(from)
										.hint_text("Replace This...")
								);

								ui.add(
									TextEdit::singleline(to)
										.hint_text("With This!")
								);

								ui.checkbox(regex, "Match via RegEx");
						
								if dcm_run(ui) {
									dcmod_scripts::replace(
										&mut self.string,
										from.as_str(),
										to.as_str()
									);
								}
							});
					},


				}
			}

			ui.horizontal(|ui| {
				if ui.button("TOOLBOX").clicked() {
					self.toggle_module_visibility(
						mem::discriminant(
							&WindowTypes::ModContainer
						)
					);
				}

				if ui.button("Strip").clicked() {
					self.string = self.string.trim().to_string();
				}

				if ui.button("Deflate").clicked() {
					dcmod_scripts::deflate(&mut self.string);
				}

				if ui.button("Replace").clicked() {
					self.toggle_module_visibility(
						mem::discriminant(
							&WindowTypes::Replace {from: String::from(""), to: String::from(""), regex: false}
						)
					);
				}
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
				let fname = self.get_desired_path(false, false);
				
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
fn dcm_run (ui: &mut Ui) -> bool {
	ui.button("Run").clicked()
}
