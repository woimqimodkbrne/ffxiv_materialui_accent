use std::{rc::Rc, cell::RefCell, fs::File, io::Read};
use crate::resource_loader::{BacktraceError, ExplorerError};

pub mod error;
pub mod tree;
pub mod modmeta;

pub mod generic;
pub mod tex;
pub mod uld;

// ----------

type ViewT = Rc<RefCell<Box<dyn View>>>;

pub struct Explorer {
	dock: egui_dock::Tree<ViewT>,
	to_add: Rc<RefCell<Vec<ViewT>>>,
	viewer: Viewer,
}

impl Explorer {
	pub fn new(ctx: egui::Context) -> Self {
		let to_add = Rc::new(RefCell::new(Vec::new()));
		
		Self {
			dock: egui_dock::Tree::new(vec![{
				// open_viewer(ctx.clone(), to_add.clone(), &"ui/uld/jobhudwar0.uld", None);
				open_viewer(ctx.clone(), to_add.clone(), &"ui/uld/ConfigSystem.uld", None);
				
				let mut tree = tree::Tree::new(
					Vec::new(),
					{
						let to_add = to_add.clone();
						let ctx = ctx.clone();
						tree::LazyTree::new(Box::new(move |path, button| {
							if button.clicked() {
								open_viewer(ctx.clone(), to_add.clone(), &path, None)
							}
						}))
					},
					{
						let to_add = to_add.clone();
						let ctx = ctx.clone();
						Box::new(move |mod_path, mod_trees| {
							open_mod(ctx.clone(), to_add.clone(), mod_path, mod_trees)
						})
					},
				);
				
				let config = crate::config();
				for m in &config.config.mod_paths {
					open_mod(ctx.clone(), to_add.clone(), m, &mut tree.mod_trees);
				}
				
				let data = || -> Result<Vec<u8>, BacktraceError> {
					let mut f = File::open(dirs::cache_dir().unwrap().join("Aetherment").join("paths"))?;
					let mut data = Vec::new();
					f.read_to_end(&mut data)?;
					Ok(data)
				}();
				
				// do smth with this, probably reload button if failed
				if let Ok(data) = data {
					_ = tree.game_paths.load(data);
				}
				
				Rc::new(RefCell::new(Box::new(tree)))
			}]),
			to_add,
			viewer: Viewer{dialog: None}
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
		
		for view in self.to_add.borrow_mut().drain(..) {
			if self.dock.len() == 1 {
				self.dock.split_right(egui_dock::NodeIndex::root(), 0.2, vec![view]);
			} else {
				let mut last_leaf = None;
				for node in self.dock.iter_mut() {
					if node.is_leaf() {
						last_leaf = Some(node);
					}
				}
				
				if let Some(node) = last_leaf {
					node.append_tab(view);
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
								log!(err, "TODO: import");
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
	}
}

fn open_mod(_ctx: egui::Context, to_add: Rc<RefCell<Vec<ViewT>>>, mod_path: &std::path::Path, mod_trees: &mut Vec<(String, std::path::PathBuf, tree::StaticTree)>) {
	let config = crate::config();
	if !config.config.mod_paths.contains(&mod_path.to_owned()) {
		config.config.mod_paths.push(mod_path.to_owned());
		_ = config.save_forced();
	}
	
	let mod_name = mod_path.file_name().unwrap().to_str().unwrap().to_owned();
	let mod_path = mod_path.to_owned();
	mod_trees.push((
		mod_path.file_name().unwrap().to_str().unwrap().to_owned(),
		mod_path.clone(),
		tree::StaticTree::new(Box::new(move |path, button| {
			if button.clicked() {
				if path == "Meta" {
					if let Err(err) = || -> Result<(), BacktraceError> {
						to_add.borrow_mut().push(Rc::new(RefCell::new(Box::new(modmeta::ModMeta::new(mod_name.clone(), mod_path.join("meta.json"))?))));
						Ok(())
					}() {
						to_add.borrow_mut().push(Rc::new(RefCell::new(Box::new(error::Error::new("_modmeta", Some(&path), err)))))
					}
				} else {
					log!("TODO: mod files opening")
					// open_viewer(ctx.clone(), to_add.clone(), "", None)
				}
			}
		}))
	));
}

fn open_viewer(ctx: egui::Context, to_add: Rc<RefCell<Vec<ViewT>>>, path: &str, real_path: Option<&str>) {
	if let Err(err) = || -> Result<(), BacktraceError> {
		match path.split(".").last().unwrap() {
			"tex" | "atex" => to_add.borrow_mut().push(Rc::new(RefCell::new(Box::new(tex::Tex::new(ctx, path, real_path)?)))),
			"uld" => to_add.borrow_mut().push(Rc::new(RefCell::new(Box::new(uld::Uld::new(path, real_path)?)))),
			_ => to_add.borrow_mut().push(Rc::new(RefCell::new(Box::new(generic::Generic::new(path, real_path)?)))),
		}
		
		Ok(())
	}() {
		to_add.borrow_mut().push(Rc::new(RefCell::new(Box::new(error::Error::new(path, real_path, err)))))
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
					.title(tab.name());
				dialog.open();
				self.dialog = Some((dialog, tab_raw.clone()));
			}
			
			if ui.button("Export").clicked() && self.dialog.is_none() {
				let mut dialog = egui_file::FileDialog::save_file(Some(crate::config().config.file_dialog_path.clone()))
					.default_filename(tab.name())
					.title(tab.name());
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