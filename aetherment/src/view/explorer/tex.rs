use std::collections::HashMap;
use noumenon::format::game::Tex as GameTex;
use crate::{resource_loader::{load_file, load_file_disk}, render_helper::{RendererExtender, EnumTools}, modman::{composite::tex::{OptionOrStatic, OptionSetting}, Path}};

// TODO: saving
// TODO: way to import textures
// TODO: perhabs have a simple settings menu to preview with different than default settings
// TODO: actually test its functionality

pub struct Tex {
	name: String,
	path: String,
	real_path: Option<String>,
	data: Option<Data>,
	texture: egui::TextureHandle,
	// width: f32,
	// height: f32,
	// uv: egui::Rect,
	
	mip: u16,
	depth: u16,
	r: bool,
	g: bool,
	b: bool,
	a: bool,
	
	mod_: Option<super::Mod>,
	views: egui_dock::Tree<Box<dyn Tab>>,
	ctx: egui::Context,
}

enum Data {
	// Tex(GameTex),
	Tex {
		data: GameTex,
		width: f32,
		height: f32,
		uv: egui::Rect,
	},
	
	Composite(Composite),
}

struct Composite {
	textures: HashMap<Path, Option<GameTex>>,
	comp: crate::modman::composite::tex::Tex,
	edit_layer: Option<usize>,
}

impl Tex {
	pub fn new(ctx: egui::Context, path: &str, real_path: Option<&str>, mod_: Option<super::Mod>) -> Result<Self, super::BacktraceError> {
		// if path ends with .comp its a composite texture
		
		let mut views: egui_dock::Tree<Box<dyn Tab>> = egui_dock::Tree::new(vec![Box::new(Preview)]);
		views.split_right(egui_dock::NodeIndex::root(), 0.8, vec![Box::new(CompositeEditor)]);
		
		let mut v = Self {
			name: path.split("/").last().unwrap().to_owned(),
			path: path.to_owned(),
			real_path: real_path.map(|v| v.to_owned()),
			data: None,
			texture: ctx.load_texture("explorer_tex", egui::epaint::image::ColorImage::new([1, 1], egui::epaint::Color32::TRANSPARENT), Default::default()),
			// width: 1.0,
			// height: 1.0,
			// uv: egui::Rect{min: egui::pos2(0.0, 0.0), max: egui::pos2(1.0, 1.0)},
			
			mip: 0,
			depth: 0,
			r: true,
			g: true,
			b: true,
			a: true,
			
			mod_,
			views,
			ctx: ctx.clone(),
		};
		
		if path.ends_with(".comp") {
			v.load_composite_data()?;
		} else {
			v.load_texture_data()?;
		}
		
		Ok(v)
	}
	
	fn reload_texture(&mut self, w: usize, h: usize) {
		if self.texture.size()[0] != w || self.texture.size()[1] != h {
			self.texture = self.ctx.load_texture("explorer_tex", egui::epaint::image::ColorImage::new([w, h], egui::epaint::Color32::TRANSPARENT), Default::default());
		}
	}
	
	fn load_texture_data(&mut self) -> Result<(), super::BacktraceError> {
		let data = load_file::<GameTex>(&self.path, self.real_path.as_deref())?;
		self.reload_texture(data.header.width as usize, data.header.height as usize);
		
		// self.width = data.header.width as f32;
		// self.height = data.header.height as f32;
		// self.data = Some(Data::Tex(data));
		self.data = Some(Data::Tex {
			width: data.header.width as f32,
			height: data.header.height as f32,
			data,
			uv: egui::Rect{min: egui::pos2(0.0, 0.0), max: egui::pos2(1.0, 1.0)},
		});
		self.refresh_texture();
		
		Ok(())
	}
	
	fn load_composite_data(&mut self) -> Result<(), super::BacktraceError> {
		self.data = Some(Data::Composite(Composite {
			textures: HashMap::new(),
			comp: serde_json::from_reader(std::io::BufReader::new(std::fs::File::open(self.real_path.as_ref().ok_or("no real path")?)?))?,
			edit_layer: None,
		}));
		self.refresh_texture();
		
		Ok(())
	}
	
	fn convert_to_composite(&mut self) {
		use crate::modman::composite::tex::*;
		
		let Some(mod_) = &self.mod_ else {return};
		let Some(real_path) = &self.real_path else {return};
		// let Some(Data::Tex(data)) = self.data.take() else {return};
		let Some(Data::Tex{data, ..}) = self.data.take() else {return};
		
		let path = Path::Mod(real_path.split(['/', '\\']).last().unwrap().to_owned());
		self.real_path = Some(std::path::Path::new(real_path).strip_prefix(&mod_.path).unwrap().to_string_lossy().to_string());
		self.data = Some(Data::Composite(Composite {
			textures: HashMap::from([
				(path.clone(), Some(data))
			]),
			comp: Tex {
				layers: vec![Layer {
					name: "New layer".to_owned(),
					path,
					modifiers: vec![],
					blend: Blend::Normal,
				}],
			},
			edit_layer: None,
		}));
		
		self.refresh_texture();
	}
	
	fn refresh_texture(&mut self) {
		if let Some(Data::Composite(comp)) = &self.data {
			let Some(tex) = comp.textures.iter().last() else {return};
			let Some(tex) = tex.1 else {return};
			self.reload_texture(tex.header.width as usize, tex.header.height as usize);
		}
		
		match &mut self.data {
			Some(Data::Tex{data, uv, ..}) => {
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
					self.texture.set_partial([0; 2], egui::ColorImage::from_rgba_unmultiplied([w as usize, h as usize], &slice), Default::default());
					uv.max = egui::pos2(w as f32 / data.header.width as f32, h as f32 / data.header.height as f32);
				}
			}
			
			Some(Data::Composite(comp)) => {
				let Some(tex) = comp.textures.iter().last() else {return};
				let Some(tex) = tex.1 else {return};
				// self.reload_texture(tex.header.width as usize, tex.header.height as usize);
				
				let mod_ = self.mod_.as_ref().unwrap();
				let settings = crate::modman::settings::Settings::from_meta(&mod_.meta.borrow());
				
				let mut textures = HashMap::new();
				for (path, tex) in &comp.textures {
					let Some(tex) = tex else {return};
					textures.insert(path, tex);
				}
				
				let Some(data) = comp.comp.composite_hashmap(&settings, textures) else {return};
				self.texture.set_partial([0; 2], egui::ColorImage::from_rgba_unmultiplied([tex.header.width as usize, tex.header.height as usize], &data), Default::default());
			}
			
			_ => {}
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
		vec![self.name.split(".").last().unwrap(), "dds", "png", "tiff", "tga"]
	}
	
	fn render(&mut self, ui: &mut egui::Ui) -> Result<(), super::BacktraceError> {
		let mut refresh_texture = false;
		
		match &mut self.data {
			Some(Data::Tex{..}) => {
				if let Err(err) = Preview.render(ui, &mut Tabs {
					texture: &mut self.texture,
					// width: &mut self.width,
					// height: &mut self.height,
					// uv: &mut self.uv,
					// composite: None,
					data: self.data.as_mut().unwrap(),
					mod_: self.mod_.as_mut(),
					refresh_texture: &mut refresh_texture,
				}) {
					super::render_error(ui, &err);
				}
			}
			
			Some(Data::Composite(_)) => {
				egui_dock::DockArea::new(&mut self.views)
					.id(egui::Id::new(&self.name))
					.style(egui_dock::Style::from_egui(ui.style().as_ref()))
					.show_close_buttons(false)
					.tab_context_menus(false)
					.show_inside(ui, &mut Tabs {
						texture: &mut self.texture,
						// width: &mut self.width,
						// height: &mut self.height,
						// uv: &mut self.uv,
						// composite: Some(comp),
						data: self.data.as_mut().unwrap(),
						mod_: self.mod_.as_mut(),
						refresh_texture: &mut refresh_texture,
					});
			}
			
			_ => {}
		}
		
		if refresh_texture {
			self.refresh_texture();
		}
		
		Ok(())
	}
	
	fn render_options(&mut self, ui: &mut egui::Ui) -> Result<(), super::BacktraceError> {
		let mut changed = false;
		
		// if let Some(Data::Tex(data)) = &self.data.as_ref() {
		if let Some(Data::Tex{data, ..}) = &self.data.as_ref() {
			changed |= ui.add(egui::Slider::new(&mut self.mip, 0..=(data.header.mip_levels - 1)).text("Mip Level")).changed();
			changed |= ui.add(egui::Slider::new(&mut self.depth, 0..=(data.header.depths - 1)).text("Depth")).changed();
		}
		
		ui.horizontal(|ui| {
			changed |= ui.checkbox(&mut self.r, "R").changed();
			changed |= ui.checkbox(&mut self.g, "G").changed();
			changed |= ui.checkbox(&mut self.b, "B").changed();
			changed |= ui.checkbox(&mut self.a, "A").changed();
		});
		
		if let Some(Data::Tex{..}) = &self.data {
			if self.real_path.is_some() && ui.button("Convert to composite texture").clicked() {
				self.convert_to_composite();
			}
		}
		
		if changed {
			self.refresh_texture()
		}
		
		Ok(())
	}
	
	fn export(&self, ext: &str, mut writer: Box<dyn super::Writer>) -> Result<(), super::BacktraceError> {
		match &self.data {
			Some(Data::Tex{data, ..}) => {
				noumenon::Convert::Tex(data.clone()).convert(ext, &mut writer)?;
				Ok(())
			}
			
			Some(Data::Composite(_comp)) => {
				log!("TODO: export composite tex");
				Ok(())
			}
			
			_ => Err(super::ExplorerError::Data.into())
		}
	}
}

fn reload_textures(textures: &mut HashMap<Path, Option<GameTex>>, comp: &crate::modman::composite::tex::Tex, meta: &crate::modman::meta::Meta, settings: &crate::modman::settings::Settings, files_root: &std::path::Path) -> Result<(), super::BacktraceError> {
	use crate::modman::composite::tex::Modifier;
	
	textures.clear();
	log!("Reloading textures");
	let mut add_texture = |path: &Path| -> Result<(), super::BacktraceError> {
		match path {
			Path::Mod(path) => {
				let tex = load_file_disk::<GameTex>(&files_root.join(path))?;
				textures.insert(Path::Mod(path.to_owned()), Some(tex));
			}
			
			Path::Game(path) => {
				let tex = load_file::<GameTex>(&path, None)?;
				textures.insert(Path::Game(path.to_owned()), Some(tex));
			}
			
			Path::Option(id) => {
				if let Some(setting) = settings.get(id) {
					if let crate::modman::settings::Value::Path(i) = setting {
						if let Some(option) = meta.options.iter().find(|v| v.name == *id) {
							if let crate::modman::meta::OptionSettings::Path(v) = &option.settings {
								if let Some((_, path)) = v.options.get(*i as usize) {
									match path {
										Path::Mod(path) => {
											let tex = load_file_disk::<GameTex>(&files_root.join(path))?;
											textures.insert(Path::Mod(path.to_owned()), Some(tex));
										}
										
										Path::Game(path) => {
											let tex = load_file::<GameTex>(&path, None)?;
											textures.insert(Path::Game(path.to_owned()), Some(tex));
										}
										
										_ => {}
									}
								}
							}
						}
					}
				}
			}
		}
		
		Ok(())
	};
	
	for layer in &comp.layers {
		add_texture(&layer.path)?;
		
		for modifier in &layer.modifiers {
			match modifier {
				Modifier::AlphaMask{path, ..} => {
					add_texture(path)?;
				}
				
				_ => {}
			}
		}
	}
	
	Ok(())
}

// ---------------------------------------- //

struct Tabs<'a> {
	texture: &'a egui::TextureHandle,
	// width: &'a f32,
	// height: &'a f32,
	// uv: &'a egui::Rect,
	// composite: Option<&'a mut Composite>,
	data: &'a mut Data,
	mod_: Option<&'a mut super::Mod>,
	refresh_texture: &'a mut bool,
}

impl<'a> egui_dock::TabViewer for Tabs<'a> {
	type Tab = Box<dyn Tab>;
	
	fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
		tab.name().into()
	}
	
	fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
		if let Err(err) = tab.render(ui, self) {
			super::render_error(ui, &err);
		}
	}
}

// ---------------------------------------- //

trait Tab {
	fn name(&self) -> &str;
	fn render(&mut self, ui: &mut egui::Ui, data: &mut Tabs) -> Result<(), super::BacktraceError>;
}

struct Preview;
impl Tab for Preview {
	fn name(&self) -> &str {
		"Preview"
	}
	
	fn render(&mut self, ui: &mut egui::Ui, data: &mut Tabs) -> Result<(), super::BacktraceError> {
		let Data::Tex{width, height, uv, ..} = data.data else {return Ok(())};
		let space = ui.available_size();
		
		if data.texture.size()[0] != 0 {
			let scale = (space.x / *width).min(space.y / *height);
			
			// TODO: checkerboard background
			ui.vertical_centered_justified(|ui| {
				ui.centered_and_justified(|ui| {
					ui.add(egui::Image::new(data.texture, egui::vec2(*width * scale, *height * scale)).uv(*uv));
				})
			});
		}
		
		Ok(())
	}
}

struct CompositeEditor;
impl Tab for CompositeEditor {
	fn name(&self) -> &str {
		"Composite"
	}
	
	fn render(&mut self, ui: &mut egui::Ui, data: &mut Tabs) -> Result<(), super::BacktraceError> {
		use crate::modman::composite::tex::*;
		
		let Some(mod_) = &mut data.mod_ else {return Ok(())};
		let meta = mod_.meta.borrow_mut();
		// let Some(comp) = &mut data.composite else {return Ok(())};
		let Data::Composite(comp) = data.data else {return Ok(())};
		let edit_layer = &mut comp.edit_layer;
		let textures = &mut comp.textures;
		let comp = &mut comp.comp;
		
		let mut delete = None;
		egui_dnd::dnd(ui, "layers").show_vec(&mut comp.layers, |ui, option, handle, state| {
			handle.ui(ui, |ui| {
				// TODO: small preview of the layer here
				ui.button(&option.name).context_menu(|ui| {
					if ui.button("Edit layer").clicked() {
						*edit_layer = Some(state.index);
						ui.close_menu();
					}
					
					ui.separator();
					if ui.delete_button("Delete layer").clicked() {
						*edit_layer = None;
						delete = Some(state.index);
						ui.close_menu();
					}
				});
			});
		});
		
		if let Some(i) = delete {
			comp.layers.remove(i);
		}
		
		if ui.button("➕ Add layer").clicked() {
			comp.layers.push(Layer {
				name: "New layer".to_string(),
				path: Path::Mod(String::new()),
				modifiers: vec![],
				blend: Blend::Normal,
			});
		}
		
		let mut open = edit_layer.is_some();
		egui::Window::new("Edit layer")
			.open(&mut open)
			.show(ui.ctx(), |ui| {
				let Some(edit_layer_index) = *edit_layer else {return};
				let layer = &mut comp.layers[edit_layer_index];
				
				ui.label("Name");
				ui.text_edit_singleline(&mut layer.name);
				
				ui.add_space(10.0);
				
				ui.label("Path");
				ui.enum_combo_id(&mut layer.path, "path");
				render_path(ui, &mut layer.path, &meta);
				
				ui.add_space(10.0);
				
				ui.label("Modifiers");
				let mut delete = None;
				egui_dnd::dnd(ui, "modifiers").show_vec(&mut layer.modifiers, |ui, modifier, handle, state| {
					ui.dnd_header(handle, |ui| {
						ui.label(modifier.to_str());
					});
					
					ui.indent(state.index, |ui| {
						ui.enum_combo(modifier, "Type");
						
						match modifier {
							Modifier::AlphaMask{path, cull_point} => {
								render_path(ui, path, &meta);
								render_modifier(ui, cull_point, &meta, |ui, v| {
									ui.num_edit_range(v, "Cull point", 0.0..=1.0);
								});
							}
							
							Modifier::Color{value} => {
								render_modifier(ui, value, &meta, |ui, v| {
									ui.num_multi_edit_range(v, "Color", &[0.0..=1.0, 0.0..=1.0, 0.0..=1.0, 0.0..=1.0]);
								});
							}
						}
						
						if ui.delete_button("Delete modifier").clicked() {
							delete = Some(state.index);
						}
					});
				});
		
				if let Some(i) = delete {
					layer.modifiers.remove(i);
				}
				
				if ui.button("➕ Add modifier").clicked() {
					layer.modifiers.push(Modifier::Color{value: OptionOrStatic::Static([1.0, 1.0, 1.0, 1.0])});
				}
				
				ui.add_space(10.0);
				
				ui.label("Blend");
				ui.enum_combo_id(&mut layer.blend, "blend");
			});
		
		if !open {
			*edit_layer = None;
			*data.refresh_texture = true;
			reload_textures(textures, comp, &meta, &crate::modman::settings::Settings::from_meta(&meta), &mod_.path)?;
		}
		
		Ok(())
	}
}

fn render_path(ui: &mut egui::Ui, path: &mut Path, meta: &crate::modman::meta::Meta) {
	match path {
		Path::Mod(s) => {
			ui.text_edit_singleline(s);
			// TODO: validation
		}
		
		Path::Game(s) => {
			ui.text_edit_singleline(s);
		}
		
		Path::Option(s) => {
			egui::ComboBox::from_label("Option")
				.selected_text(s.as_str())
				.show_ui(ui, |ui| {
					for item in &meta.options {
						if let crate::modman::meta::OptionSettings::Path(_) = &item.settings {
							ui.selectable_value(s, item.name.clone(), &item.name);
						}
					}
				});
		}
	}
}

fn render_modifier<S: OptionSetting + Default + PartialEq>(ui: &mut egui::Ui, option: &mut OptionOrStatic<S>, meta: &crate::modman::meta::Meta, edit: impl FnOnce(&mut egui::Ui, &mut S::Value)) {
	ui.enum_combo_id(option, "type");
	
	match option {
		OptionOrStatic::Option(v) => {
			egui::ComboBox::from_label("Option")
				.selected_text(v.option_id())
				.show_ui(ui, |ui| {
					for item in &meta.options {
						if v.valid_option(&item.settings) {
							ui.selectable_value(v.option_id_mut(), item.name.clone(), &item.name);
						}
					}
				});
		}
		
		OptionOrStatic::Static(v) => edit(ui, v),
	}
}