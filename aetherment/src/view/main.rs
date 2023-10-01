pub struct Main {
	import_dialog: Option<egui_file::FileDialog>,
}

impl Main {
	pub fn new() -> Self {
		Self {
			import_dialog: None,
		}
	}
}

impl super::View for Main {
	fn name(&self) -> &'static str {
		&"Main"
	}
	
	fn render(&mut self, ui: &mut egui::Ui) {
		if ui.button("Download Paths").clicked() {
			crate::view::explorer::tree::update_paths()
		}
		
		if ui.button("Import").clicked() {
			let mut dialog = egui_file::FileDialog::open_file(Some(crate::config().config.file_dialog_path.clone()))
				.filter(Box::new(|p| p.extension().map(|e| e == "aeth").unwrap_or(false)))
				.title("Import mod");
			dialog.open();
			self.import_dialog = Some(dialog);
		}
		
		if let Some(dialog) = &mut self.import_dialog {
			if dialog.show(ui.ctx()).selected() {
				if let Some(path) = dialog.path() {
					let path = path.to_owned();
					
					if let Some(parent) = path.parent() {
						crate::config().config.file_dialog_path = parent.to_owned();
						_ = crate::config().save_forced();
					}
					
					match crate::backend().install_mod(&path) {
						Ok(v) => log!("Successfully installed mod {v}"),
						Err(e) => log!(err, "Failed to install mod: {e}"),
					}
				}
			}
		}
	}
}