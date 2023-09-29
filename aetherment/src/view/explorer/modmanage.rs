use std::path::PathBuf;

pub struct ModManage {
	name: String,
	path: PathBuf,
	notif: Option<String>,
}

impl ModManage {
	pub fn new(name: impl Into<String>, path: impl Into<PathBuf>) -> Result<Self, super::BacktraceError> {
		Ok(Self {
			name: name.into(),
			path: path.into(),
			notif: None,
		})
	}
}

impl super::View for ModManage {
	fn name(&self) -> &str {
		&self.name
	}
	
	fn path(&self) -> &str {
		"_modmanage"
	}
	
	fn render(&mut self, ui: &mut egui::Ui) -> Result<(), super::BacktraceError> {
		// if ui.button("Cleanup reduntant files").clicked() {
		// 	self.notif = Some(match crate::modman::cleanup(&self.path) {
		// 		Ok(()) => format!("Cleaned reduntant files"),
		// 		Err(err) => format!("Failed cleaning reduntant files\n\n{err:?}"),
		// 	});
		// }
		
		if ui.button("Create modpack").clicked() {
			self.notif = Some(match crate::modman::create_mod(&self.path, crate::modman::ModCreationSettings {
				current_game_files_hash: true,
			}) {
				Ok(path) => format!("Created modpack at {path:?}"),
				Err(err) => format!("Failed creating modpack\n\n{err:?}"),
			});
		}
		
		if let Some(notif) = &self.notif {
			ui.label(notif);
		}
		
		let mut open = self.notif.is_some();
		egui::Window::new("Notif")
			.open(&mut open)
			.show(ui.ctx(), |ui| {
				let Some(notif) = &self.notif else {return};
				
				ui.horizontal_centered(|ui| {
					ui.label(notif);
				});
			});
		
		if !open {
			self.notif = None;
		}
		
		Ok(())
	}
}