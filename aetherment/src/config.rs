use std::{collections::HashMap, fs::File, path::{PathBuf, Path}, io::Write};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[derive(Debug, Clone)]
#[serde(default)]
pub struct Config {
	pub local_path: String,
	pub tab_explorer: bool,
	pub tab_moddev: bool,
	
	pub explorer_path: String,
	pub explorer_exts: HashMap<String, String>,
	
	#[serde(skip)] json: String,
	#[serde(skip)] path: PathBuf,
}

impl Default for Config {
	fn default() -> Self {
		Self {
			local_path: "".to_owned(),
			tab_explorer: false,
			tab_moddev: false,
			
			explorer_path: dirs::document_dir().unwrap().to_string_lossy().to_string(),
			explorer_exts: HashMap::new(),
			
			json: "".to_owned(),
			path: PathBuf::new(),
		}
	}
}

impl Config {
	pub fn load<T>(path: T) -> Self where T: AsRef<Path> {
		let mut config = if let Ok(f) = File::open(&path) {
			serde_json::from_reader(f).unwrap()
		} else {
			Config::default()
		};
		
		config.path = path.as_ref().to_owned();
		config.local_path.reserve(128);
		config
	}
	
	pub fn mark_for_changes(&mut self) {
		self.json = serde_json::json!(self).to_string();
	}
	
	pub fn save(&mut self) -> std::io::Result<()> {
		let json = serde_json::json!(self).to_string();
		if self.json != json {
			File::create(&self.path)?.write_all(json.as_bytes())?;
		}
		Ok(())
	}
}