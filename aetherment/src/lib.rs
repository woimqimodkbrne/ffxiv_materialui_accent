#[macro_use]
mod log;
// mod config;
// mod migrate;
mod view;
mod render_helper;
mod resource_loader;

pub use log::LogType;
pub use renderer::Backends;

#[cfg(feature = "plugin")]
lazy_static::lazy_static! {
	pub static ref NOUMENON: Option<noumenon::Noumenon> = noumenon::get_noumenon(Some(std::env::current_exe().unwrap().parent().unwrap().parent().unwrap()));
}

#[cfg(not(feature = "plugin"))]
lazy_static::lazy_static! {
	// TODO: load from config if entry exists
	pub static ref NOUMENON: Option<noumenon::Noumenon> = noumenon::get_noumenon(None::<&str>);
}

static mut BACKEND: renderer::Backends = renderer::Backends::empty();
pub(crate) fn get_backend() -> renderer::Backends {
	unsafe{BACKEND.clone()}
}

pub struct Core {
	views: egui_dock::Tree<Box<dyn view::View>>,
}

impl Core {
	pub fn new(log: fn(log::LogType, String), ctx: egui::Context, backend: renderer::Backends) -> Self {
		unsafe {
			log::LOG = log;
			BACKEND = backend;
		}
		
		Self {
			views: egui_dock::Tree::new(vec![
				Box::new(view::Explorer::new(ctx)),
				Box::new(view::Debug::new()),
				Box::new(view::Main::new()),
				// Box::new(view::Debug::new()),
			])
		}
	}
	
	pub fn draw(&mut self, ui: &mut egui::Ui) {
		egui_dock::DockArea::new(&mut self.views)
			.id(egui::Id::new("tabs"))
			.style(egui_dock::Style::from_egui(ui.style().as_ref()))
			.draggable_tabs(false)
			.show_close_buttons(false)
			.tab_context_menus(false)
			.show_inside(ui, &mut view::Viewer);
	}
}