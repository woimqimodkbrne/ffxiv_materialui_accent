pub(crate) mod texture;
mod scoped;
mod tabbar;
mod divider;

pub use self::texture::{Texture, TextureOptions};
pub use self::scoped::*;
pub use self::tabbar::*;
pub use self::divider::*;

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

use super::imgui;

pub fn frame_height() -> f32 {
	imgui::get_style().frame_padding[1] * 2.0 + imgui::get_font_size()
}

pub fn width_left() -> f32 {
	imgui::get_column_width(-1)
}

pub fn offset(xy: [f32; 2]) {
	imgui::set_cursor_pos(imgui::get_cursor_pos().add(xy));
}