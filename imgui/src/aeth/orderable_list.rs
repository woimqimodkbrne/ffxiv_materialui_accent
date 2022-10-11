use crate as imgui;
use super::F2;

pub fn orderable_list<T, F, F2>(id: &str, vec: &mut Vec<T>, mut context_menu: F, mut draw: F2) -> bool where
F: FnMut(usize, &mut T),
F2: FnMut(usize, &mut T) {
	orderable_list2(id, super::frame_height(), vec, &mut context_menu, &mut draw)
}

pub fn orderable_list2<T, F, F2>(id: &str, h: f32, vec: &mut Vec<T>, mut context_menu: F, mut draw: F2) -> bool where
F: FnMut(usize, &mut T),
F2: FnMut(usize, &mut T) {
	if vec.len() == 0 {return false}
	
	imgui::push_id(id);
	let cur = imgui::get_state_storage().i32(imgui::get_id(id), 0);
	let mpos = imgui::get_mouse_pos().y();
	let start = imgui::get_cursor_screen_pos().y();
	let end = start + (vec.len() - 1) as f32 * (h + imgui::get_style().item_spacing.y());
	for i in 0..vec.len() + 1 {
		if *cur != 0 {
			let y = imgui::get_cursor_screen_pos().y();
			let p = (mpos - (*cur >> 16) as f32).clamp(start, end);
			if p + h / 2.0 >= y && p + h / 2.0 < y + h + imgui::get_style().item_spacing.y() {
				imgui::dummy([0.0, h]);
			}
			if i == (*cur & 0xFFFF) as usize {continue}
		}
		
		if i == vec.len() {continue}
		
		imgui::push_id_i32(i as i32);
		super::button_icon(""); // fa-bars
		imgui::same_line();
		if imgui::is_item_clicked(imgui::MouseButton::Left) {
			let o = (mpos - imgui::get_cursor_screen_pos().y()) as i32;
			*cur = (o << 16) + (i as i32);
		}
		if imgui::is_item_clicked(imgui::MouseButton::Right) {imgui::open_popup("context", imgui::PopupFlags::MouseButtonRight)}
		if imgui::begin_popup("context", imgui::WindowFlags::None) {
			context_menu(i, vec.get_mut(i).unwrap());
			imgui::end_popup();
		}
		draw(i, vec.get_mut(i).unwrap());
		imgui::pop_id();
	}
	
	let pos = imgui::get_cursor_pos();
	let mut r = false;
	if *cur != 0 {
		let i = (*cur & 0xFFFF) as usize;
		let o = (*cur >> 16) as f32;
		imgui::push_id("move");
		imgui::set_cursor_screen_pos([imgui::get_cursor_screen_pos().x(), (mpos - o).clamp(start, end)]);
		super::button_icon("");
		imgui::same_line();
		draw(i, vec.get_mut(i).unwrap());
		imgui::pop_id();
		
		if !imgui::is_mouse_down(imgui::MouseButton::Left) {
			let curi = (((mpos - o + h / 2.0 - start) / (h + imgui::get_style().item_spacing.y())).floor() as usize).clamp(0, vec.len());
			if i < curi {
				vec[i..=curi].rotate_left(1);
				r = true;
			} else if i > curi {
				vec[curi..=i].rotate_right(1);
				r = true;
			}
			*cur = 0;
		}
	}
	imgui::set_cursor_pos(pos);
	imgui::pop_id();
	
	r
}