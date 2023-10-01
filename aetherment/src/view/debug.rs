pub struct Debug {
	texture: Option<egui::TextureHandle>,
}

impl Debug {
	pub fn new() -> Self {
		Self {
			texture: None,
		}
	}
}

impl super::View for Debug {
	fn name(&self) -> &'static str {
		&"Debug"
	}
	
	fn render(&mut self, ui: &mut egui::Ui) {
		let texture: &egui::TextureHandle = self.texture.get_or_insert_with(|| {
			ui.ctx().load_texture("", egui::ColorImage::example(), Default::default())
		});
		
		ui.image(texture, egui::vec2(400.0, 400.0));
		ui.add_space(20.0);
		
		ui.ctx().clone().style_ui(ui);
		ui.add_space(20.0);
		
		let backend = crate::backend();
		ui.label(format!("Backend debug ({})", backend.name()));
		ui.label(backend.description());
		backend.debug_renderer(ui);
	}
}