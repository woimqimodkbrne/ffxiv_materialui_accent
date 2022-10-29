use std::{fs::{File, self}, path::PathBuf, io::{Write, Cursor, BufReader, BufRead, Read, Seek}};
use crate::{gui::aeth::{self, F2}, GAME, apply::{self, penumbra::{self, ConfOption, PenumbraFile, FileLayer}}};

mod tree;
mod viewer;
use serde_json::json;
use tree::Tree;

use self::viewer::{Viewer, Conf};

pub struct Tab {
	first_draw: bool,
	refresh_mod: bool,
	curmod: Option<Mod>,
	selected_mod: String,
	
	populated_modselect: bool,
	mod_entries: Vec<String>,
	
	importing: bool,
	exporting: bool,
	
	gametree: Tree,
	viewer: Option<Box<dyn Viewer + Send>>,
	path: String,
	valid_path: bool,
}

impl Tab {
	pub fn new(_state: &mut crate::Data) -> Self {
		Tab {
			first_draw: true,
			refresh_mod: false,
			curmod: None,
			selected_mod: "None".to_owned(),
			
			populated_modselect: false,
			mod_entries: Vec::new(),
			
			importing: false,
			exporting: false,
			
			gametree: Tree::new("Game Files"),
			viewer: None,
			path: String::with_capacity(128),
			valid_path: false,
		}
	}
	
	pub fn draw(&mut self, state: &mut crate::Data) {
		if self.first_draw {
			let path = state.binary_path.join("assets").join("paths");
			let reader = BufReader::new(File::open(path).unwrap());
			for path in reader.lines() {
				let mut path = path.unwrap();
				path.make_ascii_lowercase();
				self.gametree.add_node(&path);
			}
			
			self.first_draw = false;
		}
		
		if self.refresh_mod {
			self.refresh_mod = false;
			let m = self.curmod.as_ref().unwrap();
			File::create(m.path.join("datas.json")).unwrap().write_all(crate::serialize_json(json!(&m.datas)).as_bytes()).unwrap();
			self.load_mod(&self.selected_mod.clone(), m.path.clone());
		}
		
		aeth::divider("explorer_div", false)
		.left(100.0, || {
			aeth::child("trees", [0.0, -(aeth::frame_height() + imgui::get_style().item_spacing.y()) * if self.curmod.is_some() {2.0} else {1.0}], false, imgui::WindowFlags::None, || {
				if let Some(m) = self.curmod.as_mut() {
					aeth::popup("modfilecontext", imgui::WindowFlags::None, || {
						if imgui::button("Remove", [0.0, 0.0]) {
							m.datas.penumbra.as_mut().unwrap().update_file(&m.opt, &m.subopt, &m.context_path, None);
							self.refresh_mod = true;
							imgui::close_current_popup();
						}
					});
					
					if let Some((button, path)) = m.tree.draw() {
						if button == imgui::MouseButton::Right {
							imgui::open_popup("modfilecontext", imgui::PopupFlags::MouseButtonRight);
							m.context_path = path;
						} else {
							self.open_file_mod(path);
						}
					}
				}
				
				if let Some((_button, path)) = self.gametree.draw() {
					self.open_file(path);
				}
			});
			
			if let Some(m) = &mut self.curmod {
				aeth::next_max_width();
				// aeth::combo("##optionselect", &format!("{}/{}", m.opt, m.subopt), imgui::ComboFlags::None, || {
				if imgui::begin_combo("##optionselect", &format!("{}/{}", m.opt, m.subopt), imgui::ComboFlags::None) { // scoped kinda sucks cuz closures suck
					let mut a = if imgui::selectable("Default", m.opt == "" && m.subopt == "", imgui::SelectableFlags::None, [0.0, 0.0]) {Some(("".to_owned(), "".to_owned()))} else {None};
					m.datas.penumbra.as_mut().unwrap().options.iter_mut().for_each(|o| if let ConfOption::Multi(opt) | ConfOption::Single(opt) = o {
						aeth::tree(&opt.name, || {
							opt.options.iter().for_each(|o2| if imgui::selectable(&o2.name, m.opt == opt.name && m.subopt == o2.name, imgui::SelectableFlags::None, [0.0, 0.0]) {
								a = Some((opt.name.clone(), o2.name.clone()));
							});
						});
					});
					
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
				
				if imgui::selectable("None", self.curmod.is_none(), imgui::SelectableFlags::None, [0.0, 0.0]) {
					self.selected_mod = "None".to_owned();
					self.curmod = None;
					crate::api::penumbra::remove_mod("aetherment_creator", i32::MAX);
					crate::api::penumbra::redraw_self();
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
					match &mut self.curmod {
						Some(m) => viewer.draw(state, Some(Conf {
							path: m.path.clone(),
							datas: &mut m.datas,
							// config: &mut m.datas.lock().unwrap().penumbra,
							option: &mut m.opt,
							sub_option: &mut m.subopt,
						})),
						None => viewer.draw(state, None),
					}
				}
			});
			
			if let Some(viewer) = self.viewer.as_ref() {
				if let Some(m) = self.curmod.as_mut() {
					if imgui::button("Import", [0.0, 0.0]) {self.importing = true}
					if self.importing {
						let filename = &self.path[self.path.rfind('/').unwrap() + 1..self.path.rfind('.').unwrap()];
						match aeth::file_dialog(format!("Importing {}", filename), || -> aeth::FileDialog {
						let mut d = aeth::FileDialog::new(&state.config.explorer_path, filename);
						for ext in viewer.valid_imports() {d = d.add_extension(ext, None)}
						
						d.finish()
					}) {
							aeth::FileDialogResult::Success(paths) => {
								self.importing = false;
								let path = &paths[0];
								if let Some(parent) = path.parent() {
									state.config.explorer_path = parent.to_path_buf().to_string_lossy().to_string();
									_ = state.config.save_forced();
								}
								log!("import {:?}", path);
								let ext = path.extension().unwrap().to_str().unwrap();
								let mut buf = Vec::new();
								if !noumenon::convert(&mut File::open(&path).unwrap(), ext, &mut Cursor::new(&mut buf), &self.path[self.path.rfind('.').unwrap() + 1..]) {
									log!("import failed"); // TODO: nice popup displaying that it failed
								}
								// let hash = blake3::hash(&buf).to_hex().as_str()[..24].to_string();
								let hash = crate::hash_str(blake3::hash(&buf).as_bytes());
								fs::create_dir_all(m.path.join("files")).unwrap();
								File::create(m.path.join("files").join(&hash)).unwrap().write_all(&buf).unwrap();
								let file = PenumbraFile(vec![FileLayer {
									id: None,
									paths: vec![format!("files/{}", &hash)],
								}]);
								m.datas.penumbra.as_mut().unwrap().update_file(&m.opt, &m.subopt, &self.path, Some(file));
								self.refresh_mod = true;
							},
							aeth::FileDialogResult::Canceled => self.importing = false,
							aeth::FileDialogResult::Busy => {},
						}
					}
					imgui::same_line();
				}
				
				if imgui::button("Export", [0.0, 0.0]) {self.exporting = true}
				if self.exporting {
					let filename = &self.path[self.path.rfind('/').unwrap() + 1..];
					match aeth::file_dialog(format!("Exporting {}", filename), || -> aeth::FileDialog {
						let mut d = aeth::FileDialog::new(&state.config.explorer_path, filename)
							.save_mode(true);
						
						for ext in viewer.valid_exports() {d = d.add_extension(ext, None)}
						
						d.finish()
					}) {
						aeth::FileDialogResult::Success(paths) => {
							self.exporting = false;
							let path = &paths[0];
							if let Some(parent) = path.parent() {
								state.config.explorer_path = parent.to_path_buf().to_string_lossy().to_string();
								_ = state.config.save_forced();
							}
							log!("export {:?}", path);
							let ext = path.extension().unwrap().to_str().unwrap();
							let mut buf = Vec::new();
							viewer.save(ext, &mut buf);
							File::create(&path).unwrap().write_all(&buf).unwrap();
						},
						aeth::FileDialogResult::Canceled => self.exporting = false,
						aeth::FileDialogResult::Busy => {},
					}
				}
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
		tree.add_node("Options");
		
		let mut datas: apply::Datas = serde_json::from_reader(File::open(path.join("datas.json")).unwrap()).unwrap();
		if datas.penumbra.is_none() {
			datas.penumbra = Some(penumbra::Config::default());
		}
		
		datas.penumbra.as_ref().unwrap().files.keys()
			.for_each(|p| tree.add_node(p));
		
		datas.penumbra.as_ref().unwrap().options.iter()
			.for_each(|o| if let ConfOption::Multi(opt) | ConfOption::Single(opt) = o {
				opt.options.iter()
					.for_each(|s| s.files.keys()
						.for_each(|p| tree.add_node(p)))
			});
		
		datas.cleanup(&path);
		
		self.curmod = Some(Mod {
			tree,
			datas,
			path,
			opt: "".to_owned(),
			subopt: "".to_owned(),
			context_path: "".to_owned(),
		});
		
		self.set_mod_option("", "");
	}
	
	fn set_mod_option<S>(&mut self, opt: S, sub: S) where
	S: Into<String> {
		let m = self.curmod.as_mut().unwrap();
		m.opt = opt.into();
		m.subopt = sub.into();
		m.tree.node_state_all(false);
		m.tree.node_state("Options", true);
		
		if m.opt == "" && m.subopt == "" {
			m.datas.penumbra.as_ref().unwrap().files.keys()
				.for_each(|p| m.tree.node_state(p, true));
		} else {
			m.datas.penumbra.as_ref().unwrap().options.iter()
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
		
		self.penumbra_draw();
	}
	
	fn penumbra_draw(&mut self) {
		let m = self.curmod.as_mut().unwrap();
		Conf {
			path: m.path.clone(),
			datas: &mut m.datas,
			option: &mut m.opt,
			sub_option: &mut m.subopt,
		}.reload_penumbra();
	}
	
	fn open_file<S>(&mut self, path: S) -> bool where
	S: Into<String> {
		self.path = path.into().to_lowercase();
		self.valid_path = valid_path(&self.path);
		if !self.valid_path {return false;}
		
		let ext = self.path.split('.').last().unwrap().to_owned();
		let path = self.path.clone();
		self.open_viewer(&ext, &path);
		
		true
	}
	
	fn open_file_mod<S>(&mut self, path: S) -> bool where
	S: Into<String> {
		self.path = path.into().to_lowercase();
		match self.path.as_str() {
			"options" => {
				self.valid_path = true;
				self.viewer = Some(Box::new(viewer::Options::new()));
				
				true
			},
			_ => {
				self.valid_path = valid_path_mod(&self.curmod, &self.path);
				if !self.valid_path {return false;}
				
				let ext = self.path.split('.').last().unwrap().to_owned();
				let path = self.path.clone();
				self.open_viewer(&ext, &path);
				
				true
			},
		}
	}
	
	fn open_viewer(&mut self, ext: &str, path: &str) {
		let conf = if let Some(m) = &mut self.curmod {
			Some(Conf {
				path: m.path.clone(),
				datas: &mut m.datas,
				option: &mut m.opt,
				sub_option: &mut m.subopt,
			})
		} else {None};
		
		self.viewer = match ext {
			"tex" | "atex" => Some(Box::new(viewer::Tex::new(path.to_owned(), conf))),
			"mtrl" => Some(Box::new(viewer::Mtrl::new(path.to_owned(), conf))),
			_ => Some(Box::new(viewer::Generic::new(path.to_owned(), conf))),
		};
	}
}

struct Mod {
	tree: Tree,
	datas: apply::Datas,
	path: PathBuf,
	opt: String,
	subopt: String,
	context_path: String,
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
				m.datas.penumbra.as_ref().unwrap().options.iter()
					.find_map(|o| if let ConfOption::Multi(opt) | ConfOption::Single(opt) = o && opt.name == m.opt {
						Some(opt.options.iter().find_map(|o| if o.name == m.subopt {Some(o)} else {None})?)
					} else {
						None
					})?.files.contains_key(path).then(|| ())
			} else {
				m.datas.penumbra.as_ref().unwrap().files.contains_key(path).then(|| ())
			}
		} else {
			None
		}
	})().is_some()
}

pub fn load_file(m: &Option<Conf>, path: &str) -> Vec<u8> {
	match m.as_ref() {
		Some(m) => {
			match m.datas.penumbra.as_ref().unwrap().file_ref(&m.option, &m.sub_option, path) {
				Some(f) => {
					let mut file = File::open(format!("{}/{}", m.path.to_str().unwrap(), f.0[0].paths[0])).unwrap();
					let mut buf = Vec::with_capacity(file.stream_len().unwrap() as usize);
					file.read_to_end(&mut buf).unwrap();
					buf
				},
				None => GAME.file::<Vec<u8>>(path).unwrap(),
			}
		},
		None => GAME.file::<Vec<u8>>(path).unwrap(),
	}
}