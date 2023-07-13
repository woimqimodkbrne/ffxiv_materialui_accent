pub struct Debug {}

impl Debug {
	pub fn new() -> Self {
		Self {}
	}
}

impl super::View for Debug {
	fn name(&self) -> &'static str {
		&"Debug"
	}
	
	fn render(&mut self, ui: &mut egui::Ui) {
		ui.ctx().clone().style_ui(ui);
	}
}