use std::{fs::File, io::{Cursor, Write}, collections::HashMap, sync::Mutex, path::{PathBuf, Path}};
use noumenon::formats::{game::tex::{self, Format}, external::{dds::Dds, png::Png}};
use crate::{gui::aeth::{self, F2}, apply::penumbra::{resolve_layer, ConfSetting, Layer as PLayer, PenumbraFile, ConfOption, FileLayer, self}};
use super::Viewer;

pub struct Tex {
	ext: String,
	gamepath: String,
	rootpath: Option<PathBuf>,
	tex: Option<tex::Tex>,
	settings: HashMap<String, ConfSetting>,
	width: u16,
	height: u16,
	miplevel: i32,
	depth: i32,
	r: bool,
	g: bool,
	b: bool,
	a: bool,
	highlighted_layer: Option<usize>,
	new_layer: Option<FileLayer>,
	invalid_layers: Vec<(usize, String)>,
	cache: HashMap<String, Vec<u8>>,
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
	pub fn new(gamepath: String, conf: Option<super::Conf>) -> Self {
		let settings = if let Some(c) = &conf && let Some(f) = c.datas.penumbra.file_ref(&c.option, &c.sub_option, &gamepath) {
			let f = &f.0;
			let mut settings = HashMap::new();
			for i in 0..f.len() {
				let l = f.get(i).unwrap();
				if let Some(id) = &l.id && let Some(setting) = c.datas.penumbra.options.iter().find(|e| e.id() == Some(&id)) {
					settings.insert(id.clone(), setting.default());
				}
			}
			settings
		} else {
			HashMap::new()
		};
		
		let mut t = Tex {
			ext: format!(".{}", gamepath.split('.').last().unwrap()),
			gamepath: gamepath.clone(),
			rootpath: if let Some(c) = &conf {Some(c.path.clone())} else {None},
			tex: None,
			settings,
			miplevel: 0,
			depth: 0,
			width: 0,
			height: 0,
			r: true,
			g: true,
			b: true,
			a: true,
			highlighted_layer: None,
			new_layer: None,
			invalid_layers: Vec::new(),
			cache: HashMap::new(),
		};
		let f = t.penumfile(&conf);
		t.reload_preview(f);
		t
	}
	
	fn reload_preview(&mut self, file: PenumbraFile) {
		let hl = self.highlighted_layer;
		let mut layers = file.0.into_iter().enumerate();
		
		self.invalid_layers.clear();
		let mut tex: Option<tex::Tex> = None;
		while let Some((i, layer)) = layers.next() {
			let val = if let Some(id) = &layer.id {
				let setting = self.settings.get(id).cloned();
				if setting.is_none() {
					self.invalid_layers.push((i, format!("Invalid ID: {}", id)));
					continue;
				}
				setting
			} else {None};
			let mut get_file = |p: &str| -> Option<Vec<u8>> {self.get_file(p)};
			match resolve_layer(&PLayer{value: val, files: layer.paths}, &mut get_file) {
				Ok(mut l) => {
					if hl == Some(i) {Self::clrtex(&mut l)}
					match &mut tex {
						Some(tex) => l.overlay_onto(tex),
						None => tex = Some(l),
					}
				},
				Err(e) => self.invalid_layers.push((i, format!("Invalid Path: {}", e))),
			}
		}
		
		if let Some(tex) = tex {
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
	}
	
	fn penumfile(&mut self, conf: &Option<super::Conf>) -> PenumbraFile {
		if conf.is_some() && let Some(f) = conf.as_ref().unwrap().file_ref(&self.gamepath) {
			f.clone()
		} else {
			PenumbraFile(vec![FileLayer {
				id: None,
				paths: vec![self.gamepath.to_owned()],
			}])
		}
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
		let data = if let Some(root) = &self.rootpath && root.join(path).exists() {
			penumbra::load_file(root.join(path).to_str().unwrap())
		} else {
			penumbra::load_file(path)
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
	
	// TODO: use file in conf instead of a seperate clone of it
	fn draw(&mut self, _state: &mut crate::Data, mut conf: Option<super::Conf>) {
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
			let mut changed = false;
			if let Some(tex) = self.tex.as_ref() {
				let header = &tex.header;
				imgui::text(&format!("{:?}", header.format));
				
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
			}
			
			if let Some(conf) = &mut conf && let Some(f) = conf.file_mut(&self.gamepath) {
				// Layers
				aeth::offset([0.0, 20.0]);
				let mut hl = None;
				let mut rem = None;
				let mut layers = &mut f.0;
				let len = layers.len();
				if aeth::orderable_list("layers", &mut layers, |i, _| {
					// Removing the last layer isnt a great idea
					if len > 1 && imgui::button("Remove", [0.0, 0.0]) {
						rem = Some(i);
					}
				}, |i, layer| {
					if let Some(e) = self.invalid_layers.iter().find(|v| v.0 == i) {
						imgui::text(&e.1);
					} else {
						if let Some(id) = &layer.id {
							changed |= self.settings.get_mut(id).unwrap().draw(id);
						} else {
							imgui::text("Generic Layer");
						}
					}
					
					if imgui::is_item_hovered() {hl = Some(i);}
				}) {
					conf.save();
					changed = true;
				} else if let Some(i) = rem {
					layers.remove(i);
					conf.save();
					changed = true
				}
				
				if self.highlighted_layer != hl {
					self.highlighted_layer = hl;
					changed = true;
				}
				
				// New Layer
				if aeth::button_icon("", aeth::fa5()) { // fa-plus
					imgui::open_popup("newlayer", imgui::PopupFlags::None);
				}
				
				aeth::popup("newlayer", imgui::WindowFlags::None, || {
					if imgui::selectable("Generic", false, imgui::SelectableFlags::None, [0.0, 0.0]) {
						self.new_layer = Some(FileLayer {
							id: None,
							paths: vec![String::with_capacity(128)],
						});
						imgui::open_popup("newlayer2", imgui::PopupFlags::None);
					}
					
					for i in 0..conf.datas.penumbra.options.len() {
						let o = conf.datas.penumbra.options.get(i).unwrap();
						if !o.is_penumbra() && imgui::selectable(&format!("{} ({})", o.name(), o.id().unwrap()), false, imgui::SelectableFlags::None, [0.0, 0.0]) {
							self.new_layer = Some(FileLayer {
								id: Some(o.id().unwrap().to_owned()),
								paths: match o {
									ConfOption::Mask(_) => vec![String::with_capacity(128), String::with_capacity(128)],
									_ => vec![String::with_capacity(128)],
								},
							});
						}
					}
				});
				
				if self.new_layer.is_some() {
					let imports = self.valid_imports();
					let layer = self.new_layer.as_mut().unwrap();
					
					imgui::set_next_window_pos(imgui::get_cursor_screen_pos(), imgui::Cond::Always, [0.0, 0.0]);
					imgui::begin("##aetherment_newlayer", &mut true, imgui::WindowFlags::AlwaysAutoResize | /*imgui::WindowFlags::Popup |*/
					                                                 imgui::WindowFlags::NoSavedSettings | imgui::WindowFlags::NoTitleBar /*|
					                                                 imgui::WindowFlags::ChildWindow*/);
					imgui::bring_window_to_display_front(imgui::get_current_window());
					
					if layer.id.is_some() {
						match conf.datas.penumbra.options.iter().find(|f| f.id() == layer.id.as_deref()).unwrap() {
							ConfOption::Mask(_) => {
								let p = layer.paths.get_mut(0).unwrap();
								aeth::file_picker(aeth::FileDialogMode::OpenFile, "Import Image", "", imports.clone(), p);
								imgui::same_line();
								imgui::input_text("Image", p, imgui::InputTextFlags::None);
								
								let p = layer.paths.get_mut(1).unwrap();
								aeth::file_picker(aeth::FileDialogMode::OpenFile, "Import Mask", "", imports, p);
								imgui::same_line();
								imgui::input_text("Mask", p, imgui::InputTextFlags::None);
							},
							_ => {
								let p = layer.paths.get_mut(0).unwrap();
								aeth::file_picker(aeth::FileDialogMode::OpenFile, "Import Image", "", imports.clone(), p);
								imgui::same_line();
								imgui::input_text("Image", p, imgui::InputTextFlags::None);
							}
						}
					} else {
						let p = layer.paths.get_mut(0).unwrap();
						aeth::file_picker(aeth::FileDialogMode::OpenFile, "Import Image", "", imports.clone(), p);
						imgui::same_line();
						imgui::input_text("Image", p, imgui::InputTextFlags::None);
					}
					
					if imgui::button("Add", [0.0, 0.0]) && layer.paths.iter().all(|p| p.len() > 0) {
						for i in 0..layer.paths.len() {
							let gamepath = layer.paths.get_mut(i).unwrap();
							let path = Path::new(&gamepath);
							if path.exists() {
								let ext = path.extension().unwrap().to_str().unwrap();
								let mut buf = Vec::new();
								if !noumenon::convert(&mut File::open(&path).unwrap(), ext, &mut Cursor::new(&mut buf), &self.gamepath[self.gamepath.rfind('.').unwrap() + 1..].to_owned()) {
									log!("import failed"); // TODO: nice popup displaying that it failed
								}
								let hash = blake3::hash(&buf).to_hex().as_str()[..24].to_string();
								File::create(conf.path.join("files").join(&hash)).unwrap().write_all(&buf).unwrap();
								*gamepath = format!("files/{}", hash);
							}
						}
						
						conf.file_mut(&self.gamepath).unwrap().0.push(layer.clone());
						self.new_layer = None;
						conf.save();
						changed = true;
					}
					imgui::same_line();
					if imgui::button("Cancel", [0.0, 0.0]) {
						self.new_layer = None;
					}
					
					imgui::end();
				}
			}
			
			let f = self.penumfile(&mut conf);
			if changed {self.reload_preview(f)}
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