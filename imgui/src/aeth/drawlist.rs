use crate as imgui;
use super::F2;

pub trait DrawList {
	fn add_rect_rounded(&mut self, p1: [f32; 2], p2: [f32; 2], u: [f32; 2], v: [f32; 2], clr: u32, rounding: f32, flags: imgui::DrawFlags);
	fn add_text_area(&self, pos: [f32; 2], col: u32, text: &str, area: [f32; 2]);
}

impl DrawList for imgui::DrawList {
	// this only exists because draw.add_image_rounded is broken
	fn add_rect_rounded(&mut self, p1: [f32; 2], p2: [f32; 2], u: [f32; 2], v: [f32; 2], clr: u32, rounding: f32, mut flags: imgui::DrawFlags) {
		if flags == imgui::DrawFlags::None {flags = imgui::DrawFlags::RoundCornersAll}
		let (w, h) = (p2.x() - p1.x(), p2.y() - p1.y());
		let rounding = rounding.clamp(0.0, w.min(h) / 2.0);
		let (urounding, vrounding) = (rounding / w, rounding / h);
		let vertices = if flags.contains(imgui::DrawFlags::RoundCornersTopLeft) {6} else {1} +
		               if flags.contains(imgui::DrawFlags::RoundCornersTopRight) {6} else {1} +
		               if flags.contains(imgui::DrawFlags::RoundCornersBottomLeft) {6} else {1} +
		               if flags.contains(imgui::DrawFlags::RoundCornersBottomRight) {6} else {1u16};
		
		self.prim_reserve((vertices as i32 - 2) * 3, vertices as i32);
		
		if flags.contains(imgui::DrawFlags::RoundCornersTopLeft) {
			for i in 0..6 {
				let r = i as f32 / 5.0 * std::f32::consts::PI / 2.0;
				self.prim_write_vtx(
					[p1.x() + rounding - f32::sin(r) * rounding, p1.y() + rounding - f32::cos(r) * rounding],
					[u.x() + urounding - f32::sin(r) * urounding, u.y() + vrounding - f32::cos(r) * vrounding],
					clr,
				)
			}
		} else {
			self.prim_write_vtx(p1, u, clr);
		}
		
		if flags.contains(imgui::DrawFlags::RoundCornersBottomLeft) {
			for i in 5..11 {
				let r = i as f32 / 5.0 * std::f32::consts::PI / 2.0;
				self.prim_write_vtx(
					[p1.x() + rounding - f32::sin(r) * rounding, p2.y() - rounding - f32::cos(r) * rounding],
					[u.x() + urounding - f32::sin(r) * urounding, v.y() - vrounding - f32::cos(r) * vrounding],
					clr,
				)
			}
		} else {
			self.prim_write_vtx([p1.x(), p2.y()], [u.x(), v.y()], clr);
		}
		
		if flags.contains(imgui::DrawFlags::RoundCornersBottomRight) {
			for i in 10..16 {
				let r = i as f32 / 5.0 * std::f32::consts::PI / 2.0;
				self.prim_write_vtx(
					[p2.x() - rounding - f32::sin(r) * rounding, p2.y() - rounding - f32::cos(r) * rounding],
					[v.x() - urounding - f32::sin(r) * urounding, v.y() - vrounding - f32::cos(r) * vrounding],
					clr,
				)
			}
		} else {
			self.prim_write_vtx(p2, v, clr);
		}
		
		if flags.contains(imgui::DrawFlags::RoundCornersTopRight) {
			for i in 15..21 {
				let r = i as f32 / 5.0 * std::f32::consts::PI / 2.0;
				self.prim_write_vtx(
					[p2.x() - rounding - f32::sin(r) * rounding, p1.y() + rounding - f32::cos(r) * rounding],
					[v.x() - urounding - f32::sin(r) * urounding, u.y() + vrounding - f32::cos(r) * vrounding],
					clr,
				)
			}
		} else {
			self.prim_write_vtx([p2.x(), p1.y()], [v.x(), u.y()], clr);
		}
		
		for i in 2..vertices {
			self.prim_write_idx(0);
			self.prim_write_idx(i - 1);
			self.prim_write_idx(i);
		}
	}
	
	fn add_text_area(&self, pos: [f32; 2], col: u32, text: &str, area: [f32; 2]) {
		let height = imgui::get_font_size();
		for (i, (line, _)) in super::wrap_text_area(text, area).into_iter().enumerate() {
			self.add_text(pos.add([0.0, height * i as f32]), col, line);
		}
	}
}