use std::{fs::File, io::{Seek, Read}, collections::HashMap, sync::Mutex, path::PathBuf};
use noumenon::formats::game::tex::{self, Format};
use crate::{gui::{imgui, aeth::{self, F2}}, GAME, apply::penumbra::{resolve_layer, ConfSetting, Layer as PLayer}};
use super::Viewer;

pub struct Tex {
	ext: String,
	gamepath: String,
	rootpath: Option<PathBuf>,
	tex: Option<tex::Tex>,
	layers: Vec<Layer>,
	width: u16,
	height: u16,
	miplevel: i32,
	depth: i32,
	r: bool,
	g: bool,
	b: bool,
	a: bool,
	
	cache: HashMap<String, Vec<u8>>,
}

#[derive(Debug, Clone)]
struct Layer {
	id: Option<String>,
	value: Option<ConfSetting>,
	files: Vec<String>,
}

const TEXSIZE: i32 = 1024;
lazy_static!{
	static ref TEXTURE: Mutex<aeth::Texture> = Mutex::new(aeth::Texture::new(aeth::TextureOptions {
		width: TEXSIZE,
		height: TEXSIZE,
		format: 87, // DXGI_FORMAT_B8G8R8A8_UNORM
		usage: 2, // D3D11_USAGE_DYNAMIC
		cpu_access_flags: 0x10000, // D3D11_CPU_ACCESS_WRITE
	}));
}

impl Tex {
	pub fn new(gamepath: String, rootpath: Option<PathBuf>, realpaths: Option<Vec<Vec<Option<String>>>>, settings: Option<HashMap<String, ConfSetting>>) -> Self {
		let layers = if let Some(paths) = realpaths {
			let settings = settings.unwrap();
			paths.into_iter().map(|l| {
				Layer {
					id: l[0].clone(),
					value: if let Some(id) = l[0].as_ref() {Some(settings[id])} else {None},
					files: l.into_iter()
						.enumerate()
						.filter(|(i, _)| *i > 0)
						.map(|(_, p)| p.unwrap())
						.collect::<Vec<String>>()
				}
			})
			.collect::<Vec<Layer>>()
		} else {
			vec![Layer {
				id: None,
				value: None,
				files: vec![gamepath.clone()],
			}]
		};
		
		let mut t = Tex {
			ext: format!(".{}", gamepath.split('.').last().unwrap()),
			gamepath,
			rootpath,
			tex: None,
			layers,
			miplevel: 0,
			depth: 0,
			width: 0,
			height: 0,
			r: true,
			g: true,
			b: true,
			a: true,
			cache: HashMap::new(),
		};
		t.reload_preview();
		t
	}
	
	fn reload_preview(&mut self) {
		let layers = self.layers.clone();
		let mut layers = layers.iter();
		let mut get_file = |p: &str| -> Option<Vec<u8>> {
			self.get_file(p)
		};
		
		let layer = layers.next().unwrap();
		let mut tex = resolve_layer(&PLayer{value: layer.value, files: layer.files.clone()}, &mut get_file).expect("Failed resolving layer");
		while let Some(layer) = layers.next() {
			resolve_layer(&PLayer{value: layer.value, files: layer.files.clone()}, &mut get_file).expect("Failed resolving layer").overlay_onto(&mut tex);
		}
		
		let isa = !self.r && !self.g && !self.b && self.a;
		
		// Nearest neighbour scaling
		let (w, h, slice) = tex.slice(self.miplevel as u16, self.depth as u16);
		let bx = TEXSIZE as f32 / w as f32;
		let by = TEXSIZE as f32 / h as f32;
		let mut data = Vec::with_capacity(TEXSIZE as usize * TEXSIZE as usize * 4);
		for y in 0..TEXSIZE as usize {
			for x in 0..TEXSIZE as usize {
				let i = (y as f32 / by).floor() as usize * 4 * w as usize + (x as f32 / bx).floor() as usize * 4;
				data.push(if isa {slice[i + 3]} else if self.b {slice[i    ]} else {0});
				data.push(if isa {slice[i + 3]} else if self.g {slice[i + 1]} else {0});
				data.push(if isa {slice[i + 3]} else if self.r {slice[i + 2]} else {0});
				data.push(if isa {255} else if self.a {slice[i + 3]} else {255});
			}
		}
		
		self.tex = Some(tex);
		self.width = w;
		self.height = h;
		
		TEXTURE.lock().unwrap().draw_to(&data).unwrap();
	}
	
	fn get_file(&mut self, path: &str) -> Option<Vec<u8>> {
		if let Some(v) = self.cache.get(path) {
			return Some(v.clone());
		}
		
		log!("loading {}", path);
		// TODO: allow reading from mods with lower priority
		let data = if let Some(root) = self.rootpath.as_mut() {
			let mut f = File::open(root.join(path)).unwrap();
			let mut buf = Vec::with_capacity(f.stream_len().unwrap() as usize);
			f.read_to_end(&mut buf).unwrap();
			Some(buf)
		} else {
			GAME.file::<Vec<u8>>(path).ok()
		};
		
		if let Some(v) = data.clone() {
			self.cache.insert(path.to_owned(), v);
		}
		
		data
	}
}

impl Viewer for Tex {
	fn valid_imports(&self) -> Vec<String> {
		vec![self.ext.to_owned(), ".dds".to_owned(), ".tex".to_owned()]
	}
	
	fn valid_exports(&self) -> Vec<String> {
		vec![self.ext.to_owned(), ".dds".to_owned(), ".tex".to_owned()]
	}
	
	fn draw(&mut self, _state: &mut crate::Data) {
		let header = &self.tex.as_ref().unwrap().header;
		let mut changed = false;
		if !matches!(header.format, Format::A8) || !matches!(header.format, Format::L8) {
			changed |= imgui::checkbox("R", &mut self.r);
			imgui::same_line();
			changed |= imgui::checkbox("G", &mut self.g);
			imgui::same_line();
			changed |= imgui::checkbox("B", &mut self.b);
			imgui::same_line();
			changed |= imgui::checkbox("A", &mut self.a);
			imgui::same_line();
		}
		
		// TODO: fancier sliders
		if header.mip_levels > 1 {
			imgui::set_next_item_width(200.0);
			changed |= imgui::slider_int("Mip Level", &mut self.miplevel, 0, header.mip_levels as i32 - 1, "%d", imgui::SliderFlags::None);
			imgui::same_line();
		}
		
		let max_depth = 1f32.max(header.depths as f32 * 0.5f32.powf(self.miplevel as f32)) as i32 - 1;
		if max_depth > 1 {
			imgui::set_next_item_width(200.0);
			changed |= imgui::slider_int("Depth", &mut self.depth, 0, max_depth as i32, "%d", imgui::SliderFlags::None);
			imgui::same_line();
		}
		self.depth = self.depth.min(max_depth);
		
		imgui::set_next_item_width(imgui::get_column_width(-1));
		aeth::combo("##layers", "Layers", imgui::ComboFlags::None, || {
			for layer in self.layers.iter_mut() {
				if let Some(id) = &layer.id {
					changed |= layer.value.as_mut().unwrap().draw(id);
				} else {
					imgui::text("Generic Layer");
				}
			}
		});
		
		if changed {
			self.reload_preview();
		}
		
		let draw = imgui::get_window_draw_list();
		let pos = imgui::get_cursor_screen_pos();
		let size = imgui::get_content_region_avail();
		
		let scale = (size.x() / self.width as f32).min(size.y() / self.height as f32);
		let (w, h) = (self.width as f32 * scale, self.height as f32 * scale);
		let preview_pos = pos.add([(size.x() - w) / 2.0, (size.y() - h) / 2.0]);
		
		// This is dumb, TODO: use a shader or smth
		draw.add_rect_filled(preview_pos, preview_pos.add([w, h]), 0xFF303030, 0.0, imgui::DrawFlags::None);
		for x in (0..(w / 32.0).ceil() as usize).step_by(2) {
			for y in 0..(h / 32.0).ceil() as usize {
				if y % 2 != 0 && x + 1 >= (w / 32.0).ceil() as usize {
					continue;
				}
				
				let p = [x as f32 * 32.0 + if y % 2 == 0 {0.0} else {32.0}, y as f32 * 32.0];
				let pos = preview_pos.add(p);
				draw.add_rect_filled(pos, pos.add([32f32.min(w - p.x()), 32f32.min(h - p.y())]), 0xFFCFCFCF, 0.0, imgui::DrawFlags::None);
			}
		}
		
		draw.add_image(TEXTURE.lock().unwrap().resource(), preview_pos, preview_pos.add([w, h]), [0.0, 0.0], [1.0, 1.0], 0xFFFFFFFF);
	}
	
	fn save(&self, _writer: &mut File) {
		todo!()
	}
}