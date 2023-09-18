use crate::render_helper::RendererExtender;

pub struct Settings {
	
}

impl Settings {
	pub fn new() -> Self {
		Self {
			
		}
	}
}

impl super::View for Settings {
	fn name(&self) -> &'static str {
		&"Settings"
	}
	
	fn render(&mut self, ui: &mut egui::Ui) {
		let config_manager = crate::config();
		config_manager.mark_for_changes();
		let config = &mut config_manager.config;
		
		ui.horizontal(|ui| {
			let mut game_install = config.game_install.is_some();
			ui.add(egui::Checkbox::without_text(&mut game_install));
			if game_install != config.game_install.is_some() {
				if game_install {
					config.game_install = Some("".to_owned());
				} else {
					config.game_install = None;
				}
			}
			
			if let Some(game_install) = &mut config.game_install {
				ui.text_edit_singleline(game_install);
			}
			
			ui.label("Game install location");
			ui.helptext("Path to the game, use this if you use a custom location where autodetection fails (requires a restart (for now))\nExample: Z:/SteamLibrary/steamapps/common/FINAL FANTASY XIV - A Realm Reborn")
		});
		
		ui.horizontal(|ui| {
			ui.label("Repositories");
			ui.helptext("List of third party repositories to fetch mods from and show in the browser");
		});
		egui::TextEdit::singleline(&mut crate::MODREPO.to_string()).show(ui);
		let mut delete = None;
		for (i, repo) in config.repos.iter_mut().enumerate() {
			ui.horizontal(|ui| {
				ui.text_edit_singleline(repo);
				if ui.button("🗑").clicked() {
					delete = Some(i);
				}
			});
		}
		
		if let Some(i) = delete {
			config.repos.remove(i);
		}
		
		if ui.button("➕ Add new repository").clicked() {
			config.repos.push(String::new());
		}
		
		_ = crate::config().save();
	}
}