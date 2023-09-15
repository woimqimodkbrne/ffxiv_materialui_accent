use std::{fs::File, path::{PathBuf, Path}, io::{Write, Read}};
use serde::{Deserialize, Serialize};

pub struct ConfigManager {
	pub config: Config,
	save_check: Option<Config>,
	path: PathBuf,
}

impl ConfigManager {
	pub fn load(path: &Path) -> Self {
		Self {
			config: 's: {
				if let Ok(mut f) = File::open(path) {
					let mut buf = Vec::new();
					if f.read_to_end(&mut buf).is_ok() {
						if let Ok(c) = serde_json::from_slice(&buf) {
							break 's c;
						}
					}
				}
				
				Config::default()
			},
			save_check: None,
			path: path.to_owned(),
		}
	}
	
	pub fn mark_for_changes(&mut self) {
		self.save_check = Some(self.config.clone());
	}
	
	pub fn save(&mut self) -> std::io::Result<()> {
		if let Some(save_check) = self.save_check.take() {
			if self.config != save_check {
				self.save_forced()?;
			}
		}
		
		Ok(())
	}
	
	pub fn save_forced(&self) -> std::io::Result<()> {
		if let Some(parent) = self.path.parent() {
			std::fs::create_dir_all(parent)?;
		}
		
		File::create(&self.path)?.write_all(serde_json::to_string(&self.config)?.as_bytes())?;
		
		Ok(())
	}
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Config {
	pub local_path: Option<String>,
	pub repos: Vec<String>,
	pub file_dialog_path: PathBuf,
}

impl Default for Config {
	fn default() -> Self {
		Self {
			local_path: None,
			repos: Vec::new(),
			file_dialog_path: dirs::document_dir().unwrap(),
		}
	}
}