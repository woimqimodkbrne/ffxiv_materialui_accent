use imgui::aeth::{DrawList, F2};
use crate::gui::aeth;

pub struct Tab {
	test: [f32; 2],
}

impl Tab {
	pub fn new(_state: &mut crate::Data) -> Self {
		Tab {
			test: [100.0, 50.0],
		}
	}
	
	pub fn draw(&mut self, state: &mut crate::Data) {
		state.config.mark_for_changes();
		
		aeth::tab_bar("settings_tabs")
			.dock_top()
			.tab("Generic", || {
				aeth::child("generic", [0.0, -imgui::get_style().item_spacing[1]], false, imgui::WindowFlags::None, || {
					imgui::text("generic");
					
					imgui::slider_float2("area", &mut self.test, 0.0, 500.0, "%.0f", imgui::SliderFlags::None);
					
					let draw = imgui::get_window_draw_list();
					let pos = imgui::get_cursor_screen_pos();
					let text = "hello there, this is a very nice   test to,check text wrapping";
					draw.add_text_area(pos, 0xFFFFFFFF, text, self.test);
					draw.add_rect(pos, pos.add(self.test), 0xFFFFFFFF, 0.0, imgui::DrawFlags::None, 2.0);
				});
			})
			.tab("Advanced", || {
				aeth::child("advanced", [0.0, -imgui::get_style().item_spacing[1]], false, imgui::WindowFlags::None, || {
					imgui::input_text("Local Mod Directory", &mut state.config.local_path, imgui::InputTextFlags::None);
					imgui::checkbox("File Explorer", &mut state.config.tab_explorer);
					imgui::checkbox("Mod Development", &mut state.config.tab_moddev);
				});
			})
			.finish();
		
		state.config.save().unwrap();
	}
}