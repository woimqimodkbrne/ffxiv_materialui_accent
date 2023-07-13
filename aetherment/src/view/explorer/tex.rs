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
					texture.set_partial([0; 2], egui::ColorImage::from_rgba_premultiplied([w as usize, h as usize], &slice), Default::default());
					self.uv.max = egui::pos2(w as f32 / data.header.width as f32, h as f32 / data.header.height as f32);
				}
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
		let pos = ui.cursor().min;
		let space = ui.available_size();
		
		// if let Some(texture) = &self.texture {
		if self.texture.is_some() {
			let style = ui.style();
			egui::Window::new("Options")
				.frame(egui::Frame {
					inner_margin: style.spacing.window_margin,
					outer_margin: Default::default(),
					shadow: egui::epaint::Shadow::NONE,
					rounding: style.visuals.window_rounding,
					fill: style.visuals.window_fill(),
					stroke: style.visuals.window_stroke(),
				})
				.drag_bounds(egui::Rect{min: pos, max: pos + space})
				.resizable(false)
				.show(ui.ctx(), |ui| {
					let data = &mut self.data.as_ref().unwrap();
					let mut changed = ui.add(egui::Slider::new(&mut self.mip, 0..=(data.header.mip_levels - 1)).text("Mip Level")).changed();
					changed |= ui.add(egui::Slider::new(&mut self.depth, 0..=(data.header.depths - 1)).text("Depth")).changed();
					changed |= ui.checkbox(&mut self.r, "R").changed();
					changed |= ui.checkbox(&mut self.g, "G").changed();
					changed |= ui.checkbox(&mut self.b, "B").changed();
					changed |= ui.checkbox(&mut self.a, "A").changed();
					
					if changed {
						self.refresh_texture()
					}
				});
		}
			
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
}