use std::{fs::File, io::BufReader};
use egui::epaint;
use noumenon::formats::game::Tex as GameTex;

pub struct Tex {
	name: String,
	path: String,
	real_path: Option<String>,
	data: Option<GameTex>,
	texture: Option<egui::TextureHandle>,
	uv: egui::Rect,
}

impl Tex {
	pub fn new(ctx: egui::Context, path: &str, real_path: Option<&str>) -> Result<Self, super::BacktraceError> {
		let mut v = Self {
			name: path.split("/").last().unwrap().to_owned(),
			path: path.to_owned(),
			real_path: real_path.map(|v| v.to_owned()),
			data: None,
			texture: None,
			uv: egui::Rect{min: egui::pos2(0.0, 0.0), max: egui::pos2(1.0, 1.0)},
		};
		
		v.load_texture_data(ctx)?;
		
		Ok(v)
	}
	
	fn load_texture_data(&mut self, ctx: egui::Context) -> Result<(), super::BacktraceError> {
		let data = if let Some(real_path) = &self.real_path {
			let file = File::open(real_path)?;
			let mut reader = BufReader::new(file);
			self.data = None;
			GameTex::read(&mut reader)?
		} else {
			self.data = None;
			crate::NOUMENON.file::<GameTex>(&self.path)?
		};
		
		self.texture = Some(ctx.load_texture("explorer_tex", epaint::image::ColorImage::new([data.header.width as usize, data.header.height as usize], epaint::Color32::TRANSPARENT), Default::default()));
		self.data = Some(data);
		self.refresh_texture();
		
		Ok(())
	}
	
	fn refresh_texture(&mut self) {
		if let Some(data) = &mut self.data {
			if let Some(texture) = &mut self.texture {
				let (w, h, slice) = data.slice(0, 0);
				texture.set_partial([0; 2], egui::ColorImage::from_rgba_premultiplied([w as usize, h as usize], slice), Default::default());
				self.uv.max = egui::pos2(w as f32 / data.header.width as f32, h as f32 / data.header.height as f32);
			}
		}
	}
}

impl super::View for Tex {
	fn name<'a>(&'a self) -> &'a str {
		&self.name
	}
	
	fn path<'a>(&'a self) -> &'a str {
		&self.path
	}
	
	fn render(&mut self, ui: &mut egui::Ui) -> Result<(), super::BacktraceError> {
		if let Some(data) = &self.data {
			if let Some(texture) = &self.texture {
				let space = ui.available_size();
				let (w, h) = (data.header.width as f32, data.header.height as f32);
				let scale = (space.x / w).min(space.y / h);
				
				ui.vertical_centered_justified(|ui| {
					ui.centered_and_justified(|ui| {
						ui.add(egui::Image::new(texture, egui::vec2(w * scale, h * scale)).uv(self.uv));
					})
				});
			}
		}
		
		Ok(())
	}
}