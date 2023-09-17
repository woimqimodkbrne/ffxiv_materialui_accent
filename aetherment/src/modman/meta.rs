use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::render_helper::EnumTools;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Meta {
	pub name: String,
	pub description: String,
	pub version: String, // TODO: regex to make sure it follows semver (https://semver.org)
	pub author: String,
	pub website: String,
	pub tags: Vec<String>,
	pub dependencies: Vec<String>,
	pub options: Vec<Option>,
	
	pub files: HashMap<String, String>,
	// pub file_swaps: _,
	// pub manipulations: _,
}

impl Default for Meta {
	fn default() -> Self {
		Self {
			name: "New Mod".to_string(),
			description: String::new(),
			version: "0.0.0".to_string(),
			author: String::new(),
			website: String::new(),
			tags: Vec::new(),
			dependencies: Vec::new(),
			options: Vec::new(),
			
			files: HashMap::new(),
		}
	}
}

// ----------

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Option {
	pub name: String,
	pub description: String,
	pub settings: OptionSettings,
}

impl std::hash::Hash for Option {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.name.hash(state);
		self.description.hash(state);
		self.settings.to_str().hash(state);
	}
}

// ----------

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum OptionSettings {
	Single(ValueFiles),
	Multi(ValueFiles),
	Rgb(ValueRgb),
	Rgba(ValueRgba),
	Grayscale(ValueSingle),
	Opacity(ValueSingle),
	Mask(ValueSingle),
	// Composite(Composite), // possibly for the future for merging multiple meshes into 1
}

impl EnumTools for OptionSettings {
	type Iterator = std::array::IntoIter<Self, 7>;
	
	fn to_str(&self) -> &'static str {
		match self {
			Self::Single(_) => "Single",
			Self::Multi(_) => "Multi",
			Self::Rgb(_) => "RGB",
			Self::Rgba(_) => "RGBA",
			Self::Grayscale(_) => "Grayscale",
			Self::Opacity(_) => "Opacity",
			Self::Mask(_) => "Mask",
		}
	}
	
	fn iter() -> Self::Iterator {
		[Self::Single(Default::default()), Self::Multi(Default::default()), Self::Rgb(Default::default()), Self::Rgba(Default::default()), Self::Grayscale(Default::default()), Self::Opacity(Default::default()), Self::Mask(Default::default())].into_iter()
	}
}

// ----------

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ValueFiles {
	pub default: u32, // TODO: perhabs dupe this struct and have default value be a vec of bools for multi
	pub options: Vec<ValueFilesOption>,
}

impl Default for ValueFiles {
	fn default() -> Self {
		Self {
			default: 0,
			options: vec![],
		}
	}
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ValueFilesOption {
	pub name: String,
	pub description: String,
	pub files: HashMap<String, String>,
	// TODO: these 2!! check at the penumbra source to see tf these are
	// pub file_swaps: _,
	// pub manipulations: _,
}

impl Default for ValueFilesOption {
	fn default() -> Self {
		Self {
			name: "New sub option".to_owned(),
			description: String::new(),
			files: HashMap::new(),
		}
	}
}

impl std::hash::Hash for ValueFilesOption {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.name.hash(state);
		self.description.hash(state);
	}
}

// ----------

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ValueRgb {
	pub default: [f32; 3],
	pub min: [f32; 3],
	pub max: [f32; 3],
}

impl Default for ValueRgb {
	fn default() -> Self {
		Self {
			default: [1.0, 1.0, 1.0],
			min: [0.0, 0.0, 0.0],
			max: [1.0, 1.0, 1.0],
		}
	}
}

// ----------

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ValueRgba {
	pub default: [f32; 4],
	pub min: [f32; 4],
	pub max: [f32; 4],
}

impl Default for ValueRgba {
	fn default() -> Self {
		Self {
			default: [1.0, 1.0, 1.0, 1.0],
			min: [0.0, 0.0, 0.0, 0.0],
			max: [1.0, 1.0, 1.0, 1.0],
		}
	}
}

// ----------

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ValueSingle {
	pub default: f32,
	pub min: f32,
	pub max: f32,
}

impl Default for ValueSingle {
	fn default() -> Self {
		Self {
			default: 0.0,
			min: 0.0,
			max: 1.0,
		}
	}
}