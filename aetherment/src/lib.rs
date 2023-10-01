#[macro_use]
mod log;
mod config;
// mod migrate;
pub mod modman;
mod view;
mod render_helper;
mod resource_loader;

pub use log::LogType;
// pub use renderer::Backends;

// static mut RENDERBACKEND: renderer::Backends = renderer::Backends::empty();
// pub(crate) fn get_rendering_backend() -> renderer::Backends {
// 	unsafe{RENDERBACKEND.clone()}
// }

static MODREPO: &str = "https://mods.aetherment.com/list.json";

static mut CONFIG: Option<config::ConfigManager> = None;
pub fn config() -> &'static mut config::ConfigManager {
	unsafe{CONFIG.get_or_insert_with(|| config::ConfigManager::load(&dirs::config_dir().unwrap().join("Aetherment").join("config.json")))}
}

static mut BACKEND: Option<Box<dyn modman::backend::Backend>> = None;
pub fn backend() -> &'static mut Box<dyn modman::backend::Backend> {
	unsafe{BACKEND.as_mut().unwrap()}
}

static mut NOUMENON: Option<Option<noumenon::Noumenon>> = None;
#[cfg(feature = "plugin")]
pub fn noumenon() -> Option<&'static noumenon::Noumenon> {
	unsafe{NOUMENON.get_or_insert_with(|| noumenon::get_noumenon(Some(std::env::current_exe().unwrap().parent().unwrap().parent().unwrap()))).as_ref()}
}
#[cfg(not(feature = "plugin"))]
pub fn noumenon() -> Option<&'static noumenon::Noumenon> {
	unsafe{NOUMENON.get_or_insert_with(|| noumenon::get_noumenon(config().config.game_install.as_ref())).as_ref()}
}

pub fn hash_str(hash: blake3::Hash) -> String {
	base64::encode_config(hash.as_bytes(), base64::URL_SAFE_NO_PAD)
}

pub fn json_pretty<T: serde::Serialize>(data: &T) -> Result<String, serde_json::Error> {
	// serde_json::to_writer_pretty(&mut File::create(path)?, self)?;
	let mut serializer = serde_json::Serializer::with_formatter(Vec::new(), serde_json::ser::PrettyFormatter::with_indent(b"\t"));
	data.serialize(&mut serializer)?;
	Ok(String::from_utf8(serializer.into_inner()).unwrap())
}

pub struct Core {
	views: egui_dock::Tree<Box<dyn view::View>>,
}

impl Core {
	pub fn new(log: fn(log::LogType, String), ctx: egui::Context, backend: modman::backend::BackendInitializers/*, render_backend: renderer::Backends*/) -> Self {
		unsafe {
			log::LOG = log;
			BACKEND = Some(modman::backend::new_backend(backend));
			// RENDERBACKEND = render_backend;
		}
		
		Self {
			views: egui_dock::Tree::new(vec![
				Box::new(view::Main::new()),
				Box::new(view::Settings::new()),
				Box::new(view::Explorer::new(ctx)),
				Box::new(view::Debug::new()),
			]),
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