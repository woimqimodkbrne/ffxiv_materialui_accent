use std::{io::{Cursor, Read, Seek}, collections::HashMap, fs::File};
use noumenon::formats::game::tex::Tex;
use serde::{Deserialize, Serialize, Serializer, ser::SerializeSeq};
use crate::GAME;

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct Config {
	pub options: Vec<ConfOption>,
	pub files: HashMap<String, PenumbraFile>,
	pub swaps: HashMap<String, String>,
	pub manipulations: Vec<u32>, // TODO: check if this is actually u32
}

impl Config {
	pub fn update_file<S>(&mut self, opt: &str, subopt: &str, path: S, file: Option<PenumbraFile>) where
	S: Into<String> {
		let files = if opt == "" { // No option
			&mut self.files
		} else {
			match self.options.iter_mut().find(|o| o.name() == opt).unwrap() {
				ConfOption::Single(v) | ConfOption::Multi(v) => &mut v.options.iter_mut().find(|o| o.name == subopt).unwrap().files,
				_ => return,
			}
		};
		
		match file {
			Some(file) => {files.entry(path.into())
				.and_modify(|p| *p = file.clone())
				.or_insert(file);},
			None => {files.remove(&path.into());},
		}
	}
	
	pub fn file_mut(&mut self, opt: &str, subopt: &str, path: &str) -> Option<&mut PenumbraFile> {
		if opt == "" { // No option
			self.files.get_mut(path)
		} else {
			match self.options.iter_mut().find(|o| o.name() == opt).unwrap() {
				ConfOption::Single(v) | ConfOption::Multi(v) => v.options.iter_mut().find(|o| o.name == subopt).unwrap().files.get_mut(path),
				_ => None,
			}
		}
	}
	
	pub fn file_ref(&self, opt: &str, subopt: &str, path: &str) -> Option<&PenumbraFile> {
		if opt == "" { // No option
			self.files.get(path)
		} else {
			match self.options.iter().find(|o| o.name() == opt).unwrap() {
				ConfOption::Single(v) | ConfOption::Multi(v) => v.options.iter().find(|o| o.name == subopt).unwrap().files.get(path),
				_ => None,
			}
		}
	}
	
	pub fn files_ref(&self, opt: &str, subopt: &str) -> Option<&HashMap<String, PenumbraFile>> {
		if opt == "" {
			Some(&self.files)
		} else {
			match self.options.iter().find(|o| o.name() == opt).unwrap() {
				ConfOption::Single(v) | ConfOption::Multi(v) => Some(&v.options.iter().find(|o| o.name == subopt).unwrap().files),
				_ => None,
			}
		}
	}
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum ConfOption {
	Rgb(TypRgb),
	Rgba(TypRgba),
	Grayscale(TypSingle),
	Opacity(TypSingle),
	Mask(TypSingle),
	
	// TODO: probably change these 2 since we gonna use the temp mod api
	Single(TypPenumbra),
	Multi(TypPenumbra),
}

impl<'a> ConfOption {
	pub fn name(&'a self) -> &'a str {
		match self {
			ConfOption::Rgb(v) => &v.name,
			ConfOption::Rgba(v) => &v.name,
			ConfOption::Grayscale(v) => &v.name,
			ConfOption::Opacity(v) => &v.name,
			ConfOption::Mask(v) => &v.name,
			ConfOption::Single(v) => &v.name,
			ConfOption::Multi(v) => &v.name,
		}
	}
	
	pub fn id(&'a self) -> Option<&'a str> {
		match self {
			ConfOption::Rgb(v) => Some(&v.id),
			ConfOption::Rgba(v) => Some(&v.id),
			ConfOption::Grayscale(v) => Some(&v.id),
			ConfOption::Opacity(v) => Some(&v.id),
			ConfOption::Mask(v) => Some(&v.id),
			_ => None,
		}
	}
	
	pub fn type_name(&'a self) -> &'static str {
		match self {
			ConfOption::Rgb(_) => "Rgb",
			ConfOption::Rgba(_) => "Rgba",
			ConfOption::Grayscale(_) => "Grayscale",
			ConfOption::Opacity(_) => "Opacity",
			ConfOption::Mask(_) => "Mask",
			ConfOption::Single(_) => "Single",
			ConfOption::Multi(_) => "Multi",
		}
	}
	
	pub fn is_penumbra(&'a self) -> bool {
		match self {
			ConfOption::Single(_) | ConfOption::Multi(_) => true,
			_ => false,
		}
	}
	
	pub fn default(&'a self) -> ConfSetting {
		match self {
			ConfOption::Rgb(v) => ConfSetting::Rgb(v.default),
			ConfOption::Rgba(v) => ConfSetting::Rgba(v.default),
			ConfOption::Grayscale(v) => ConfSetting::Grayscale(v.default),
			ConfOption::Opacity(v) => ConfSetting::Opacity(v.default),
			ConfOption::Mask(v) => ConfSetting::Mask(v.default),
			_ => ConfSetting::Mask(0.0), // i shouldnt to this but cba atm
		}
	}
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct TypRgb {
	pub id: String,
	pub name: String,
	pub description: String,
	pub default: [f32; 3],
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct TypRgba {
	pub id: String,
	pub name: String,
	pub description: String,
	pub default: [f32; 4],
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct TypSingle {
	pub id: String,
	pub name: String,
	pub description: String,
	pub default: f32,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct TypPenumbra {
	pub name: String,
	pub description: String,
	pub options: Vec<PenumbraOption>,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
#[serde(rename_all = "lowercase")]
pub struct PenumbraOption {
	pub name: String,
	pub files: HashMap<String, PenumbraFile>,
	#[serde(alias = "FileSwaps")] pub swaps: HashMap<String, String>,
	pub manipulations: Vec<u32>, // TODO: check if this is actually u32
}

#[derive(Clone, Debug)]
pub struct PenumbraFile(pub Vec<FileLayer>);

impl PenumbraFile {
	pub fn new_simple(path: &str) -> Self {
		PenumbraFile(vec![
			FileLayer {
				id: None,
				paths: vec![path.to_owned()],
			}
		])
	}
}

#[derive(Clone, Debug)]
pub struct FileLayer {
	pub id: Option<String>,
	pub paths: Vec<String>,
}

impl<'de> Deserialize<'de> for PenumbraFile {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
	D: serde::Deserializer<'de> {
		#[derive(Deserialize)]
		#[serde(untagged)]
		enum Paths {
			Simple(String),
			Complex(Vec<Vec<Option<String>>>),
		}
		
		let a: Paths = Deserialize::deserialize(deserializer)?;
		Ok(match a {
			Paths::Simple(v) => PenumbraFile(vec![FileLayer {
				id: None,
				paths: vec![v],
			}]),
			Paths::Complex(v) => PenumbraFile(
				// TODO: dont use unwrap
				v.into_iter().map(|v| {
					let mut segs = v.into_iter();
					FileLayer {
						id: segs.next().unwrap(),
						paths: segs.map(|p| p.unwrap()).collect(),
					}
				}).collect()
			),
		})
	}
}

impl Serialize for PenumbraFile {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
	S: Serializer {
		let mut layers = serializer.serialize_seq(Some(self.0.len()))?;
		for layer in &self.0 {
			let mut paths = vec![layer.id.as_ref()];
			for p in &layer.paths {
				paths.push(Some(p));
			}
			layers.serialize_element(&paths)?;
		}
		layers.end()
	}
}

#[derive(Deserialize, Clone, Copy, Debug)]
#[serde(rename_all = "lowercase", untagged)]
pub enum ConfSetting {
	Rgb([f32; 3]),
	Rgba([f32; 4]),
	Grayscale(f32),
	Opacity(f32),
	Mask(f32),
}

impl ConfSetting {
	pub fn draw(&mut self, label: &str) -> bool {
		match self {
			Self::Rgb(v) => imgui::color_edit3(label, v, imgui::ColorEditFlags::PickerHueWheel | imgui::ColorEditFlags::NoInputs),
			Self::Rgba(v) => imgui::color_edit4(label, v, imgui::ColorEditFlags::PickerHueWheel | imgui::ColorEditFlags::NoInputs | imgui::ColorEditFlags::AlphaBar | imgui::ColorEditFlags::AlphaPreviewHalf),
			Self::Grayscale(v) => {
				let mut v2 = *v * 255.0;
				let r = imgui::drag_float(label, &mut v2, 0.0, 0.0, 255.0, "%f", imgui::SliderFlags::NoRoundToFormat);
				*v = v2 / 255.0;
				r
			},
			Self::Opacity(v) => {
				let mut v2 = *v * 255.0;
				let r = imgui::drag_float(label, &mut v2, 0.0, 0.0, 255.0, "%f", imgui::SliderFlags::NoRoundToFormat);
				*v = v2 / 255.0;
				r
			},
			Self::Mask(v) => {
				let mut v2 = *v * 100.0;
				let r = imgui::drag_float(label, &mut v2, 0.0, 0.0, 100.0, "%.1f%%", imgui::SliderFlags::NoRoundToFormat);
				*v = v2 / 100.0;
				r
			}
		}
	}
}

#[derive(Clone, Debug)]
pub struct Layer {
	pub value: Option<ConfSetting>,
	pub files: Vec<String>,
}

pub fn load_file(path: &str) -> Option<Vec<u8>> {
	// TODO: allow reading from mods with lower priority
	if let Ok(mut f) = File::open(path) {
		let mut buf = Vec::with_capacity(f.stream_len().unwrap() as usize);
		f.read_to_end(&mut buf).unwrap();
		Some(buf)
	} else {
		GAME.file::<Vec<u8>>(path).ok()
	}
}

// Might not want to return tex, idk yet
pub fn resolve_layer(layer: &Layer, mut load_file: impl FnMut(&str) -> Option<Vec<u8>>) -> Result<Tex, String> {
	Ok(if let Some(v) = &layer.value {
		match v {
			ConfSetting::Rgb(val) => {
				let mut tex = Tex::read(&mut Cursor::new(&load_file(&layer.files[0]).ok_or(layer.files[0].clone())?));
				tex.as_pixels_mut().iter_mut().for_each(|pixel| {
					pixel.b = (pixel.b as f32 * val[2]) as u8;
					pixel.g = (pixel.g as f32 * val[1]) as u8;
					pixel.r = (pixel.r as f32 * val[0]) as u8;
				});
				tex
			},
			ConfSetting::Rgba(val) => {
				let mut tex = Tex::read(&mut Cursor::new(&load_file(&layer.files[0]).ok_or(layer.files[0].clone())?));
				tex.as_pixels_mut().iter_mut().for_each(|pixel| {
					pixel.b = (pixel.b as f32 * val[2]) as u8;
					pixel.g = (pixel.g as f32 * val[1]) as u8;
					pixel.r = (pixel.r as f32 * val[0]) as u8;
					pixel.a = (pixel.r as f32 * val[3]) as u8;
				});
				tex
			},
			ConfSetting::Grayscale(val) => {
				let mut tex = Tex::read(&mut Cursor::new(&load_file(&layer.files[0]).ok_or(layer.files[0].clone())?));
				tex.as_pixels_mut().iter_mut().for_each(|pixel| {
					pixel.b = (pixel.b as f32 * val) as u8;
					pixel.g = (pixel.g as f32 * val) as u8;
					pixel.r = (pixel.r as f32 * val) as u8;
				});
				tex
			},
			ConfSetting::Opacity(val) => {
				let mut tex = Tex::read(&mut Cursor::new(&load_file(&layer.files[0]).ok_or(layer.files[0].clone())?));
				tex.as_pixels_mut().iter_mut().for_each(|pixel| {
					pixel.a = (pixel.a as f32 * val) as u8;
				});
				tex
			},
			ConfSetting::Mask(val) => {
				let val = (val * 255f32) as u8;
				let mut tex = Tex::read(&mut Cursor::new(&load_file(&layer.files[0]).ok_or(layer.files[0].clone())?));
				let mask = Tex::read(&mut Cursor::new(&load_file(&layer.files[1]).ok_or(layer.files[1].clone())?));
				let mask_pixels = mask.as_pixels();
				tex.as_pixels_mut().iter_mut().enumerate().for_each(|(i, pixel)| {
					pixel.a = if val >= mask_pixels[i].r {pixel.a} else {0};
				});
				tex
			},
		}
	} else {
		Tex::read(&mut Cursor::new(&load_file(&layer.files[0]).ok_or(layer.files[0].clone())?))
	})
}