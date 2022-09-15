use regex::Regex;
use crate as imgui;

pub mod texture;
pub mod file_dialog;
mod scoped;
mod tabbar;
mod divider;
mod orderable_list;
mod drawlist;

pub use self::texture::{Texture, TextureOptions};
pub use self::file_dialog::{FileDialogMode, FileDialogResult, FileDialogStatus, file_dialog, file_picker};
pub use self::scoped::*;
pub use self::tabbar::*;
pub use self::divider::*;
pub use self::orderable_list::*;
pub use self::drawlist::*;

pub trait F2 {
	fn add(&self, a: [f32; 2]) -> [f32; 2];
	fn sub(&self, a: [f32; 2]) -> [f32; 2];
	fn mul(&self, a: [f32; 2]) -> [f32; 2];
	fn div(&self, a: [f32; 2]) -> [f32; 2];
	fn inv(&self) -> [f32; 2];
	fn x(&self) -> f32;
	fn y(&self) -> f32;
}

impl F2 for [f32; 2] {
	fn add(&self, a: [f32; 2]) -> [f32; 2] {
		[self[0] + a[0], self[1] + a[1]]
	}
	
	fn sub(&self, a: [f32; 2]) -> [f32; 2] {
		[self[0] - a[0], self[1] - a[1]]
	}
	
	fn mul(&self, a: [f32; 2]) -> [f32; 2] {
		[self[0] * a[0], self[1] * a[1]]
	}
	
	fn div(&self, a: [f32; 2]) -> [f32; 2] {
		[self[0] / a[0], self[1] / a[1]]
	}
	
	fn inv(&self) -> [f32; 2] {
		[-self[0], -self[1]]
	}
	
	fn x(&self) -> f32 {
		self[0]
	}
	
	fn y(&self) -> f32 {
		self[1]
	}
}

pub static mut FA5: *mut imgui::sys::ImFont = std::ptr::null_mut::<imgui::sys::ImFont>();
pub fn fa5() -> &'static mut imgui::sys::ImFont {
	unsafe{&mut *FA5}
}

pub fn frame_height() -> f32 {
	imgui::get_style().frame_padding[1] * 2.0 + imgui::get_font_size()
}

pub fn width_left() -> f32 {
	imgui::get_column_width(-1)
}

pub fn offset(xy: [f32; 2]) {
	imgui::set_cursor_pos(imgui::get_cursor_pos().add(xy));
}

pub fn next_max_width() {
	imgui::set_next_item_width(imgui::get_column_width(-1));
}

pub fn button_icon(icon: &str) -> bool {
	imgui::push_style_color(imgui::Col::Button, 0);
	imgui::push_font(unsafe{&mut *FA5});
	imgui::push_id(icon);
	let h = frame_height();
	let pos = imgui::get_cursor_screen_pos();
	let r = imgui::button("", [h, h]);
	imgui::get_window_draw_list().add_text(pos.add([h, h].sub(imgui::calc_text_size(icon, false, 0.0)).div([2.0, 2.0])), imgui::get_color(imgui::Col::Text), icon);
	imgui::pop_id();
	imgui::pop_font();
	imgui::pop_style_color(1);
	
	r
}

pub fn button_icon_state(icon: &str, enabled: bool) -> bool {
	imgui::push_style_color(imgui::Col::Button, 0);
	imgui::push_font(unsafe{&mut *FA5});
	imgui::push_id(icon);
	let h = frame_height();
	let pos = imgui::get_cursor_screen_pos();
	let r = imgui::button("", [h, h]);
	imgui::get_window_draw_list().add_text(pos.add([h, h].sub(imgui::calc_text_size(icon, false, 0.0)).div([2.0, 2.0])), imgui::get_color(if enabled {imgui::Col::Text} else {imgui::Col::TextDisabled}), icon);
	imgui::pop_id();
	imgui::pop_font();
	imgui::pop_style_color(1);
	
	r
}

pub fn icon(icon: &str) {
	imgui::push_font(unsafe{&mut *FA5});
	imgui::text(icon);
	imgui::pop_font();
}

pub fn tooltip(label: &str) {
	if imgui::is_item_hovered() {
		imgui::set_tooltip(label);
	}
}

// TODO: end with ... if cut short
pub(crate) fn wrap_text_area<'a>(text: &'a str, area: [f32; 2]) -> Vec<&'a str> {
	lazy_static::lazy_static! {
		static ref WORD: Regex = Regex::new(r"\b\w+[[:punct:]]*\b").unwrap();
	}
	
	let mut lines = Vec::new();
	let linecount = (area.y() / imgui::get_font_size()).floor() as i32;
	let mut curline = 0;
	let mut lineindex = 0;
	let mut previndex = 0;
	
	for cap in WORD.find_iter(text) {
		let line = &text[lineindex..cap.end()];
		if imgui::calc_text_size(line, false, -1.0).x() > area.x() {
			lines.push(&text[lineindex..previndex]);
			curline += 1;
			lineindex = cap.start();
			previndex = lineindex;
			if curline >= linecount {
				return lines;
			}
		} else {
			previndex = cap.end();
		}
	}
	
	if lineindex != previndex {
		lines.push(&text[lineindex..])
	}
	
	lines
}