use std::{fs::File, io::BufReader};
use noumenon::format::game::Tex as GameTex;

pub struct Tex {
	name: String,
	path: String,
	real_path: Option<String>,
	data: Option<GameTex>,
	texture: Option<egui::TextureHandle>,
	uv: egui::Rect,
	
	mip: u16,
	depth: u16,
	r: bool,
	g: bool,
	b: bool,
	a: bool,
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
			
			mip: 0,
			depth: 0,
			r: true,
			g: true,
			b: true,
			a: true,
		};
		
		v.load_texture_data(ctx)?;
		
		Ok(v)
	}
	
	fn load_texture_data(&mut self, ctx: egui::Context) -> Result<(), super::BacktraceError> {
		self.data = None;
		let data = if let Some(real_path) = &self.real_path {
			let file = File::open(real_path).map_err(|_| super::ExplorerError::RealPath(real_path.clone()))?;
			let mut reader = BufReader::new(file);
			GameTex::read(&mut reader)?
		} else {
			crate::NOUMENON.as_ref().ok_or(super::ExplorerError::Path(self.path.clone()))?.file::<GameTex>(&self.path)?
		};
		
		self.texture = Some(ctx.load_texture("explorer_tex", egui::epaint::image::ColorImage::new([data.header.width as usize, data.header.height as usize], egui::epaint::Color32::TRANSPARENT), Default::default()));
		self.data = Some(data);
		self.refresh_texture();
		
		Ok(())
	}
	
	fn refresh_texture(&mut self) {
		if let Some(data) = &mut self.data {
			if let Some(texture) = &mut self.texture {
				if let Some((w, h, slice)) = data.slice(self.mip, self.depth) {
					let isa = !self.r && !self.g && !self.b && self.a;
					let slice = slice
						.chunks_exact(4)
						.flat_map(|v| [
							if isa {v[3]} else if self.r {v[0]} else {0},
							if isa {v[3]} else if self.g {v[1]} else {0},
							if isa {v[3]} else if self.b {v[2]} else {0},
							if isa {255} else if self.a {v[3]} else {255},
						])
						.collect::<Vec<u8>>();
					texture.set_partial([0; 2], egui::ColorImage::from_rgba_unmultiplied([w as usize, h as usize], &slice), Default::default());
					self.uv.max = egui::pos2(w as f32 / data.header.width as f32, h as f32 / data.header.height as f32);
				}
			}
		}
	}
}

impl super::View for Tex {
	fn name(&self) -> &str {
		&self.name
	}
	
	fn path(&self) -> &str {
		&self.path
	}
	
	fn exts(&self) -> Vec<&str> {
		vec![self.name.split(".").last().unwrap(), "dds", "png", "tiff"]
	}
	
	fn render(&mut self, ui: &mut egui::Ui) -> Result<(), super::BacktraceError> {
		let space = ui.available_size();
			
		if let Some(texture) = &self.texture {
			if let Some(data) = &self.data {
				let (w, h) = (data.header.width as f32, data.header.height as f32);
				let scale = (space.x / w).min(space.y / h);
				
				// TODO: checkerboard background
				ui.vertical_centered_justified(|ui| {
					ui.centered_and_justified(|ui| {
						ui.add(egui::Image::new(texture, egui::vec2(w * scale, h * scale)).uv(self.uv));
					})
				});
			}
		}
		
		Ok(())
	}
	
	fn render_options(&mut self, ui: &mut egui::Ui) -> Result<(), super::BacktraceError> {
		if self.texture.is_some() {
			let data = &mut self.data.as_ref().unwrap();
			let mut changed = ui.add(egui::Slider::new(&mut self.mip, 0..=(data.header.mip_levels - 1)).text("Mip Level")).changed();
			changed |= ui.add(egui::Slider::new(&mut self.depth, 0..=(data.header.depths - 1)).text("Depth")).changed();
			ui.horizontal(|ui| {
				changed |= ui.checkbox(&mut self.r, "R").changed();
				changed |= ui.checkbox(&mut self.g, "G").changed();
				changed |= ui.checkbox(&mut self.b, "B").changed();
				changed |= ui.checkbox(&mut self.a, "A").changed();
			});
			
			if changed {
				self.refresh_texture()
			}
		}
		
		Ok(())
	}
	
	fn export(&self, ext: &str, mut writer: Box<dyn super::Writer>) -> Result<(), super::BacktraceError> {
		if let Some(data) = &self.data {
			noumenon::Convert::Tex(data.clone()).convert(ext, &mut writer)?;
			Ok(())
		} else {
			Err(super::ExplorerError::Data.into())
		}
	}
}