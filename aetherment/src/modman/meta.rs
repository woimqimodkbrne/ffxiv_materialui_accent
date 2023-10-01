use std::{collections::HashMap, path::Path, fs::File, io::Write};
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
	pub file_swaps: HashMap<String, String>,
	pub manipulations: Vec<Manipulation>,
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
			file_swaps: HashMap::new(),
			manipulations: Vec::new(),
		}
	}
}

impl Meta {
	pub fn save(&self, path: &Path) -> std::io::Result<()> {
		// serde_json::to_writer_pretty(&mut File::create(path)?, self)?;
		File::create(path)?.write_all(crate::json_pretty(self)?.as_bytes())?;
		Ok(())
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
	SingleFiles(ValueFiles),
	MultiFiles(ValueFiles),
	Rgb(ValueRgb),
	Rgba(ValueRgba),
	Grayscale(ValueSingle),
	Opacity(ValueSingle),
	Mask(ValueSingle),
	Path(ValuePath),
	// Composite(Composite), // possibly for the future for merging multiple meshes into 1
}

impl EnumTools for OptionSettings {
	type Iterator = std::array::IntoIter<Self, 8>;
	
	fn to_str(&self) -> &'static str {
		match self {
			Self::SingleFiles(_) => "Single Files",
			Self::MultiFiles(_) => "Multi Files",
			Self::Rgb(_) => "RGB",
			Self::Rgba(_) => "RGBA",
			Self::Grayscale(_) => "Grayscale",
			Self::Opacity(_) => "Opacity",
			Self::Mask(_) => "Mask",
			Self::Path(_) => "Path",
		}
	}
	
	fn iter() -> Self::Iterator {
		[
			Self::SingleFiles(ValueFiles::default()),
			Self::MultiFiles(ValueFiles::default()),
			Self::Rgb(ValueRgb::default()),
			Self::Rgba(ValueRgba::default()),
			Self::Grayscale(ValueSingle::default()),
			Self::Opacity(ValueSingle::default()),
			Self::Mask(ValueSingle::default()),
			Self::Path(ValuePath::default()),
		].into_iter()
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
	pub file_swaps: HashMap<String, String>,
	pub manipulations: Vec<Manipulation>,
}

impl Default for ValueFilesOption {
	fn default() -> Self {
		Self {
			name: "New sub option".to_owned(),
			description: String::new(),
			files: HashMap::new(),
			file_swaps: HashMap::new(),
			manipulations: Vec::new(),
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

// ----------

// TODO: this only allows for single path selection, which is an issue for example status icons
// being able to select the shape only affects 1 path (buff, debuff, neutral) instead of all 3
// possibly add a ValuePaths in the future of type Vec<(String, HashMap<id, Path>)> or smth
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ValuePath {
	pub default: u32,
	/// name, path, path does NOT support being Option
	pub options: Vec<(String, super::Path)>,
}

impl Default for ValuePath {
	fn default() -> Self {
		Self {
			default: 0,
			options: Vec::new(),
		}
	}
}

// ----------

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum Manipulation {
	Imc {
		attribute_and_sound: i32,
		material_id: i32,
		decal_id: i32,
		vfx_id: i32,
		material_animation_id: i32,
		attribute_mask: i32,
		sound_id: i32,
		
		primary_id: i32,
		secondary_id: i32,
		variant: i32,
		object_type: String,
		equip_slot: String,
		body_slot: String,
	},
	
	Eqdp {
		entry: u64,
		set_id: i32,
		slot: String,
		race: String,
		gender: String,
	},
	
	Eqp {
		entry: u64,
		set_id: i32,
		slot: String,
	},
	
	Est {
		entry: u64,
		set_id: i32,
		slot: String,
		race: String,
		gender: String,
	},
	
	Gmp {
		enabled: bool,
		animated: bool,
		rotation_a: i32,
		rotation_b: i32,
		rotation_c: i32,
		unknown_a: i32,
		unknown_b: i32,
		unknown_total: i32,
		value: u64,
		
		set_id: i32,
	},
	
	Rsp {
		entry: f32,
		sub_race: String,
		attribute: String,
	},
}