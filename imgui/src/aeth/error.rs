use std::sync::Mutex;
use crate as imgui;
use super::F2;

static ERROR: Mutex<Option<(String, String)>> = Mutex::new(None);

pub fn show_error<S, S2>(title: S, error: S2) where
S: Into<String>,
S2: Into<String> {
	*ERROR.lock().unwrap() = Some((title.into(), error.into()));
}

pub fn draw_error() {
	let mut err = ERROR.lock().unwrap();
	if let Some((title, error)) = err.as_ref() {
		let padding = imgui::get_style().window_padding;
		let textclr = imgui::get_color(imgui::Col::Text);
		let footer = super::frame_height();
		
		let bounds = [600.0, 800.0].sub(padding).sub(padding);
		let height = imgui::get_font_size();
		
		let lines_title = super::wrap_text_area(title, [bounds.x(), 999999.0]);
		let lines_body = super::wrap_text_area(error, [bounds.x(), 999999.0]);
		
		let bounds = [
			bounds.x().min(
				lines_title.iter().reduce(|a, b| if a.1 > b.1 {a} else {b}).unwrap().1.max(
				lines_body.iter().reduce(|a, b| if a.1 > b.1 {a} else {b}).unwrap().1)
			),
			bounds.y().min(
				lines_title.len() as f32 * height +
				lines_body.len() as f32 * height +
				height * 2.0 +
				footer
			),
		];
		
		imgui::set_next_window_pos(imgui::get_main_viewport_center(), imgui::Cond::Always, [0.5, 0.5]);
		imgui::set_next_window_size(bounds.add(padding).add(padding), imgui::Cond::Always);
		imgui::begin("###errorpopup", None, imgui::WindowFlags::Modal | imgui::WindowFlags::NoDecoration);
		
		let draw = imgui::get_window_draw_list();
		let mut pos = imgui::get_cursor_screen_pos();
		for (line, len) in lines_title.into_iter() {
			draw.add_text(pos.add([(bounds.x() - len) / 2.0, 0.0]), textclr, line);
			pos[1] += height;
		}
		pos[1] += height;
		for (line, _len) in lines_body.into_iter() {
			draw.add_text(pos, 0xFF3030B0, line);
			pos[1] += height;
		}
		pos[1] += height;
		
		imgui::set_cursor_screen_pos(pos);
		if imgui::button("Close", [bounds.x(), footer]) {*err = None}
		
		imgui::end_popup();
	}
}