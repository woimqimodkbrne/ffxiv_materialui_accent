use crate::{gui::{imgui, aeth}};

mod settings;
mod manager;
mod browser;
mod explorer;
mod creator;

pub struct Window {
	pub visible: bool,
	pub settings: settings::Tab,
	pub manager: manager::Tab,
	pub browser: browser::Tab,
	pub explorer: explorer::Tab,
	pub creator: creator::Tab,
}

impl Window {
	pub fn new(state: &mut crate::Data) -> Self {
		Window {
			visible: true,
			settings: settings::Tab::new(state),
			manager: manager::Tab::new(state),
			browser: browser::Tab::new(state),
			explorer: explorer::Tab::new(state),
			creator: creator::Tab::new(state),
		}
	}
	
	pub fn draw(&mut self, state: &mut crate::Data) -> anyhow::Result<()> {
		aeth::tab_bar("tabs")
			.tab("Settings", || {
				aeth::offset([0.0, -imgui::get_style().item_spacing[1]]);
				self.settings.draw(state)
			})
			.tab("Mod Manager", ||{
				self.manager.draw(state);
			})
			.tab("Mod Browser", || {
				aeth::offset([0.0, -imgui::get_style().item_spacing[1]]);
				self.browser.draw(state);
			})
			.condition(state.config.tab_explorer).tab("File Explorer", || {
				self.explorer.draw(state);
			})
			.condition(state.config.tab_moddev).tab("Mod Creator", || {
				self.creator.draw(state);
			})
			.finish();
		
		Ok(())
	}
}