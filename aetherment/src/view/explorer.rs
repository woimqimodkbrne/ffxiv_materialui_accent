use std::{rc::Rc, sync::Mutex, cell::RefCell};

pub mod generic;
pub mod error;
pub mod tree;
pub mod tex;

// ----------

type ViewT = Rc<RefCell<Box<dyn View>>>;

pub struct Explorer {
	dock: egui_dock::Tree<ViewT>,
	to_add: Rc<Mutex<Vec<ViewT>>>,
	viewer: Viewer,
}

impl Explorer {
	pub fn new(ctx: egui::Context) -> Self {
		let to_add = Rc::new(Mutex::new(Vec::new()));
		
		Self {
			dock: egui_dock::Tree::new(vec![{
				let to_add = to_add.clone();
				let mut tree = tree::Tree::new();
				// do smth with this, probably reload button if failed
				_ = tree.load(Box::new(move |path, button| {
					if button.clicked() {
						open_viewer(ctx.clone(), to_add.clone(), &path, None)
					}
				}));
				
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
		
		// self.dock.
		
		for view in self.to_add.lock().unwrap().drain(..) {
			let mut last_leaf = None;
			for node in self.dock.iter_mut() {
				if node.is_leaf() {
					last_leaf = Some(node);
				}
			}
			
			if let Some(node) = last_leaf {
				node.append_tab(view);
			}
			
			// TODO: if first make it auto split
		}
		
		if let Some((dialog, tab)) = &mut self.viewer.dialog {
			if dialog.show(ui.ctx()).selected() {
				if let Some(path) = dialog.path() {
					if let Some(ext) = path.extension() {
						let ext = ext.to_string_lossy();
						
						if let Ok(file) = std::fs::File::create(path) {
							match tab.borrow().export(&ext, Box::new(std::io::BufWriter::new(file))) {
								Ok(()) => log!("File saved successfully to {path:?}"),
								Err(err) => log!(err, "Failed saving file {err}"),
							}
						}
					}
				}
				
				self.viewer.dialog = None;
			}
		}
	}
}

fn open_viewer(ctx: egui::Context, to_add: Rc<Mutex<Vec<ViewT>>>, path: &str, real_path: Option<&str>) {
	if let Err(err) = || -> Result<(), BacktraceError> {
		match path.split(".").last().unwrap() {
			"tex" | "atex" => to_add.lock().unwrap().push(Rc::new(RefCell::new(Box::new(tex::Tex::new(ctx, path, real_path)?)))),
			_ => to_add.lock().unwrap().push(Rc::new(RefCell::new(Box::new(generic::Generic::new(path, real_path)?)))),
		}
		
		Ok(())
	}() {
		to_add.lock().unwrap().push(Rc::new(RefCell::new(Box::new(error::Error::new(path, real_path, err)))))
	}
}

// ----------

pub type BacktraceError = Box<dyn std::error::Error>;

// #[derive(Debug)]
// pub struct BacktraceError {
// 	#[allow(dead_code)]
// 	err: Box<dyn std::any::Any + Send + 'static>,
// 	backtrace: backtrace::Backtrace,
// }
// 
// // impl BacktraceError {
// // 	pub fn new(err: Box<dyn std::any::Any + Send + 'static>) -> Self {
// // 		Self {
// // 			err,
// // 			backtrace: backtrace::Backtrace::new(),
// // 		}
// // 	}
// // }
// 
// impl<E> From<E> for BacktraceError where
// E: std::error::Error + Send + Sync + 'static {
// 	fn from(err: E) -> Self {
// 		Self {
// 			err: Box::new(err),
// 			backtrace: backtrace::Backtrace::new(),
// 		}
// 	}
// }
// 
// impl std::fmt::Display for BacktraceError {
// 	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// 		write!(f, "{:?}", self.backtrace)
// 	}
// }

#[derive(Debug)]
pub enum ExplorerError {
	Path(String),
	RealPath(String),
	Data,
}

impl std::fmt::Display for ExplorerError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Path(path) => write!(f, "Invalid game path: {:?}", path),
			Self::RealPath(path) => write!(f, "Invalid real path: {:?}", path),
			Self::Data => write!(f, "File is invalid"),
		}
	}
}

impl std::error::Error for ExplorerError {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		match self {
			_ => None,
		}
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
	
	fn id(&mut self, tab: &mut Self::Tab) -> egui::Id {
		egui::Id::new(tab.borrow().name())
	}
	
	fn ui(&mut self, ui: &mut egui::Ui, tab_raw: &mut Self::Tab) {
		let mut tab = tab_raw.borrow_mut();
		let pos = ui.cursor().min;
		let space = ui.available_size();
		
		if let Err(err) = tab.render(ui) {
			ui.horizontal_centered(|ui| {
				ui.vertical_centered(|ui| {
					ui.label(egui::RichText::new(format!("{:?}", err))
						.color(egui::epaint::Color32::RED))
				})
			});
		} else if !tab.is_tree() {
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
				.id(egui::Id::new(tab.path()))
				.drag_bounds(egui::Rect{min: pos, max: pos + space})
				.resizable(false)
				.show(ui.ctx(), |ui| {
					if let Err(err) = tab.render_options(ui) {
						ui.horizontal_centered(|ui| {
							ui.vertical_centered(|ui| {
								ui.label(egui::RichText::new(format!("{:?}", err))
									.color(egui::epaint::Color32::RED))
							})
						});
					}
					
					let exts = tab.exts();
					if exts.len() > 0 {
						// ui.separator();
						ui.add_space(10.0);
						
						ui.horizontal(|ui| {
							if ui.button("Import").clicked() && self.dialog.is_none() {
								log!(err, "TODO: import");
							}
							
							if ui.button("Export").clicked() && self.dialog.is_none() {
								// TODO: save last location and open it there
								let mut dialog = egui_file::FileDialog::save_file(dirs::document_dir())
									.default_filename(tab.name())
									.title(tab.name());
								dialog.open();
								self.dialog = Some((dialog, tab_raw.clone()));
							}
						});
					}
				});
		}
	}
	
	fn on_close(&mut self, tab: &mut Self::Tab) -> bool {
		!tab.borrow().is_tree()
	}
}