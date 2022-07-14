use std::{fs::File, path::PathBuf, collections::HashMap, sync::{Arc, Mutex}, io::{Write, Cursor}};
use crate::{gui::aeth::{self, F2}, GAME, apply::{self, penumbra::{ConfOption, ConfSetting, PenumbraFile, FileLayer}}};

mod tree;
mod viewer;
use serde_json::json;
use tree::Tree;

use self::viewer::Viewer;

pub struct Tab {
	refresh_mod: Arc<Mutex<bool>>,
	curmod: Option<Mod>,
	selected_mod: String,
	
	populated_modselect: bool,
	mod_entries: Vec<String>,
	
	newopt: String,
	
	gametree: Tree,
	viewer: Option<Arc<Mutex<dyn Viewer + Send>>>,
	path: String,
	valid_path: bool,
}

impl Tab {
	pub fn new(state: &mut crate::Data) -> Self {
		Tab {
			refresh_mod: Arc::new(Mutex::new(false)),
			curmod: None,
			selected_mod: "".to_owned(),
			
			populated_modselect: false,
			mod_entries: Vec::new(),
			
			newopt: String::with_capacity(32),
			
			// TODO: thread it or smth, its super slow
			// gametree: Tree::from_file("Game Files", state.binary_path.join("assets").join("paths")).unwrap(),
			gametree: Tree::new("Game Files"),
			viewer: None,
			path: String::with_capacity(128),
			valid_path: false,
		}
	}
	
	pub fn draw(&mut self, state: &mut crate::Data) {
		let mut refresh = self.refresh_mod.lock().unwrap();
		if *refresh {
			// refresh mod, probably wanna simply add tree entry and refresh viewer instead. TODO: this
			*refresh = false;
			drop(refresh);
			let m = self.curmod.as_ref().unwrap();
			File::create(m.path.join("datas.json")).unwrap().write_all(crate::serialize_json(json!(*m.datas.lock().unwrap())).as_bytes()).unwrap();
			self.load_mod(&self.selected_mod.clone(), m.path.clone());
		} else {
			drop(refresh);
		}
		
		aeth::divider("explorer_div", false)
		.left(100.0, || {
			aeth::child("trees", [0.0, -(aeth::frame_height() + imgui::get_style().item_spacing.y()) * if self.curmod.is_some() {2.0} else {1.0}], false, imgui::WindowFlags::None, || {
				if let Some(m) = self.curmod.as_mut() {
					aeth::popup("modfilecontext", imgui::WindowFlags::None, || {
						if imgui::button("Remove", [0.0, 0.0]) {
							m.datas.lock().unwrap().penumbra.update_file(&m.opt, &m.subopt, &self.path, None);
							*self.refresh_mod.lock().unwrap() = true;
							imgui::close_current_popup();
						}
					});
					
					if let Some((button, path)) = m.tree.draw() {
						if button == imgui::MouseButton::Right {
							imgui::open_popup("modfilecontext", imgui::PopupFlags::MouseButtonRight);
						} else {
							self.open_file_mod(path);
						}
					}
				}
				
				if let Some((_button, path)) = self.gametree.draw() {
					self.open_file(path);
				}
			});
			
			if let Some(m) = &self.curmod {
				aeth::next_max_width();
				// aeth::combo("##optionselect", &format!("{}/{}", m.opt, m.subopt), imgui::ComboFlags::None, || {
				if imgui::begin_combo("##optionselect", &format!("{}/{}", m.opt, m.subopt), imgui::ComboFlags::None) { // scoped kinda sucks cuz closures suck
					let mut a = if imgui::selectable("Default", m.opt == "" && m.subopt == "", imgui::SelectableFlags::None, [0.0, 0.0]) {Some(("".to_owned(), "".to_owned()))} else {None};
					m.datas.lock().unwrap().penumbra.options.iter_mut().for_each(|o| if let ConfOption::Multi(opt) | ConfOption::Single(opt) = o {
						aeth::tree(&opt.name, || {
							opt.options.iter().for_each(|o2| if imgui::selectable(&o2.name, m.opt == opt.name && m.subopt == o2.name, imgui::SelectableFlags::None, [0.0, 0.0]) {
								a = Some((opt.name.clone(), o2.name.clone()));
							});
							
							if aeth::button_icon("", state.fa5) { // fa-plus
								opt.options.push(apply::penumbra::PenumbraOption {
									name: self.newopt.clone(),
									files: HashMap::new(),
									swaps: HashMap::new(),
									manipulations: Vec::new(),
								});
								self.newopt.clear();
								*self.refresh_mod.lock().unwrap() = true;
							}
							imgui::same_line();
							aeth::next_max_width();
							imgui::input_text_with_hint("##newopt", "New Sub Option", &mut self.newopt, imgui::InputTextFlags::None);
						});
					});
					
					aeth::popup("addoptselect", imgui::WindowFlags::None, || {
						if imgui::button("Single", [0.0, 0.0]) {
							m.datas.lock().unwrap().penumbra.options.push(ConfOption::Single(apply::penumbra::TypPenumbra {
								name: self.newopt.clone(),
								description: "".to_owned(), // TODO: editable in mod overview or smth
								options: Vec::new(),
							}));
							self.newopt.clear();
							*self.refresh_mod.lock().unwrap() = true;
							imgui::close_current_popup();
						}
						
						if imgui::button("Multi", [0.0, 0.0]) {
							m.datas.lock().unwrap().penumbra.options.push(ConfOption::Multi(apply::penumbra::TypPenumbra {
								name: self.newopt.clone(),
								description: "".to_owned(), // TODO: editable in mod overview or smth
								options: Vec::new(),
							}));
							self.newopt.clear();
							*self.refresh_mod.lock().unwrap() = true;
							imgui::close_current_popup();
						}
					});
					
					if aeth::button_icon("", state.fa5) && self.newopt.len() > 0 { // fa-plus
						imgui::open_popup("addoptselect", imgui::PopupFlags::MouseButtonLeft);
					}
					imgui::same_line();
					aeth::next_max_width();
					imgui::input_text_with_hint("##newopt", "New Option", &mut self.newopt, imgui::InputTextFlags::None);
					
					if let Some(o) = a {self.set_mod_option(o.0, o.1);}
					
					imgui::end_combo();
				}
				// });
			}
			
			aeth::next_max_width();
			self.populated_modselect = aeth::combo("##modselect", &self.selected_mod.clone(), imgui::ComboFlags::None, || {
				if !self.populated_modselect {
					self.mod_entries = std::fs::read_dir(&state.config.local_path)
						.unwrap()
						.into_iter()
						.filter_map(|e| {
							let e = e.unwrap();
							if e.metadata().unwrap().is_dir() {Some(e.file_name().to_str().unwrap().to_owned())} else {None}
						})
						.collect()
				}
				
				for m in self.mod_entries.clone().into_iter() { // just clone it, idc anymore, it just some small strings
					if imgui::selectable(&m, self.selected_mod == m, imgui::SelectableFlags::None, [0.0, 0.0]) {
						self.load_mod(&m, PathBuf::from(&state.config.local_path).join(&m));
					}
				}
			});
		}).right(400.0, || {
			aeth::child("viewer", [0.0, -aeth::frame_height() - imgui::get_style().item_spacing.y()], false, imgui::WindowFlags::None, || {
				if let Some(viewer) = self.viewer.as_mut() {
					match &self.curmod {
						Some(m) => viewer.lock().unwrap().draw(state, Some(&mut m.datas.lock().unwrap().penumbra)),
						None => viewer.lock().unwrap().draw(state, None),
					}
				}
			});
			
			if self.viewer.is_some() {
				if self.curmod.is_some() {
					if imgui::button("Import", [0.0, 0.0]) {self.import()}
					imgui::same_line();
				}
				
				if imgui::button("Export", [0.0, 0.0]) {self.export()}
				imgui::same_line();
			}
			
			let err = !self.valid_path;
			if err {imgui::push_style_color(imgui::Col::FrameBg, 0xFF3030B0)}
			imgui::set_next_item_width(aeth::width_left());
			if imgui::input_text("##path", &mut self.path, imgui::InputTextFlags::None) {
				let p = self.path.clone();
				if !self.open_file_mod(&p) {
					if !self.open_file(&p) {
						self.viewer = None;
					}
				}
			}
			if err {imgui::pop_style_color(1)}
		});
	}
	
	fn load_mod(&mut self, m: &str, path: PathBuf) {
		self.selected_mod = m.to_owned();
		
		let mut tree = Tree::new(m);
		let datas: apply::Datas = serde_json::from_reader(File::open(path.join("datas.json")).unwrap()).unwrap();
		
		datas.penumbra.files.keys()
			.for_each(|p| tree.add_node(p));
		
		datas.penumbra.options.iter()
			.for_each(|o| if let ConfOption::Multi(opt) | ConfOption::Single(opt) = o {
				opt.options.iter()
					.for_each(|s| s.files.keys()
						.for_each(|p| tree.add_node(p)))
			});
		
		self.curmod = Some(Mod {
			tree,
			datas: Arc::new(Mutex::new(datas)),
			path,
			opt: "".to_owned(),
			subopt: "".to_owned(),
		});
		
		self.set_mod_option("", "");
	}
	
	fn set_mod_option<S>(&mut self, opt: S, sub: S) where
	S: Into<String> {
		let m = self.curmod.as_mut().unwrap();
		m.opt = opt.into();
		m.subopt = sub.into();
		m.tree.node_state_all(false);
		
		if m.opt == "" && m.subopt == "" {
			m.datas.lock().unwrap().penumbra.files.keys()
				.for_each(|p| m.tree.node_state(p, true));
		} else {
			m.datas.lock().unwrap().penumbra.options.iter()
				.for_each(|o| if let ConfOption::Multi(opt) | ConfOption::Single(opt) = o && opt.name == m.opt {
					opt.options.iter()
						.filter(|s| s.name == m.subopt)
						.for_each(|s| s.files.keys()
							.for_each(|p| m.tree.node_state(p, true)))
				});
		}
		
		let p = self.path.clone();
		if !self.open_file_mod(&p) {
			self.open_file(&p);
		}
	}
	
	fn open_file<S>(&mut self, path: S) -> bool where
	S: Into<String> {
		self.path = path.into().to_lowercase();
		self.valid_path = valid_path(&self.path);
		if !self.valid_path {return false;}
		
		let ext = self.path.split('.').last().unwrap().to_owned();
		let path = self.path.clone();
		self.open_viewer(&ext, &path, None, None, None);
		
		true
	}
	
	fn open_file_mod<S>(&mut self, path: S) -> bool where
	S: Into<String> {
		self.path = path.into().to_lowercase();
		self.valid_path = valid_path_mod(&self.curmod, &self.path);
		if !self.valid_path {return false;}
		
		let ext = self.path.split('.').last().unwrap().to_owned();
		let path = self.path.clone();
		let m = self.curmod.as_mut().unwrap();
		let datas = m.datas.lock().unwrap();
		let paths = if m.opt == "" {
			datas.penumbra.files.get(&path).unwrap()
		} else {
			datas.penumbra.options.iter()
				.find_map(|o| if let ConfOption::Multi(opt) | ConfOption::Single(opt) = o && opt.name == m.opt {
					Some(opt.options.iter()
						.find_map(|o| if o.name == m.subopt {Some(o)} else {None})
						.unwrap()
						.files.get(&path)
						.unwrap()
					)} else {None})
				.unwrap()
		};
		let paths = paths.clone();
		let rootpath = Some(m.path.clone());
		let mut settings = HashMap::new();
		for l in paths.0.iter() {
			if let Some(id) = &l.id {
				settings.insert(id.clone(), datas.penumbra.options.iter().find(|e| e.id() == Some(&id)).unwrap().default());
			}
		}
		// paths.push(paths[1].clone());
		drop(datas);
		self.open_viewer(&ext, &path, rootpath, Some(paths), Some(settings));
		
		true
	}
	
	fn open_viewer(&mut self, ext: &str, path: &str, rootpath: Option<PathBuf>, realpaths: Option<PenumbraFile>, settings: Option<HashMap<String, ConfSetting>>) {
		self.viewer = match ext {
			"tex" | "atex" => Some(Arc::new(Mutex::new(viewer::Tex::new(path.to_owned(), rootpath, realpaths, settings)))),
			_ => Some(Arc::new(Mutex::new(viewer::Generic::new(path.to_owned(), rootpath, realpaths, settings)))),
		};
	}
	
	fn import(&self) {
		// TODO: allow importing into invalid paths
		let viewer = self.viewer.as_ref().unwrap().clone();
		let curmod = self.curmod.as_ref().unwrap();
		let gamepath = self.path.clone();
		let modpath = curmod.path.clone();
		let datas = curmod.datas.clone();
		let opt = curmod.opt.clone();
		let subopt = curmod.subopt.clone();
		let refresh_mod = self.refresh_mod.clone();
		
		std::thread::spawn(move || {
			let exts = viewer.lock().unwrap().valid_imports();
			if let Some(path) = crate::file_dialog(0, "Import".to_owned(), exts, "".to_owned()) {
				log!("import {:?}", path);
				let ext = path.extension().unwrap().to_str().unwrap();
				let mut buf = Vec::new();
				if !noumenon::convert(&mut File::open(&path).unwrap(), ext, &mut Cursor::new(&mut buf), &gamepath[gamepath.rfind('.').unwrap() + 1..].to_owned()) {
					log!("import failed"); // TODO: nice popup displaying that it failed
				}
				let hash = blake3::hash(&buf).to_hex().as_str()[..24].to_string();
				File::create(modpath.join("files").join(&hash)).unwrap().write_all(&buf).unwrap();
				let file = PenumbraFile(vec![FileLayer {
					id: None,
					paths: vec![format!("files/{}", &hash)],
				}]);
				datas.lock().unwrap().penumbra.update_file(&opt, &subopt, &gamepath, Some(file));
				*refresh_mod.lock().unwrap() = true;
			}
		});
	}
	
	fn export(&self) {
		let viewer = self.viewer.as_ref().unwrap().clone();
		let name = self.path[self.path.rfind('/').unwrap() + 1..self.path.rfind('.').unwrap()].to_owned();
		std::thread::spawn(move || {
			let exts = viewer.lock().unwrap().valid_exports();
			if let Some(path) = crate::file_dialog(1, format!("Export {}", &name), exts, name) {
				log!("export {:?}", path);
				let ext = path.extension().unwrap().to_str().unwrap();
				let mut buf = Vec::new();
				viewer.lock().unwrap().save(ext, &mut buf);
				File::create(&path).unwrap().write_all(&buf).unwrap();
			}
		});
	}
}

struct Mod {
	tree: Tree,
	datas: Arc<Mutex<apply::Datas>>,
	path: PathBuf,
	opt: String,
	subopt: String,
}

impl Mod {
	
}

// ---------------------------------------- //

struct NullWriter();
impl ironworks::file::File for NullWriter {
	fn read<'a>(_: impl Into<std::borrow::Cow<'a, [u8]>>) -> noumenon::formats::game::Result<Self> {
		Ok(NullWriter())
	}
}

fn valid_path(path: &str) -> bool {
	// TODO: add way to check if a path is valid to ironworks, this is stupid
	GAME.file::<NullWriter>(path).is_ok()
}

fn valid_path_mod(m: &Option<Mod>, path: &str) -> bool {
	(|| {
		if let Some(m) = m {
			if m.opt != "" {
				m.datas.lock().unwrap().penumbra.options.iter()
					.find_map(|o| if let ConfOption::Multi(opt) | ConfOption::Single(opt) = o && opt.name == m.opt {
						Some(opt.options.iter().find_map(|o| if o.name == m.subopt {Some(o)} else {None})?)
					} else {
						None
					})?.files.contains_key(path).then(|| ())
			} else {
				m.datas.lock().unwrap().penumbra.files.contains_key(path).then(|| ())
			}
		} else {
			None
		}
	})().is_some()
}