mod main;
pub use main::Main as Main;

mod settings;
pub use settings::Settings as Settings;

mod explorer;
pub use explorer::Explorer as Explorer;

mod debug;
pub use debug::Debug as Debug;

pub trait View {
	fn name(&self) -> &'static str;
	fn render(&mut self, ui: &mut egui::Ui);
}

pub struct Viewer;
impl egui_dock::TabViewer for Viewer {
	type Tab = Box<dyn View>;
	
	fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
		tab.render(ui);
	}
	
	fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
		tab.name().into()
	}
}