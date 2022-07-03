use std::{fs::File, path::PathBuf, collections::HashMap};

use crate::{gui::{imgui, aeth::{self, F2}}, GAME, apply::{self, penumbra::{ConfOption, ConfSetting}}};

mod tree;
mod viewer;
use tree::Tree;

use self::viewer::Viewer;

pub struct Tab {
	curmod: Option<Mod>,
	selected_mod: String,
	
	populated_modselect: bool,
	mod_entries: Vec<String>,
	
	gametree: Tree,
	viewer: Option<Box<dyn Viewer>>,
	path: String,
	valid_path: bool,
}

struct Mod {
	tree: Tree,
	datas: apply::Datas,
	path: PathBuf,
	opt: String,
	subopt: String,
}

impl Tab {
	pub fn new(state: &mut crate::Data) -> Self {
		Tab {
			curmod: None,
			selected_mod: "".to_owned(),
			
			populated_modselect: false,
			mod_entries: Vec::new(),
			
			gametree: Tree::from_file("Game Files", state.binary_path.join("assets").join("paths")).unwrap(),
			viewer: None,
			path: String::with_capacity(128),
			valid_path: false,
		}
	}
	
	pub fn draw(&mut self, state: &mut crate::Data) {
		aeth::child("left", [300.0, -imgui::get_style().item_spacing.y()], false, imgui::WindowFlags::None, || {
			aeth::child("trees", [0.0, -(aeth::frame_height() + imgui::get_style().item_spacing.y()) * if self.curmod.is_some() {2.0} else {1.0}], false, imgui::WindowFlags::None, || {
				if let Some(m) = self.curmod.as_mut() && let Some(path) = m.tree.draw() {
					self.open_file_mod(path);
				}
				
				if let Some(path) = self.gametree.draw() {
					self.open_file(path);
				}
			});
			
			if let Some(m) = &self.curmod {
				imgui::set_next_item_width(aeth::width_left());
				// aeth::combo("##optionselect", &format!("{}/{}", m.opt, m.subopt), imgui::ComboFlags::None, || {
				if imgui::begin_combo("##optionselect", &format!("{}/{}", m.opt, m.subopt), imgui::ComboFlags::None) { // scoped kinda sucks cuz closures suck
					let mut a = if imgui::selectable("Default", m.opt == "" && m.subopt == "", imgui::SelectableFlags::None, [0.0, 0.0]) {Some(("".to_owned(), "".to_owned()))} else {None};
					m.datas.penumbra.options.iter().for_each(|o| if let ConfOption::Multi(opt) | ConfOption::Single(opt) = o {
						aeth::tree(&opt.name, || {
							opt.options.iter().for_each(|o2| if imgui::selectable(&o2.name, m.opt == opt.name && m.subopt == o2.name, imgui::SelectableFlags::None, [0.0, 0.0]) {
								a = Some((opt.name.clone(), o2.name.clone()));
							});
						});
					});
					if let Some(o) = a {
						self.set_mod_option(o.0, o.1);
						
						let p = self.path.clone();
						if !self.open_file_mod(&p) {
							self.open_file(&p);
						}
					}
					imgui::end_combo();
				}
				// });
			}
			
			imgui::set_next_item_width(aeth::width_left());
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
		});
		
		imgui::same_line();
		aeth::child("right", [0.0, -imgui::get_style().item_spacing.y()], false, imgui::WindowFlags::None, || {
			aeth::child("viewer", [0.0, -aeth::frame_height() - imgui::get_style().item_spacing.y()], false, imgui::WindowFlags::None, || {
				if let Some(viewer) = self.viewer.as_mut() {
					viewer.draw(state);
				}
			});
			
			let err = !self.valid_path;
			if err {imgui::push_style_color(imgui::Col::FrameBg, 0xFF3030B0)}
			imgui::set_next_item_width(aeth::width_left());
			if imgui::input_text("##path", &mut self.path, imgui::InputTextFlags::None) {
				let p = self.path.clone();
				if !self.open_file_mod(&p) {
					self.open_file(&p);
				}
			}
			if err {imgui::pop_style_color(1)}
		});
		
		// This shit doesnt fucking work, i cant move it.
		// aeth::divider("##explorer")
		// 	.column(imgui::TableColumnFlags::WidthFixed, 200.0, || {
		// 		imgui::text("test");
		// 	})
		// 	.column(imgui::TableColumnFlags::WidthStretch, 0.0, || {
		// 		imgui::text("test2");
		// 	})
		// 	.finish();
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
			datas,
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
			m.datas.penumbra.files.keys()
				.for_each(|p| m.tree.node_state(p, true));
		} else {
			m.datas.penumbra.options.iter()
				.for_each(|o| if let ConfOption::Multi(opt) | ConfOption::Single(opt) = o && opt.name == m.opt {
					opt.options.iter()
						.filter(|s| s.name == m.subopt)
						.for_each(|s| s.files.keys()
							.for_each(|p| m.tree.node_state(p, true)))
				});
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
		let paths = if m.opt == "" {
			m.datas.penumbra.files.get(&path).unwrap().complex()
		} else {
			m.datas.penumbra.options.iter()
				.find_map(|o| if let ConfOption::Multi(opt) | ConfOption::Single(opt) = o && opt.name == m.opt {
					Some(opt.options.iter()
						.find_map(|o| if o.name == m.subopt {Some(o)} else {None})
						.unwrap()
						.files.get(&path)
						.unwrap()
						.complex()
					)} else {None})
				.unwrap()
		};
		let rootpath = Some(m.path.clone());
		let mut settings = HashMap::new();
		
		for l in &paths {
			if let Some(id) = &l[0] {
				settings.insert(id.clone(), m.datas.penumbra.options.iter().find(|e| e.id() == Some(&id)).unwrap().default());
			}
		}
		
		self.open_viewer(&ext, &path, rootpath, Some(paths), Some(settings));
		
		true
	}
	
	fn open_viewer(&mut self, ext: &str, path: &str, rootpath: Option<PathBuf>, realpaths: Option<Vec<Vec<Option<String>>>>, settings: Option<HashMap<String, ConfSetting>>) {
		self.viewer = match ext {
			"tex" | "atex" => Some(Box::new(viewer::Tex::new(path.to_owned(), rootpath, realpaths, settings))),
			_ => None,
		};
	}
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
				m.datas.penumbra.options.iter()
					.find_map(|o| if let ConfOption::Multi(opt) | ConfOption::Single(opt) = o && opt.name == m.opt {
						Some(opt.options.iter().find_map(|o| if o.name == m.subopt {Some(o)} else {None})?)
					} else {
						None
					})?.files.contains_key(path).then(|| ())
			} else {
				m.datas.penumbra.files.contains_key(path).then(|| ())
			}
		} else {
			None
		}
	})().is_some()
}