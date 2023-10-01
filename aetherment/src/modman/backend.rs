#[allow(non_snake_case)]
pub mod penumbra_ipc;

pub trait Backend {
	fn name(&self) -> &'static str;
	fn description(&self) -> &'static str;
	fn install_mod(&self, file: &std::path::Path) -> Result<String, crate::resource_loader::BacktraceError>;
	fn get_all_mods(&self) -> Vec<String>;
	fn apply_mod_settings(&self, mod_id: &str, collection: &str, settings: Option<&crate::modman::settings::Settings>) -> Result<(), crate::resource_loader::BacktraceError>;
	fn debug_renderer(&self, _ui: &mut egui::Ui) {}
}

pub struct DummyBackend;
impl Backend for DummyBackend {
	fn name(&self) -> &'static str {
		"No backend"
	}
	
	fn description(&self) -> &'static str {
		#[cfg(feature = "plugin")]
		return "No valid backend found for plugin";
		#[cfg(not(feature = "plugin"))]
		return "No valid backend found for standalone";
	}
	
	fn install_mod(&self, _file: &std::path::Path) -> Result<String, crate::resource_loader::BacktraceError> {Ok(String::new())}
	fn get_all_mods(&self) -> Vec<String> {Vec::new()}
	fn apply_mod_settings(&self, _mod_id: &str, _collection: &str, _settings: Option<&crate::modman::settings::Settings>) -> Result<(), crate::resource_loader::BacktraceError> {Ok(())}
}

pub enum BackendInitializers {
	PenumbraIpc(penumbra_ipc::PenumbraFunctions),
	None,
}

pub fn new_backend(backend: BackendInitializers) -> Box<dyn Backend> {
	match backend {
		// #[cfg(feature = "plugin")]
		BackendInitializers::PenumbraIpc(funcs) => Box::new(penumbra_ipc::Penumbra::new(funcs)),
		_ => Box::new(DummyBackend),
	}
}