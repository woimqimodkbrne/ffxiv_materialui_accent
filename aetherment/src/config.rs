use std::{collections::HashMap, fs::File, path::PathBuf, io::{Write, Read}};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
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
	pub fn load<T>(path: T) -> Self where
	T: Into<PathBuf> {
		let path = path.into();
		let mut config = if let Ok(mut f) = File::open(&path) {
			// serde_json::from_reader(f).unwrap()
			let mut buf = Vec::new();
			if f.read_to_end(&mut buf).is_ok() && let Ok(c) = serde_json::from_slice(&buf) {
				c
			} else {
				Config::default()
			}
		} else {
			Config::default()
		};
		
		config.path = path;
		config
	}
	
	pub fn mark_for_changes(&mut self) {
		self.json = serde_json::to_string(self).unwrap();
	}
	
	pub fn save(&mut self) -> std::io::Result<()> {
		let json = serde_json::to_string(self).unwrap();
		if self.json != json {
			std::fs::create_dir_all(self.path.parent().unwrap())?;
			File::create(&self.path)?.write_all(json.as_bytes())?;
		}
		Ok(())
	}
	
	pub fn save_forced(&self) -> std::io::Result<()> {
		if let Some(parent) = self.path.parent() {
			std::fs::create_dir_all(parent)?;
		}
		
		File::create(&self.path)?.write_all(serde_json::to_string(self)?.as_bytes())?;
		Ok(())
	}
}