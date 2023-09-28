use std::ops::{Deref, DerefMut};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Settings(std::collections::HashMap<String, Value>);

impl Settings {
	pub fn from_meta(meta: &super::meta::Meta) -> Self {
		let mut settings = Self(std::collections::HashMap::new());
		for option in &meta.options {
			settings.insert(option.name.clone(), Value::from_meta_option(option));
		}
		
		settings
	}
}

impl Deref for Settings {
	type Target = std::collections::HashMap<String, Value>;
	
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for Settings {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum Value {
	SingleFiles(u32),
	MultiFiles(u32),
	Rgb([f32; 3]),
	Rgba([f32; 4]),
	Grayscale(f32),
	Opacity(f32),
	Mask(f32),
	Path(u32),
}

impl Value {
	pub fn from_meta_option(option: &super::meta::Option) -> Self {
		match &option.settings {
			super::meta::OptionSettings::SingleFiles(v) => Self::SingleFiles(v.default),
			super::meta::OptionSettings::MultiFiles(v) => Self::MultiFiles(v.default),
			super::meta::OptionSettings::Rgb(v) => Self::Rgb(v.default),
			super::meta::OptionSettings::Rgba(v) => Self::Rgba(v.default),
			super::meta::OptionSettings::Grayscale(v) => Self::Grayscale(v.default),
			super::meta::OptionSettings::Opacity(v) => Self::Opacity(v.default),
			super::meta::OptionSettings::Mask(v) => Self::Mask(v.default),
			super::meta::OptionSettings::Path(v) => Self::Path(v.default),
		}
	}
}