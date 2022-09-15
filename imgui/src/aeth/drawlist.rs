use crate as imgui;
use super::F2;

pub trait DrawList {
	fn add_rect_rounded(&mut self, p1: [f32; 2], p2: [f32; 2], u: [f32; 2], v: [f32; 2], clr: u32, rounding: f32);
	fn add_text_area(&self, pos: [f32; 2], col: u32, text: &str, area: [f32; 2]);
}

impl DrawList for imgui::DrawList {
	// this only exists because draw.add_image_rounded is broken
	fn add_rect_rounded(&mut self, p1: [f32; 2], p2: [f32; 2], u: [f32; 2], v: [f32; 2], clr: u32, rounding: f32) {
		let (urounding, vrounding) = (rounding / (p2.x() - p1.x()), rounding / (p2.y() - p1.y()));
		self.prim_reserve((24 - 2) * 3, 24);
		for i in 0..6 {
			let r = i as f32 / 5.0 * std::f32::consts::PI / 2.0;
			self.prim_write_vtx(
				[p1.x() + rounding - f32::sin(r) * rounding, p1.y() + rounding - f32::cos(r) * rounding],
				[u.x() + urounding - f32::sin(r) * urounding, u.y() + urounding - f32::cos(r) * urounding],
				clr,
			)
		}
		
		for i in 5..11 {
			let r = i as f32 / 5.0 * std::f32::consts::PI / 2.0;
			self.prim_write_vtx(
				[p1.x() + rounding - f32::sin(r) * rounding, p2.y() - rounding - f32::cos(r) * rounding],
				[u.x() + urounding - f32::sin(r) * urounding, v.y() - vrounding - f32::cos(r) * vrounding],
				clr,
			)
		}
		
		for i in 10..16 {
			let r = i as f32 / 5.0 * std::f32::consts::PI / 2.0;
			self.prim_write_vtx(
				[p2.x() - rounding - f32::sin(r) * rounding, p2.y() - rounding - f32::cos(r) * rounding],
				[v.x() - vrounding - f32::sin(r) * vrounding, v.y() - vrounding - f32::cos(r) * vrounding],
				clr,
			)
		}
		
		for i in 15..21 {
			let r = i as f32 / 5.0 * std::f32::consts::PI / 2.0;
			self.prim_write_vtx(
				[p2.x() - rounding - f32::sin(r) * rounding, p1.y() + rounding - f32::cos(r) * rounding],
				[v.x() - vrounding - f32::sin(r) * vrounding, u.y() + urounding - f32::cos(r) * urounding],
				clr,
			)
		}
		
		for i in 2..24 {
			self.prim_write_idx(0);
			self.prim_write_idx(i - 1);
			self.prim_write_idx(i);
		}
	}
	
	fn add_text_area(&self, pos: [f32; 2], col: u32, text: &str, area: [f32; 2]) {
		let height = imgui::get_font_size();
		for (i, line) in super::wrap_text_area(text, area).into_iter().enumerate() {
			self.add_text(pos.add([0.0, height * i as f32]), col, line);
		}
	}
}