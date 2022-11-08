use std::{collections::HashMap, fs::File, path::PathBuf, io::{Write, Read}};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct ConfigManager {
	mods: HashMap<i32, ModConfig>,
	
	path: PathBuf,
}

impl ConfigManager {
	pub fn load<T>(path: T) -> Self where
	T: Into<PathBuf> {
		Self {
			mods: HashMap::new(),
			path: path.into(),
		}
	}
	
	pub fn mark_for_changes(&mut self) {
		for m in self.mods.values_mut() {
			m.mark_for_changes();
		}
	}
	
	pub fn save(&mut self) {
		for m in self.mods.values_mut() {
			_ = m.save();
		}
	}
	
	pub fn save_forced(&self){
		for m in self.mods.values() {
			_ = m.save_forced();
		}
	}
	
	pub fn get_mod<'a>(&'a mut self, id: i32) -> &'a mut ModConfig {
		self.mods.entry(id).or_insert_with(|| ModConfig::load(self.path.join(id.to_string())))
	}
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct ModConfig {
	pub dalamud: Option<i32>, // TODO
	
	// reason we use a vec instead of hashmap is because hashmap somehow fails string comparisons
	// and ends up with duplicate entries with the same key (wtf?!)
	// perhabs the nightly branch im using has a weird issue, should probably check later
	pub penumbra: Option<Vec<(String, crate::apply::penumbra::Config)>>,
	
	#[serde(skip)] json: String,
	#[serde(skip)] path: PathBuf,
}

impl ModConfig {
	pub fn load<T>(path: T) -> Self where
	T: Into<PathBuf> {
		let path = path.into();
		let mut config = if let Ok(mut f) = File::open(&path) {
			// serde_json::from_reader(f).unwrap()
			let mut buf = Vec::new();
			if f.read_to_end(&mut buf).is_ok() && let Ok(c) = serde_json::from_slice(&buf) {
				c
			} else {
				ModConfig::default()
			}
		} else {
			ModConfig::default()
		};
		
		config.path = path;
		config
	}
	
	pub fn mark_for_changes(&mut self) {
		self.json = serde_json::to_string(self).unwrap();
	}
	
	pub fn save(&mut self) -> std::io::Result<bool> {
		let json = serde_json::to_string(self).unwrap();
		if self.json != json {
			if let Some(parent) = self.path.parent() {std::fs::create_dir_all(parent)?}
			File::create(&self.path)?.write_all(json.as_bytes())?;
			Ok(true)
		} else {
			Ok(false)
		}
	}
	
	pub fn save_forced(&self) -> std::io::Result<()> {
		if let Some(parent) = self.path.parent() {std::fs::create_dir_all(parent)?}
		File::create(&self.path)?.write_all(serde_json::to_string(self)?.as_bytes())?;
		Ok(())
	}
}