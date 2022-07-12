use crate::gui::imgui;
use super::F2 as _;

pub struct TabBar<'a> {
	id: &'a str,
	docked_bottom: bool,
	ignore_add: bool,
	tabs: Vec<&'a str>,
	rect: [f32; 4],
}

impl<'a> TabBar<'a> {
	pub fn dock_top(mut self) -> Self {
		self.docked_bottom = false;
		self
	}
	
	pub fn condition(mut self, condition: bool) -> Self {
		self.ignore_add = !condition;
		self
	}
	
	pub fn tab<F>(mut self, label: &'a str, func: F) -> Self where F: FnOnce() {
		if imgui::get_state_storage().get_i32(imgui::get_id(self.id), 0) == self.tabs.len() as i32 {
			func();
		}
		if !self.ignore_add {
			self.tabs.push(label);
		}
		self.ignore_add = false;
		self
	}
	
	pub fn finish(self) {
		let id = imgui::get_id(self.id);
		let curtab = imgui::get_state_storage().i32(id, 0);
		let s = imgui::get_style();
		let draw = imgui::get_window_draw_list();
		
		let trueorg = imgui::get_cursor_pos();
		let org = [self.rect[0], self.rect[1]];
		let barw = self.rect[2];
		let tabw = barw / self.tabs.len().max(1) as f32;
		let tabh = self.rect[3];
		let tabsize = [tabw, tabh];
		let l = self.tabs.len() - 1;
		
		let clrn = imgui::get_color(imgui::Col::Tab);
		let clrh = imgui::get_color(imgui::Col::TabHovered);
		let clra = imgui::get_color(imgui::Col::TabActive);
		let clrt = imgui::get_color(imgui::Col::Text);

		for (i, tab) in self.tabs.iter().enumerate() {
			let pos = [org[0] + tabw * i as f32, org[1]];
			imgui::set_cursor_pos(pos);
			let pos = pos.add(imgui::get_window_pos());
			imgui::dummy(tabsize);
			if imgui::is_item_clicked(imgui::MouseButton::Left) {
				*curtab = i as i32;
			}
			
			draw.add_rect_filled(
				if i != 0 {pos.add([1.0, 0.0])} else {pos},
				pos.add(if i != l {tabsize.sub([1.0, 0.0])} else {tabsize}),
				if imgui::is_item_hovered() {clrh} else if *curtab == i as i32 {clra} else {clrn},
				s.tab_rounding,
				if i == 0 {if self.docked_bottom {imgui::DrawFlags::RoundCornersTopLeft} else {imgui::DrawFlags::RoundCornersBottomLeft}}
				else if i == l {if self.docked_bottom {imgui::DrawFlags::RoundCornersTopRight} else {imgui::DrawFlags::RoundCornersBottomRight}}
				else {imgui::DrawFlags::RoundCornersNone}
			);
			
			draw.add_text(pos.add(s.frame_padding), clrt, tab);
		}
		
		let p = org.add(imgui::get_window_pos()).add(if self.docked_bottom {[0.0, tabh]} else {[0.0, 0.0]});
		draw.add_line(p, p.add([barw, 0.0]), clra, 1.0);
		
		*curtab = (*curtab).min(self.tabs.len() as i32 - 1);
		imgui::set_cursor_pos(trueorg);
	}
}

pub fn tab_bar(id: &str) -> TabBar {
	let pos = imgui::get_cursor_pos();
	let size = [imgui::get_column_width(-1), super::frame_height()];
	imgui::dummy(size);
	
	TabBar {
		id,
		docked_bottom: true,
		ignore_add: false,
		tabs: Vec::new(),
		rect: [pos[0], pos[1], size[0], size[1]],
	}
}