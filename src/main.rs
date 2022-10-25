/*
** I only somewhat know what I'm doing.
** - Dex		10/25/2022
*/

// Hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;

const TITLEBAR_HEIGHT: f32 = 28.0;

fn main() {
	let options = eframe::NativeOptions {
		// Hide the OS-specific "chrome" around the window:
		decorated: false,
		// To have rounded corners we need transparency:
		transparent: true,
		min_window_size: Some(egui::vec2(320.0, 100.0)),
		..Default::default()
	};
	eframe::run_native(
		"Custom window frame", // unused title
		options,
		Box::new(|_cc| Box::new(MyApp::default())),
	);
}

#[derive(Default)]
struct MyApp {
	file:	String,
}

impl eframe::App for MyApp {
	fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
		egui::Rgba::TRANSPARENT // Make sure we don't paint anything behind the rounded corners
	}

	fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
		custom_window_frame(ctx, frame, "StringSuite", |ui| {
			ui.label("Test");
			ui.horizontal(|ui| {
				
			});

			ui.heading(format!("Editing File \"{}\"", self.file));
			
			ui.horizontal(|ui| {
				ui.label("Your name: ");
				ui.text_edit_singleline(&mut self.file);
			});
		});
	}
}

fn custom_window_frame(
	ctx: &egui::Context,
	frame: &mut eframe::Frame,
	title: &str,
	add_contents: impl FnOnce(&mut egui::Ui),
) {
	use egui::*;
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

			//egui::widgets::global_dark_light_mode_buttons(ui);

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
