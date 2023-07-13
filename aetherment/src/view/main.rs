pub struct Main {
	
}

impl Main {
	pub fn new() -> Self {
		Self {
			
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
	}
}