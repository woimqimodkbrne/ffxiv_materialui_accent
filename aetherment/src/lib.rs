#[macro_use]
mod log;
mod view;

pub use log::LogType;

lazy_static::lazy_static! {
	pub static ref NOUMENON: noumenon::Noumenon = noumenon::get_noumenon();
}

pub struct Core {
	views: egui_dock::Tree<Box<dyn view::View>>,
}

impl Core {
	pub fn new(log: fn(log::LogType, String), ctx: egui::Context) -> Self {
		unsafe{log::LOG = log};
		
		Self {
			views: egui_dock::Tree::new(vec![
				Box::new(view::Main::new()),
				Box::new(view::Explorer::new(ctx)),
				Box::new(view::Debug::new()),
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
			.show_inside(ui, &mut view::Viewer{});
	}
}