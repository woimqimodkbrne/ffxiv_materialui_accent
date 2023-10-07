#[allow(non_snake_case)]
pub mod penumbra_ipc;

pub trait Backend {
	fn name(&self) -> &'static str;
	fn description(&self) -> &'static str;
	fn is_functional(&self) -> bool {true}
	fn get_mods(&self) -> Vec<String>;
	fn get_collections(&self) -> Vec<String>;
	fn install_mod(&mut self, file: &std::path::Path) -> Result<String, crate::resource_loader::BacktraceError>;
	fn apply_mod_settings(&mut self, mod_id: &str, collection: &str, settings: Option<&crate::modman::settings::Settings>) -> Result<(), crate::resource_loader::BacktraceError>;
	
	fn get_aeth_meta(&self, mod_id: &str) -> Option<super::meta::Meta>;
	
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
	
	fn is_functional(&self) -> bool {false}
	fn get_mods(&self) -> Vec<String> {Vec::new()}
	fn get_collections(&self) -> Vec<String> {Vec::new()}
	fn install_mod(&mut self, _file: &std::path::Path) -> Result<String, crate::resource_loader::BacktraceError> {Ok(String::new())}
	fn apply_mod_settings(&mut self, _mod_id: &str, _collection: &str, _settings: Option<&crate::modman::settings::Settings>) -> Result<(), crate::resource_loader::BacktraceError> {Ok(())}
	
	fn get_aeth_meta(&self, _mod_id: &str) -> Option<super::meta::Meta> {None}
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