// TODO: save the changes to layers

use std::{fs::File, io::{Seek, Read, Cursor}, collections::HashMap, sync::Mutex, path::PathBuf};
use noumenon::formats::{game::tex::{self, Format}, external::{dds::Dds, png::Png}};
use crate::{gui::{imgui, aeth::{self, F2}}, GAME, apply::penumbra::{resolve_layer, ConfSetting, Layer as PLayer, Config, PenumbraFile}};
use super::Viewer;

pub struct Tex {
	ext: String,
	#[allow(dead_code)] gamepath: String,
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
	highlighted_layer: Option<usize>,
	moving_layer: Option<(usize, f32)>,
	
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
	pub fn new(gamepath: String, rootpath: Option<PathBuf>, realpaths: Option<PenumbraFile>, settings: Option<HashMap<String, ConfSetting>>) -> Self {
		let layers = if let Some(paths) = realpaths {
			let settings = settings.unwrap();
			paths.0.into_iter().map(|l| {
				Layer {
					value: if let Some(id) = l.id.as_ref() {Some(settings[id])} else {None},
					id: l.id,
					files: l.paths,
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
			highlighted_layer: None,
			moving_layer: None,
			cache: HashMap::new(),
		};
		t.reload_preview();
		t
	}
	
	fn reload_preview(&mut self) {
		let hl = self.highlighted_layer;
		let layers = self.layers.clone();
		let mut layers = layers.iter().enumerate();
		let mut get_file = |p: &str| -> Option<Vec<u8>> {
			self.get_file(p)
		};
		
		let layer = layers.next().unwrap();
		let mut tex = resolve_layer(&PLayer{value: layer.1.value, files: layer.1.files.clone()}, &mut get_file).expect("Failed resolving layer");
		if hl == Some(layer.0) {Self::clrtex(&mut tex)}
		while let Some((i, layer)) = layers.next() {
			let mut l = resolve_layer(&PLayer{value: layer.value, files: layer.files.clone()}, &mut get_file).expect("Failed resolving layer");
			if hl == Some(i) {Self::clrtex(&mut l)}
			l.overlay_onto(&mut tex);
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
	
	fn clrtex(tex: &mut tex::Tex) {
		tex.data.iter_mut().for_each(|p| {
			*p = ((*p).clone() as i16 - 32).clamp(0, 255) as u8;
		})
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
		vec![self.ext.to_owned(), ".dds".to_owned(), ".png".to_owned()]
	}
	
	fn valid_exports(&self) -> Vec<String> {
		vec![self.ext.to_owned(), ".dds".to_owned(), ".png".to_owned()]
	}
	
	fn draw(&mut self, state: &mut crate::Data, _conf: Option<&mut Config>) {
		aeth::divider("tex", true)
		.left(200.0, || {
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
		}).right(175.0, || {
			let header = &self.tex.as_ref().unwrap().header;
			imgui::text(&format!("{:?}", header.format));
			
			let mut changed = false;
			if !matches!(header.format, Format::A8) || !matches!(header.format, Format::L8) {
				changed |= imgui::checkbox("R", &mut self.r);
				imgui::same_line();
				changed |= imgui::checkbox("G", &mut self.g);
				imgui::same_line();
				changed |= imgui::checkbox("B", &mut self.b);
				imgui::same_line();
				changed |= imgui::checkbox("A", &mut self.a);
			}
			
			// TODO: fancier sliders
			if header.mip_levels > 1 {
				imgui::set_next_item_width(aeth::width_left() - 70.0);
				changed |= imgui::slider_int("Mip Level", &mut self.miplevel, 0, header.mip_levels as i32 - 1, "%d", imgui::SliderFlags::None);
			}
			
			let max_depth = 1f32.max(header.depths as f32 * 0.5f32.powf(self.miplevel as f32)) as i32 - 1;
			if max_depth > 1 {
				imgui::set_next_item_width(aeth::width_left() - 70.0);
				changed |= imgui::slider_int("Depth", &mut self.depth, 0, max_depth as i32, "%d", imgui::SliderFlags::None);
			}
			self.depth = self.depth.min(max_depth);
			
			// Layers, this is *kinda* a big mess
			aeth::offset([0.0, 20.0]);
			let h = aeth::frame_height();
			let spos = imgui::get_cursor_screen_pos().y();
			let mut pos = imgui::get_mouse_pos().y();
			if let Some(ml) = self.moving_layer {
				pos = (pos - ml.1).clamp(spos, spos + (self.layers.len() - 1) as f32 * (h + imgui::get_style().item_spacing.y()))
			}
			let mut hl = None;
			let layers: &mut Vec<Layer> = self.layers.as_mut();
			// for (i, layer) in layers.iter_mut().enumerate() {
			let ll = layers.len();
			for mut i in (0..ll).rev() {
				if let Some(ml) = self.moving_layer {
					let y = imgui::get_cursor_screen_pos().y();
					if pos + h / 2.0 >= y && pos + h / 2.0 < y + h + imgui::get_style().item_spacing.y() {
						imgui::dummy([0.0, h]);
					}
					if ml.0 == i {continue;}
				}
				
				imgui::push_id_i32(i as i32);
				// TODO: make custom element sortable list?
				aeth::button_icon("", state.fa5); // fa-bars
				imgui::same_line();
				
				if imgui::is_item_active() {
					let o = imgui::get_mouse_pos().y() - imgui::get_cursor_screen_pos().y();
					pos -= o;
					self.moving_layer = Some((i, o));
				}
				
				if imgui::is_item_clicked(imgui::MouseButton::Right) {
					imgui::open_popup("layeredit", imgui::PopupFlags::MouseButtonLeft)
				}
				
				aeth::popup("layeredit", imgui::WindowFlags::None, || {
					// Removing the last layer isnt a great idea
					if ll > 1 && imgui::button("Remove", [0.0, 0.0]) {
						layers.remove(i);
						changed = true;
						i -= 1;
					}
				});
				
				let layer = layers.get_mut(i).unwrap();
				if let Some(id) = &layer.id {
					if layer.value.as_mut().unwrap().draw(id) {
						changed = true;
						let id = layer.id.clone();
						let value = layer.value.clone();
						for j in 0..ll {
							let layer2 = layers.get_mut(j).unwrap();
							if layer2.id == id {
								layer2.value = value;
							}
						}
					}
				} else {
					imgui::text("Generic Layer");
				}
				
				if imgui::is_item_hovered() {
					hl = Some(i);
				}
				imgui::pop_id();
			}
			
			if let Some(ml) = self.moving_layer {
				imgui::set_cursor_screen_pos([imgui::get_cursor_screen_pos().x(), pos]);
				
				if !imgui::is_mouse_down(imgui::MouseButton::Left) {
					let mli = layers.len() - 1 - (((pos + h / 2.0 - spos) / (h + imgui::get_style().item_spacing.y())).floor() as usize).min(self.layers.len() - 1);
					if ml.0 < mli {
						self.layers[ml.0..=mli].rotate_left(1);
					} else if ml.0 > mli {
						self.layers[mli..=ml.0].rotate_right(1);
					}
					
					self.moving_layer = None;
					changed = true;
				} else {
					aeth::button_icon("", state.fa5); // fa-bars
					imgui::same_line();
					
					let layer = self.layers.get_mut(ml.0).unwrap();
					if let Some(id) = &layer.id {
						changed |= layer.value.as_mut().unwrap().draw(id);
					} else {
						imgui::text("Generic Layer");
					}
				}
			} else {
				aeth::button_icon("", state.fa5); // fa-plus
			}
			
			if self.highlighted_layer != hl {
				self.highlighted_layer = hl;
				changed = true;
			}
			
			if changed {self.reload_preview();}
		});
	}
	
	fn save(&self, ext: &str, writer: &mut Vec<u8>) {
		let tex = self.tex.as_ref().unwrap();
		let cursor = &mut Cursor::new(writer);
		
		match ext {
			"tex" => tex::Tex::write(tex, cursor),
			"dds" => <tex::Tex as Dds>::write(tex, cursor),
			"png" => <tex::Tex as Png>::write(tex, cursor),
			_ => {},
		}
	}
}