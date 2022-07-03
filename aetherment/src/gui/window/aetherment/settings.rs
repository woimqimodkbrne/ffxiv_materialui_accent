use crate::gui::{imgui, aeth};

pub struct Tab();
impl Tab {
	pub fn new(_state: &mut crate::Data) -> Self {
		Tab {
			
		}
	}
	
	pub fn draw(&mut self, state: &mut crate::Data) {
		state.config.mark_for_changes();
		
		aeth::tab_bar("settings_tabs")
			.dock_top()
			.tab("Generic", || {
				aeth::child("generic", [0.0, -imgui::get_style().item_spacing[1]], false, imgui::WindowFlags::None, || {
					imgui::text("generic");
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