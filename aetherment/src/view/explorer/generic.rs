pub struct Generic {
	name: String,
	path: String,
	real_path: Option<String>,
}

impl Generic {
	pub fn new(path: &str, real_path: Option<&str>) -> Self {
		Self {
			name: path.split("/").last().unwrap().to_owned(),
			path: path.to_owned(),
			real_path: real_path.map(|v| v.to_owned()),
		}
	}
}

impl super::View for Generic {
	fn name<'a>(&'a self) -> &'a str {
		&self.name
	}
	
	fn path<'a>(&'a self) -> &'a str {
		&self.path
	}
	
	fn render(&mut self, ui: &mut egui::Ui) -> Result<(), super::BacktraceError> {
		ui.label("TODO");
		
		Ok(())
	}
}