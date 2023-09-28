use serde::{Deserialize, Serialize};
use crate::render_helper::EnumTools;

pub mod meta;
pub mod settings;
pub mod composite;

// ----------

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Path {
	Mod(String),
	Game(String),
	Option(String),
}

impl EnumTools for Path {
	type Iterator = std::array::IntoIter<Self, 3>;
	
	fn to_str(&self) -> &'static str {
		match self {
			Self::Mod(_) => "Mod",
			Self::Game(_) => "Game",
			Self::Option(_) => "Option",
		}
	}
	
	fn iter() -> Self::Iterator {
		[
			Self::Mod(String::new()),
			Self::Game(String::new()),
			Self::Option(String::new()),
		].into_iter()
	}
}

// ----------

pub fn cleanup(path: &std::path::Path) -> Result<(), crate::resource_loader::BacktraceError> {
	let meta: meta::Meta = serde_json::from_reader(std::io::BufReader::new(std::fs::File::open(path.join("meta.json"))?))?;
	let mut files = std::collections::HashSet::new();
	
	for (_, path) in &meta.files {
		files.insert(path);
	}
	
	for option in &meta.options {
		if let meta::OptionSettings::SingleFiles(v) | meta::OptionSettings::MultiFiles(v) = &option.settings {
			for sub in &v.options {
				for (_, path) in &sub.files {
					files.insert(path);
				}
			}
		}
	}
	
	// TODO: check comp files, delete files not in the files list, basically finish this
	
	Ok(())
}