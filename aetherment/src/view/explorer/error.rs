pub struct Error {
	name: String,
	path: String,
	real_path: Option<String>,
	err: super::BacktraceError,
}

impl Error {
	pub fn new(path: &str, real_path: Option<&str>, err: super::BacktraceError) -> Self {
		Self {
			name: path.split("/").last().unwrap().to_owned(),
			path: path.to_owned(),
			real_path: real_path.map(|v| v.to_owned()),
			err,
		}
	}
}

impl super::View for Error {
	fn name(&self) -> &str {
		&self.name
	}
	
	fn path(&self) -> &str {
		&self.path
	}
	
	fn exts(&self) -> Vec<&str> {
		vec![self.name.split(".").last().unwrap()]
	}
	
	fn render(&mut self, ui: &mut egui::Ui) -> Result<(), super::BacktraceError> {
		// ui.vertical_centered_justified(|ui| {
		// 	ui.centered_and_justified(|ui| {
		ui.label(egui::RichText::new(format!("{}\n{:?}\n\n{:?}", self.path, self.real_path, self.err))
			.color(egui::epaint::Color32::RED));
		// 	})
		// });
		
		Ok(())
	}
}