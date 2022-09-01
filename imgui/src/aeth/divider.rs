use crate as imgui;
use super::F2;

pub struct Divider<'a> {
	id: &'a str,
	minsize: f32,
	w: f32,
	sr: bool,
}

impl<'a> Divider<'a> {
	pub fn left<F>(mut self, minsize: f32, func: F) -> Self where F: FnOnce() {
		let id = imgui::get_id(self.id);
		self.w = imgui::get_content_region_avail().x() - 16.0;
		let p = imgui::get_state_storage().f32(id, self.w * if self.sr {0.75} else {0.25});
		if self.sr {
			let w = imgui::get_state_storage().f32(imgui::get_id(&format!("{}_w", id)), self.w);
			*p = (*p - (*w - self.w)).max(minsize);
			*w = self.w;
		}
		
		
		imgui::push_id(self.id);
		imgui::begin_child("left", [*p, -imgui::get_style().item_spacing.y()], false, imgui::WindowFlags::None);
		func();
		imgui::end_child();
		imgui::pop_id();
		
		self.minsize = minsize;
		self
	}
	
	pub fn right<F>(self, minsize: f32, func: F) where F: FnOnce() {
		let id = imgui::get_id(self.id);
		let s = imgui::get_style();
		
		imgui::push_id(self.id);
		imgui::same_line();
		super::offset([-s.item_spacing.x(), 0.0]);
		let p = imgui::get_state_storage().f32(id, 0.0);
		let pos = imgui::get_cursor_screen_pos();
		let h = imgui::get_content_region_avail().y() - s.item_spacing.y();
		imgui::invisible_button("##div", [16.0, h], imgui::ButtonFlags::MouseButtonLeft);
		if imgui::is_item_active() {
			*p += imgui::get_mouse_drag_delta(imgui::MouseButton::Left, 0.0).x();
			imgui::reset_mouse_drag_delta(imgui::MouseButton::Left);
		}
		*p = (*p).clamp(self.minsize, (self.w - minsize).max(self.minsize));
		
		imgui::get_window_draw_list().add_line(
			pos.add([8.0, 0.0]),
			pos.add([8.0, h]),
			if imgui::is_item_active() {imgui::get_color(imgui::Col::SeparatorActive)}
				else if imgui::is_item_hovered() {imgui::get_color(imgui::Col::SeparatorHovered)}
				else {imgui::get_color(imgui::Col::Separator)},
			2.0
		);
		
		imgui::same_line();
		super::offset([-s.item_spacing.x(), 0.0]);
		imgui::begin_child("right", [imgui::get_content_region_avail().x(), -imgui::get_style().item_spacing.y()], false, imgui::WindowFlags::None);
		func();
		imgui::end_child();
		imgui::pop_id();
	}
}

pub fn divider(id: &str, stick_right: bool) -> Divider {
	Divider {
		id,
		minsize: 0.0,
		w: 0.0,
		sr: stick_right,
	}
}