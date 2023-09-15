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
			let mut local_path = config.local_path.is_some();
			ui.add(egui::Checkbox::without_text(&mut local_path));
			if local_path != config.local_path.is_some() {
				if local_path {
					config.local_path = Some("".to_owned());
				} else {
					config.local_path = None;
				}
			}
			
			if let Some(local_path) = &mut config.local_path {
				ui.text_edit_singleline(local_path);
			}
			
			ui.label("Local path");
			ui.helptext("Path to local dev mods, this should only be set for mod creators. If you're one, set it to a new empty directory")
		});
		ui.add_space(10.0);
		
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