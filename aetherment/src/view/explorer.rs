use std::{rc::Rc, cell::RefCell, fs::File, io::{Read, BufReader, BufWriter, Cursor, Write}, collections::HashMap, path::PathBuf};
use crate::resource_loader::{BacktraceError, ExplorerError};

pub mod error;
pub mod tree;
pub mod modmeta;
pub mod modmanage;

pub mod generic;
pub mod tex;
pub mod uld;

// ----------

#[derive(Debug, Clone)]
pub struct Mod {
	pub path: PathBuf,
	pub meta: Rc<RefCell<crate::modman::meta::Meta>>,
}

type ViewT = Rc<RefCell<Box<dyn View>>>;
// type ModsMap = Rc<RefCell<HashMap<String, (PathBuf, Rc<RefCell<crate::modman::meta::Meta>>)>>>; // wtf am i doing
type ModsMap = Rc<RefCell<HashMap<String, Mod>>>; // wtf am i doing

#[derive(Debug)]
pub enum DialogStatus {
	None,
	Dialog(egui_file::FileDialog),
	CreateReq(PathBuf),
}

pub struct Explorer {
	dock: egui_dock::Tree<ViewT>,
	to_add: Rc<RefCell<Vec<(ViewT, bool)>>>,
	#[allow(dead_code)] tree_data: Rc<RefCell<tree::TreeData>>,
	#[allow(dead_code)] mods: ModsMap,
	viewer: Viewer,
	
	import: DialogStatus,
	import_mod: Option<String>,
	import_option: Option<(u32, u32)>,
	import_path: String,
}

impl Explorer {
	pub fn new(ctx: egui::Context) -> Self {
		let to_add = Rc::new(RefCell::new(Vec::new()));
		
		// open_viewer(ctx.clone(), to_add.clone(), &"ui/uld/jobhudwar0.uld", None);
		// open_viewer(ctx.clone(), to_add.clone(), &"ui/uld/ConfigSystem.uld", None);
		
		let tree_data = Rc::new(RefCell::new(tree::TreeData {
			mod_trees: Vec::new(),
			game_paths: {
				let to_add = to_add.clone();
				let ctx = ctx.clone();
				tree::LazyTree::new(Box::new(move |path, button| {
					let mut in_new_tab = false;
					if button.context_menu(|ui| {
						in_new_tab |= ui.button("Open new tab").clicked();
					}).clicked() || in_new_tab {
						open_viewer(ctx.clone(), to_add.clone(), &path, None, None, !in_new_tab);
					}
				}))
			},
		}));
		
		let mods = Rc::new(RefCell::new(HashMap::new()));
		
		let tree = tree::Tree::new(
			tree_data.clone(),
			{
				let tree_data = tree_data.clone();
				let mods = mods.clone();
				let to_add = to_add.clone();
				let ctx = ctx.clone();
				Box::new(move |mod_path| {
					open_mod(ctx.clone(), to_add.clone(), mod_path, tree_data.clone(), mods.clone())
				})
			},
		);
		
		let config = crate::config();
		for m in &config.config.mod_paths {
			open_mod(ctx.clone(), to_add.clone(), m, tree_data.clone(), mods.clone());
		}
		
		let data = || -> Result<Vec<u8>, BacktraceError> {
			let mut f = File::open(dirs::cache_dir().unwrap().join("Aetherment").join("paths"))?;
			let mut data = Vec::new();
			f.read_to_end(&mut data)?;
			Ok(data)
		}();
		
		// do smth with this, probably reload button if failed
		if let Ok(data) = data {
			_ = tree_data.borrow_mut().game_paths.load(data);
		}
		
		Self {
			dock: egui_dock::Tree::new(vec![Rc::new(RefCell::new(Box::new(tree)))]),
			to_add,
			tree_data,
			mods,
			viewer: Viewer{dialog: None},
			
			import: DialogStatus::None,
			import_mod: None,
			import_option: None,
			import_path: String::new(),
		}
	}
}

impl super::View for Explorer {
	fn name(&self) -> &'static str {
		&"Explorer"
	}
	
	fn render(&mut self, ui: &mut egui::Ui) {
		egui_dock::DockArea::new(&mut self.dock)
			.id(egui::Id::new("explorer_dock"))
			.style(egui_dock::Style::from_egui(ui.style().as_ref()))
			// .show_close_buttons(false)
			.show_inside(ui, &mut self.viewer);
		
		for (view, replace) in self.to_add.borrow_mut().drain(..) {
			if self.dock.num_tabs() == 1 {
				self.dock.split_right(egui_dock::NodeIndex::root(), 0.2, vec![view]);
			}  else {
				let mut last_leaf = None;
				for node in self.dock.iter_mut() {
					if node.is_leaf() {
						last_leaf = Some(node);
					}
				}
				
				if let Some(node) = last_leaf {
					if replace {
						if let egui_dock::Node::Leaf {tabs, active, ..} = node {
							tabs[active.0] = view;
						}
					} else {
						node.append_tab(view);
					}
				}
			}
		}
		
		if let Some((dialog, tab)) = &mut self.viewer.dialog {
			if dialog.show(ui.ctx()).selected() {
				if let Some(path) = dialog.path() {
					if let Some(ext) = path.extension() {
						let ext = ext.to_string_lossy();
						
						match dialog.dialog_type() {
							egui_file::DialogType::SaveFile => {
								if let Ok(file) = std::fs::File::create(path) {
									match tab.borrow().export(&ext, Box::new(std::io::BufWriter::new(file))) {
										Ok(()) => log!("File saved successfully to {path:?}"),
										Err(err) => log!(err, "Failed saving file {err}"),
									}
								}
							}
							
							egui_file::DialogType::OpenFile => {
								self.import_path = tab.borrow().path().to_owned();
								self.import = DialogStatus::CreateReq(path.to_owned());
							}
							
							_ => {}
						}
					}
					
					if let Some(parent) = path.parent() {
						crate::config().config.file_dialog_path = parent.to_owned();
						_ = crate::config().save_forced();
					}
				}
				
				self.viewer.dialog = None;
			}
		}
		
		match &mut self.import {
			DialogStatus::CreateReq(path) => {
				let path = path.to_owned();
				egui::Window::new("Import file").show(ui.ctx(), |ui| {
					use crate::modman::meta::*;
					
					egui::ComboBox::from_label("Mod")
						.selected_text(self.import_mod.as_ref().map_or("Invalid Mod", |s| &s))
						.show_ui(ui, |ui| {
							for m in self.mods.borrow().keys() {
								ui.selectable_value(&mut self.import_mod, Some(m.to_owned()), m);
							}
						});
					
					if let Some(selected_mod) = &self.import_mod {
						if let Some(m) = self.mods.borrow().get(selected_mod) {
							let selected_text = if let Some((option, _)) = &self.import_option {
								m.meta.borrow().options[*option as usize].name.to_string()
							} else {
								"None".to_string()
							};
							
							egui::ComboBox::from_label("Option")
								.selected_text(selected_text)
								.show_ui(ui, |ui| {
									let mut option = self.import_option.map(|v| v.0.clone());
									if ui.selectable_value(&mut option, None, "None").changed() {
										self.import_option = None;
									}
									
									for (i, o) in m.meta.borrow().options.iter().enumerate() {
										if let OptionSettings::SingleFiles(_) | OptionSettings::MultiFiles(_) = &o.settings {
											if ui.selectable_value(&mut option, Some(i as u32), &o.name).changed() {
												self.import_option = Some((option.unwrap(), 0));
											}
										}
									}
								});
							
							if let Some((option, sub_option)) = &mut self.import_option {
								let option = &m.meta.borrow().options[*option as usize];
								let selected_text = if let OptionSettings::SingleFiles(f) | OptionSettings::MultiFiles(f) = &option.settings {
									f.options[*sub_option as usize].name.to_string()
								} else {
									"Invalid".to_string()
								};
								
								egui::ComboBox::from_label("Sub option")
									.selected_text(selected_text)
									.show_ui(ui, |ui| {
										if let OptionSettings::SingleFiles(f) | OptionSettings::MultiFiles(f) = &option.settings {
											for (i, so) in f.options.iter().enumerate() {
												ui.selectable_value(sub_option, i as u32, &so.name);
											}
										}
									});
							}
						}
					}
					
					ui.text_edit_singleline(&mut self.import_path);
					
					ui.horizontal(|ui| {
						if ui.button("Import").clicked() {
							if let Some(mod_id) = &self.import_mod {
								if let Err(err) = (|| -> Result<(), noumenon::Error> {
									let converter = noumenon::Convert::from_ext(path.extension().ok_or("No Extension")?.to_str().unwrap(), &mut BufReader::new(File::open(&path)?))?;
									let mut data = Cursor::new(Vec::new());
									converter.convert(self.import_path.split(".").last().unwrap(), &mut data)?;
									let data = data.into_inner();
									// let hash = base64::encode_config(blake3::hash(&data).as_bytes(), base64::URL_SAFE_NO_PAD);
									let hash = crate::hash_str(blake3::hash(&data));
									let m = self.mods.borrow_mut();
									let m = m.get(mod_id).ok_or("Invalid import mod target")?;
									let mut f = BufWriter::new(File::create(m.path.join("files").join(&hash))?);
									f.write_all(&data)?;
									
									if let Some((option, sub_option)) = &self.import_option {
										let option = &mut m.meta.borrow_mut().options[*option as usize];
										if let OptionSettings::SingleFiles(f) | OptionSettings::MultiFiles(f) = &mut option.settings {
											self.tree_data.borrow_mut().mod_trees.iter_mut().find(|(_, path, _)| path == &m.path).unwrap().2.add_path(self.import_path.as_str(), &hash, Some((option.name.clone(), f.options[*sub_option as usize].name.clone())));
											f.options[*sub_option as usize].files.insert(self.import_path.to_owned(), hash);
										}
									} else {
										self.tree_data.borrow_mut().mod_trees.iter_mut().find(|(_, path, _)| path == &m.path).unwrap().2.add_path(self.import_path.as_str(), &hash, None);
										m.meta.borrow_mut().files.insert(self.import_path.to_owned(), hash);
									}
									
									m.meta.borrow().save(&m.path.join("meta.json"))?;
									
									Ok(())
								})() {
									log!(err, "Failed to import file {err:?}");
								}
							}
							
							self.import = DialogStatus::None;
						}
						
						if ui.button("Cancel").clicked() {
							self.import = DialogStatus::None;
						}
					})
				});
			}
			
			_ => {}
		}
	}
}

fn open_mod(ctx: egui::Context, to_add: Rc<RefCell<Vec<(ViewT, bool)>>>, mod_path: &std::path::Path, tree_data: Rc<RefCell<tree::TreeData>>, mods: ModsMap) {
	use crate::modman::meta::*;
	
	let config = crate::config();
	if !config.config.mod_paths.contains(&mod_path.to_owned()) {
		config.config.mod_paths.push(mod_path.to_owned());
		_ = config.save_forced();
	}
	
	let mod_name = mod_path.file_name().unwrap().to_str().unwrap().to_owned();
	let mod_path = mod_path.to_owned();
	
	if let Err(err) = (|| -> Result<_, BacktraceError> {
		let meta = serde_json::from_reader(std::io::BufReader::new(File::open(mod_path.join("meta.json"))?))?;
		mods.borrow_mut().insert(mod_name.clone(), Mod{path: mod_path.clone(), meta: Rc::new(RefCell::new(meta))});
		
		Ok(())
	})() {
		log!(err, "Failed opening mod {err:?}");
		return
	}
	
	let mut tree = {
		let mod_path = mod_path.clone();
		let tree_data = tree_data.clone();
		let mods = mods.clone();
		tree::StaticTree::new(Box::new(move |path, real_path, _options, button| {
			let mut in_new_tab = false;
			if button.context_menu(|ui| {
				in_new_tab |= ui.button("Open new tab").clicked();
			}).clicked() || in_new_tab {
				let mod_trees = &tree_data.borrow().mod_trees;
				if let Some((mod_name, mod_path, _mod_tree)) = mod_trees.iter().find(|(_, path, _)| path == &mod_path) {
					if let Some(mod_) = mods.borrow().get(mod_name) {
						match path.as_str() {
							"\0meta" => {
								if let Err(err) = || -> Result<(), BacktraceError> {
									to_add.borrow_mut().push((Rc::new(RefCell::new(Box::new(modmeta::ModMeta::new(mod_name.clone(), mod_path.join("meta.json"), mod_.meta.clone())?))), !in_new_tab));
									Ok(())
								}() {
									to_add.borrow_mut().push((Rc::new(RefCell::new(Box::new(error::Error::new("_modmeta", Some(&path), err)))), !in_new_tab))
								}
							}
							
							"\0manage" => {
								if let Err(err) = || -> Result<(), BacktraceError> {
									to_add.borrow_mut().push((Rc::new(RefCell::new(Box::new(modmanage::ModManage::new(mod_name.clone(), mod_path)?))), !in_new_tab));
									Ok(())
								}() {
									to_add.borrow_mut().push((Rc::new(RefCell::new(Box::new(error::Error::new("_modmanage", Some(&path), err)))), !in_new_tab))
								}
							}
							
							_ => {
								if let Some(real_path) = real_path {
									open_viewer(ctx.clone(), to_add.clone(), &path, Some(&mod_path.join("files").join(real_path).to_string_lossy().to_string()), Some(mod_.clone()), !in_new_tab);
								} else {
									log!(err, "No real path {path} (???)");
								}
							}
						}
					}
				}
			}
		}))
	};
	
	let m = mods.borrow_mut();
	let meta = m.get(&mod_name).unwrap().meta.borrow_mut();
	for (game, real) in &meta.files {
		tree.add_path(game, real, None);
	}
	for option in &meta.options {
		if let OptionSettings::SingleFiles(f) | OptionSettings::MultiFiles(f) = &option.settings {
			for sub_option in &f.options {
				for (game, real) in &sub_option.files {
					tree.add_path(game, real, Some((option.name.clone(), sub_option.name.clone())));
				}
			}
		}
	}
	
	tree_data.borrow_mut().mod_trees.push((
		mod_path.file_name().unwrap().to_str().unwrap().to_owned(),
		mod_path.clone(),
		tree
	));
}

fn open_viewer(ctx: egui::Context, to_add: Rc<RefCell<Vec<(ViewT, bool)>>>, path: &str, real_path: Option<&str>, mod_: Option<Mod>, replace: bool) {
	if let Err(err) = || -> Result<(), BacktraceError> {
		let viewer: ViewT = match path.trim_end_matches(".comp").split(".").last().unwrap() {
			"tex" | "atex" => Rc::new(RefCell::new(Box::new(tex::Tex::new(ctx, path, real_path, mod_)?))),
			"uld" => Rc::new(RefCell::new(Box::new(uld::Uld::new(path, real_path)?))),
			_ => Rc::new(RefCell::new(Box::new(generic::Generic::new(path, real_path)?))),
		};
		to_add.borrow_mut().push((viewer, replace));
		
		Ok(())
	}() {
		to_add.borrow_mut().push((Rc::new(RefCell::new(Box::new(error::Error::new(path, real_path, err)))), replace))
	}
}

// ----------

pub trait Writer: std::io::Write + std::io::Seek {}
// impl Writer for std::fs::File {} (should never use file directly as that is inefficient)
impl Writer for std::io::BufWriter<std::fs::File> {}

pub trait View {
	fn is_tree(&self) -> bool {false}
	fn name(&self) -> &str;
	fn path(&self) -> &str;
	fn exts(&self) -> Vec<&str> {Vec::new()}
	fn render(&mut self, ui: &mut egui::Ui) -> Result<(), BacktraceError>;
	fn render_options(&mut self, _ui: &mut egui::Ui) -> Result<(), BacktraceError> {Ok(())}
	fn export(&self, _ext: &str, _writer: Box<dyn Writer>) -> Result<(), BacktraceError> {Ok(())}
}

struct Viewer {
	dialog: Option<(egui_file::FileDialog, ViewT)>,
}

impl egui_dock::TabViewer for Viewer {
	type Tab = ViewT;
	
	fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
		tab.borrow().name().into()
	}
	
	fn ui(&mut self, ui: &mut egui::Ui, tab_raw: &mut Self::Tab) {
		if let Err(err) = tab_raw.borrow_mut().render(ui) {
			render_error(ui, &err);
		}
	}
	
	fn context_menu(&mut self, ui: &mut egui::Ui, tab_raw: &mut Self::Tab) {
		let mut tab = tab_raw.borrow_mut();
		if let Err(err) = tab.render_options(ui) {
			render_error(ui, &err);
		}
		
		let exts = tab.exts();
		if exts.len() > 0 {
			if ui.button("Import").clicked() && self.dialog.is_none() {
				let mut dialog = egui_file::FileDialog::open_file(Some(crate::config().config.file_dialog_path.clone()))
					.default_filename(tab.name())
					.title(&format!("Import {}", tab.name()));
				dialog.open();
				self.dialog = Some((dialog, tab_raw.clone()));
			}
			
			if ui.button("Export").clicked() && self.dialog.is_none() {
				let mut dialog = egui_file::FileDialog::save_file(Some(crate::config().config.file_dialog_path.clone()))
					.default_filename(tab.name())
					.title(&format!("Export {}", tab.name()));
				dialog.open();
				self.dialog = Some((dialog, tab_raw.clone()));
			}
			
			ui.separator();
		}
	}
	
	fn on_close(&mut self, tab: &mut Self::Tab) -> bool {
		!tab.borrow().is_tree()
	}
}

pub fn render_error(ui: &mut egui::Ui, err: &BacktraceError) {
	ui.horizontal_centered(|ui| {
		ui.vertical_centered(|ui| {
			ui.label(egui::RichText::new(format!("{:?}", err))
				.color(egui::epaint::Color32::RED))
		})
	});
}