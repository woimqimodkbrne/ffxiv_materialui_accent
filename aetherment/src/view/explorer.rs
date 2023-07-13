use std::{rc::Rc, sync::Mutex};

pub mod generic;
pub mod error;
pub mod tree;
pub mod tex;

// ----------

pub struct Explorer {
	dock: egui_dock::Tree<Box<dyn View>>,
	to_add: Rc<Mutex<Vec<Box<dyn View>>>>,
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
				
				Box::new(tree)
			}]),
			to_add,
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
			.show_inside(ui, &mut Viewer{});
		
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
	}
}

fn open_viewer(ctx: egui::Context, to_add: Rc<Mutex<Vec<Box<dyn View>>>>, path: &str, real_path: Option<&str>) {
	if let Err(err) = || -> Result<(), BacktraceError> {
		match path.split(".").last().unwrap() {
			"tex" => to_add.lock().unwrap().push(Box::new(tex::Tex::new(ctx, path, real_path)?)),
			_ => to_add.lock().unwrap().push(Box::new(generic::Generic::new(path, real_path))),
		}
		
		Ok(())
	}() {
		to_add.lock().unwrap().push(Box::new(error::Error::new(path, real_path, err)))
	}
}

// ----------

// pub type BacktraceError = Box<dyn std::error::Error>;

#[derive(Debug)]
pub struct BacktraceError {
	#[allow(dead_code)]
	err: Box<dyn std::any::Any + Send + 'static>,
	backtrace: backtrace::Backtrace,
}

// impl BacktraceError {
// 	pub fn new(err: Box<dyn std::any::Any + Send + 'static>) -> Self {
// 		Self {
// 			err,
// 			backtrace: backtrace::Backtrace::new(),
// 		}
// 	}
// }

impl<E> From<E> for BacktraceError where
E: std::error::Error + Send + Sync + 'static {
	fn from(err: E) -> Self {
		Self {
			err: Box::new(err),
			backtrace: backtrace::Backtrace::new(),
		}
	}
}

impl std::fmt::Display for BacktraceError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", self.backtrace)
	}
}

// ----------

pub trait View {
	fn name<'a>(&'a self) -> &'a str;
	fn path<'a>(&'a self) -> &'a str;
	// fn load(&mut self, ctx: &mut egui::Context, path: &str, real_path: Option<&str>) -> Result<(), Box<dyn std::error::Error>>;
	fn render(&mut self, ui: &mut egui::Ui) -> Result<(), BacktraceError>;
}

struct Viewer;
impl egui_dock::TabViewer for Viewer {
	type Tab = Box<dyn View>;
	
	fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
		tab.name().into()
	}
	
	fn id(&mut self, tab: &mut Self::Tab) -> egui::Id {
		egui::Id::new(tab.name())
	}
	
	fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
		if let Err(err) = tab.render(ui) {
			ui.horizontal_centered(|ui| {
				ui.vertical_centered(|ui| {
					ui.label(egui::RichText::new(format!("{:?}", err))
						.color(egui::epaint::Color32::RED))
				})
			});
		}
	}
	
	fn on_close(&mut self, tab: &mut Self::Tab) -> bool {
		tab.path() != "_filetree"
	}
}