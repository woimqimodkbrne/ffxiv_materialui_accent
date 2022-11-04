use regex::Regex;
use crate as imgui;

pub mod texture;
pub mod file_dialog;
mod scoped;
mod tabbar;
mod divider;
mod orderable_list;
mod drawlist;
mod error;

pub const RED: u32 = 0xFF3030B0;

pub use self::texture::{Texture, TextureOptions};
// pub use self::file_dialog::{FileDialogMode, FileDialogResult, FileDialogStatus, file_dialog, file_picker};
pub use self::file_dialog::{FileDialog, FileDialogResult, file_dialog, file_picker};
pub use self::scoped::*;
pub use self::tabbar::*;
pub use self::divider::*;
pub use self::orderable_list::*;
pub use self::drawlist::*;
pub use self::error::*;

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
pub fn fa5() -> *const imgui::sys::ImFont {
	unsafe{FA5 as *const _}
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

pub fn text_clickable(text: &str) -> bool {
	let draw = imgui::get_window_draw_list();
	let pos = imgui::get_cursor_screen_pos();
	let r = imgui::invisible_button(text, imgui::calc_text_size2(text), imgui::ButtonFlags::None);
	draw.add_text(pos, imgui::get_color(if imgui::is_item_hovered() {imgui::Col::CheckMark} else {imgui::Col::Text}), text);
	r
}

pub fn button_icon(icon: &str) -> bool {
	imgui::push_style_color(imgui::Col::Button, 0);
	imgui::push_font(unsafe{&mut *FA5});
	imgui::push_id(icon);
	let h = frame_height();
	let pos = imgui::get_cursor_screen_pos();
	let r = imgui::button("", [h, h]);
	imgui::get_window_draw_list().add_text(pos.add([h, h].sub(imgui::calc_text_size(&icon[..3], false, 0.0)).div([2.0, 2.0])), imgui::get_color(imgui::Col::Text), &icon[..3]);
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

pub fn selectable_with_icon(icon: &str, text: &str, selected: bool, flags: imgui::SelectableFlags, size: [f32; 2]) -> bool {
	selectable_with_icon_u32(icon, imgui::get_color(imgui::Col::Text), text, selected, flags, size)
}

pub fn selectable_with_icon_u32(icon: &str, icon_clr: u32, text: &str, selected: bool, flags: imgui::SelectableFlags, size: [f32; 2]) -> bool {
	let draw = imgui::get_window_draw_list();
	let icon_w = imgui::get_font_size() * 1.5;
	let pos = imgui::get_cursor_screen_pos();
	imgui::push_id(text);
	let r = imgui::selectable("", selected, flags, size);
	imgui::pop_id();
	
	imgui::push_font(unsafe{&mut *FA5});
	let s = imgui::calc_text_size(icon, false, -1.0).x();
	draw.add_text(pos.add([(icon_w - s) / 2.0, 0.0]), icon_clr, icon);
	imgui::pop_font();
	
	draw.add_text(pos.add([icon_w, 0.0]), imgui::get_color(imgui::Col::Text), text);
	
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

// TODO: end with ... if cut short (use Cow as return)
pub(crate) fn wrap_text_area<'a>(text: &'a str, area: [f32; 2]) -> Vec<(&'a str, f32)> {
	lazy_static::lazy_static! {static ref WORD: Regex = Regex::new(r"\b\w+[[:punct:]]*").unwrap();}
	
	let mut lines = Vec::new();
	let linecount = (area.y() / imgui::get_font_size()).floor() as i32;
	let mut curline = 0;
	
	for test_seg in text.split('\n') {
		let mut lineindex = 0;
		let mut previndex = 0;
		let mut prevlen = 0.0;
		
		for cap in WORD.find_iter(test_seg) {
			let line = &test_seg[lineindex..cap.end()];
			let len = imgui::calc_text_size(line, false, -1.0).x();
			if len > area.x() {
				lines.push((&test_seg[lineindex..previndex], prevlen));
				curline += 1;
				lineindex = cap.start();
				previndex = lineindex;
				prevlen = 0.0;
				if curline >= linecount {
					return lines;
				}
			} else {
				previndex = cap.end();
				prevlen = len;
			}
		}
		
		if lineindex != previndex || (lineindex == 0 && previndex == 0) {
			let line = &test_seg[lineindex..];
			lines.push((line, imgui::calc_text_size(line, false, -1.0).x()));
			curline += 1;
			if curline >= linecount {
				return lines;
			}
		}
	}
	
	lines
}

pub enum TextAlign {
	Left,
	Center,
	Right,
}

pub fn wrapped_text(text: &str, mut area: [f32; 2], align: TextAlign) {
	if area.x() <= 0.0 {area[0] = width_left()}
	let lines = wrap_text_area(text, if area.y() <= 0.0 {[area.x(), f32::MAX]} else {area});
	let h = frame_height();
	let clr = imgui::get_color(imgui::Col::Text);
	let pos = imgui::get_cursor_screen_pos();
	let draw = imgui::get_window_draw_list();
	
	imgui::dummy(if area.y() <= 0.0 {[area.x(), lines.len() as f32 * h]} else {area});
	
	match align {
		TextAlign::Left => {
			for (i, (line, _len)) in lines.into_iter().enumerate() {
				draw.add_text(pos.add([0.0, i as f32 * h]), clr, line);
			}
		},
		TextAlign::Center => {
			for (i, (line, len)) in lines.into_iter().enumerate() {
				draw.add_text(pos.add([(area.x() - len) / 2.0, i as f32 * h]), clr, line);
			}
		},
		TextAlign::Right => {
			for (i, (line, len)) in lines.into_iter().enumerate() {
				draw.add_text(pos.add([area.x() - len, i as f32 * h]), clr, line);
			}
		}
	}
}

// macos displays file sizes based on 1000 (KB), while windows and linux uses 1024 (KiB). windows and macos display it as KB, linux as KiB (thunar atleast, terminal stuff only use a single character)
#[cfg(target_os = "macos")]
const SIZE: u64 = 1000;
#[cfg(not(target_os = "macos"))]
const SIZE: u64 = 1024;

#[cfg(target_os = "linux")]
const AFFIXES: [&str; 5] = ["B", "KiB", "MiB", "GiB", "TiB"];
#[cfg(not(target_os = "linux"))]
const AFFIXES: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];

pub fn format_size(bytes: u64) -> String {
	let bytes = bytes as f64;
	for (i, affix) in AFFIXES.iter().enumerate() {
		let unit = (SIZE.pow(i as u32)) as f64;
		let val = bytes / unit;
		if val < SIZE as f64 {
			let val = (val * 10.0).round() / 10.0;
			return format!("{val} {affix}");
		}
	}
	
	format!("{bytes} B")
}

pub fn image(texture_id: usize, size: [f32; 2], u: [f32; 2], v: [f32; 2], clr: u32) {
	let mut draw = imgui::get_window_draw_list();
	let pos = imgui::get_cursor_screen_pos();
	imgui::dummy(size);
	draw.push_texture_id(texture_id);
	draw.add_rect_rounded(pos, pos.add(size), u, v, clr, imgui::get_style().frame_rounding, imgui::DrawFlags::RoundCornersAll);
	draw.pop_texture_id();
}

pub fn frame_sized<F>(scope: F) where F: FnOnce() {
	let draw = imgui::get_window_draw_list();
	draw.channels_split(2);
	draw.channels_set_current(1);
	imgui::begin_group();
	let style = imgui::get_style();
	offset([0.0, style.frame_padding.y()]);
	imgui::indent_f32(style.frame_padding.x());
	scope();
	imgui::unindent_f32(style.frame_padding.x());
	imgui::end_group();
	draw.channels_set_current(0);
	draw.add_rect_filled(imgui::get_item_rect_min(), imgui::get_item_rect_max().add(style.frame_padding), imgui::get_color(imgui::Col::FrameBg), style.frame_rounding, imgui::DrawFlags::None);
	draw.channels_merge();
}