use std::borrow::Cow;
use serde::{Deserialize, Serialize};
use crate::{modman::{settings::Value as SettingsValue, meta::OptionSettings, Path}, render_helper::EnumTools};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Tex {
	pub layers: Vec<Layer>,
}

impl Tex {
	pub fn composite(&self, _meta: &crate::modman::meta::Meta, settings: &crate::modman::settings::Settings, textures: &std::collections::HashMap<&Path, &noumenon::format::game::Tex>) -> Option<Vec<u8>> {
		let mut layers = self.layers.iter().rev();
		
		let layer = layers.next()?;
		let tex = textures.get(&layer.path)?;
		let (width, height) = (tex.header.width as u32, tex.header.height as u32);
		let mut data = tex.data.clone();
		
		let apply_modifiers = |data: &mut [u8]| -> Option<()> {
			for modifier in layer.modifiers.iter().rev() {
				match modifier {
					Modifier::AlphaMask{path, cull_point} => {
						let cull_point = cull_point.get_value(settings)?;
						let tex = textures.get(path)?;
						let (w, h) = (tex.header.width as u32, tex.header.height as u32);
						let mask_data = get_resized(tex, w, h, width, height);
						
						for (i, pixel) in data.chunks_exact_mut(4).enumerate() {
							if mask_data[i * 4 + 3] as f32 / 255.0 < cull_point {
								pixel[0] = 0;
								pixel[1] = 0;
								pixel[2] = 0;
								pixel[3] = 0;
							}
						}
					}
					
					Modifier::Color{value} => {
						let color = value.get_value(settings)?;
						for pixel in data.chunks_exact_mut(4) {
							pixel[0] = (pixel[0] as f32 * color[0]).min(255.0) as u8;
							pixel[1] = (pixel[1] as f32 * color[1]).min(255.0) as u8;
							pixel[2] = (pixel[2] as f32 * color[2]).min(255.0) as u8;
							pixel[3] = (pixel[3] as f32 * color[3]).min(255.0) as u8;
						}
					}
				}
			}
			
			Some(())
		};
		
		apply_modifiers(&mut data)?;
		
		for layer in layers {
			let tex = textures.get(&layer.path)?;
			let (w, h) = (tex.header.width as u32, tex.header.height as u32);
			let layer_data = get_resized(tex, w, h, width, height);
			
			apply_modifiers(&mut data)?;
			
			match layer.blend {
				Blend::Normal => {
					for (i, pixel) in data.chunks_exact_mut(4).enumerate() {
						let ar = pixel[3] as f32 / 255.0;
						let ao = layer_data[i * 4 + 3] as f32 / 255.0;
						let a = ao + ar * (1.0 - ao);
						
						pixel[0] = ((layer_data[i * 4 + 0] as f32 * ao + pixel[0] as f32 * ar * (1.0 - ao)) / a) as u8;
						pixel[1] = ((layer_data[i * 4 + 1] as f32 * ao + pixel[1] as f32 * ar * (1.0 - ao)) / a) as u8;
						pixel[2] = ((layer_data[i * 4 + 2] as f32 * ao + pixel[2] as f32 * ar * (1.0 - ao)) / a) as u8;
						pixel[3] = (a * 255.0) as u8;
					}
				}
			}
		}
		
		Some(data)
	}
}

impl super::Composite for Tex {
	fn get_files(&self) -> Vec<&str> {
		let mut files = Vec::new();
		for layer in &self.layers {
			if let Path::Mod(path) = &layer.path {
				files.push(path.as_str());
			}
			
			for modifier in &layer.modifiers {
				match modifier {
					Modifier::AlphaMask{path, ..} => {
						if let Path::Mod(path) = path {
							files.push(path.as_str());
						}
					}
					_ => {}
				}
			}
		}
		
		files
	}
}

fn get_resized(tex: &noumenon::format::game::Tex, width: u32, height: u32, target_width: u32, target_height: u32) -> Cow<Vec<u8>> {
	if width != target_width || height != target_height {
		Cow::Owned(image::imageops::resize(tex, width, height, image::imageops::FilterType::Nearest).into_vec())
	} else {
		Cow::Borrowed(&tex.data)
	}
}

// ----------

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Layer {
	pub name: String,
	pub path: Path,
	pub modifiers: Vec<Modifier>,
	pub blend: Blend,
}

impl std::hash::Hash for Layer {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.name.hash(state);
		self.path.hash(state);
		self.blend.hash(state);
	}
}

// ----------

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Blend {
	Normal,
}

impl EnumTools for Blend {
	type Iterator = std::array::IntoIter<Self, 1>;
	
	fn to_str(&self) -> &'static str {
		match self {
			Self::Normal => "Normal",
		}
	}
	
	fn iter() -> Self::Iterator {
		[Self::Normal].into_iter()
	}
}

// ----------

#[derive(Debug, Clone, PartialEq, Hash, Deserialize, Serialize)]
pub enum Modifier {
	AlphaMask {
		path: Path,
		cull_point: OptionOrStatic<MaskOption>,
	},
	
	Color {
		value: OptionOrStatic<ColorOption>,
	}
}

impl EnumTools for Modifier {
	type Iterator = std::array::IntoIter<Self, 2>;
	
	fn to_str(&self) -> &'static str {
		match self {
			Self::AlphaMask{..} => "Alpha Mask",
			Self::Color{..} => "Color",
		}
	}
	
	fn iter() -> Self::Iterator {
		[
			Self::AlphaMask{path: Path::Mod(String::new()), cull_point: OptionOrStatic::Static(1.0)},
			Self::Color{value: OptionOrStatic::Static([1.0, 1.0, 1.0, 1.0])},
		].into_iter()
	}
}

// ----------

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum OptionOrStatic<T: OptionSetting + Sized + Default> {
	Option(T),
	Static(T::Value),
}

impl<T: OptionSetting + Sized + Default> OptionOrStatic<T> {
	pub fn get_value(&self, settings: &crate::modman::settings::Settings) -> Option<T::Value> {
		match self {
			Self::Static(v) => Some(v.clone()),
			Self::Option(t) => t.get_value(settings.get(t.option_id())?),
		}
	}
}

// this is NOT a proper hash!!! it only hashes the pointer of the value so it can be used in drag n drop elements
impl<T: OptionSetting + Sized + Default> std::hash::Hash for OptionOrStatic<T> {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		match self {
			Self::Option(t) => (t as *const _ as usize).hash(state),
			Self::Static(t) => (t as *const _ as usize).hash(state),
		}
	}
}

impl<T: OptionSetting + Sized + Default> EnumTools for OptionOrStatic<T> {
	type Iterator = std::array::IntoIter<Self, 2>;
	
	fn to_str(&self) -> &'static str {
		match self {
			Self::Option(_) => "Option",
			Self::Static(_) => "Static",
		}
	}
	
	fn iter() -> Self::Iterator {
		[
			Self::Option(T::default()),
			Self::Static(T::Value::default()),
		].into_iter()
	}
}

// ----------

pub trait OptionSetting {
	type Value: Clone + Default + PartialEq;
	
	fn option_id(&self) -> &str;
	fn option_id_mut(&mut self) -> &mut String;
	fn get_value(&self, settings_value: &SettingsValue) -> Option<Self::Value>;
	fn valid_option(&self, option: &OptionSettings) -> bool;
}

// ----------

#[derive(Debug, Clone, Default, PartialEq, Hash, Deserialize, Serialize)]
pub struct ColorOption(String);
impl OptionSetting for ColorOption {
	type Value = [f32; 4];
	
	fn option_id(&self) -> &str {
		&self.0
	}
	
	fn option_id_mut(&mut self) -> &mut String {
		&mut self.0
	}
	
	fn get_value(&self, settings_value: &SettingsValue) -> Option<Self::Value> {
		match settings_value {
			SettingsValue::Rgba(v) => Some(*v),
			SettingsValue::Rgb(v) => Some([v[0], v[1], v[2], 1.0]),
			SettingsValue::Grayscale(v) => Some([*v, *v, *v, 1.0]),
			SettingsValue::Opacity(v) => Some([1.0, 1.0, 1.0, *v]),
			_ => None,
		}
	}
	
	fn valid_option(&self, option: &OptionSettings) -> bool {
		match option {
			OptionSettings::Rgb(_) |
			OptionSettings::Rgba(_) |
			OptionSettings::Grayscale(_) |
			OptionSettings::Opacity(_) => true,
			_ => false,
		}
	}
}

// ----------

#[derive(Debug, Clone, Default, PartialEq, Hash, Deserialize, Serialize)]
pub struct MaskOption(String);
impl OptionSetting for MaskOption {
	type Value = f32;
	
	fn option_id(&self) -> &str {
		&self.0
	}
	
	fn option_id_mut(&mut self) -> &mut String {
		&mut self.0
	}
	
	fn get_value(&self, settings_value: &SettingsValue) -> Option<Self::Value> {
		match settings_value {
			SettingsValue::Mask(v) => Some(*v),
			_ => None,
		}
	}
	
	fn valid_option(&self, option: &OptionSettings) -> bool {
		match option {
			OptionSettings::Mask(_) => true,
			_ => false,
		}
	}
}