use std::{io::Cursor, collections::HashMap};
use noumenon::formats::game::tex::Tex;
use serde::Deserialize;
use crate::gui::imgui;

#[derive(Deserialize)]
pub struct Config {
	pub options: Vec<ConfOption>,
	pub files: HashMap<String, PenumbraFile>,
	pub swaps: HashMap<String, String>,
	pub manipulations: Vec<u32>, // TODO: check if this is actually u32
}

#[derive(Deserialize)]
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

#[derive(Deserialize)]
pub struct TypRgba {
	pub id: String,
	pub name: String,
	pub description: String,
	pub default: [f32; 4],
}

#[derive(Deserialize)]
pub struct TypRgb {
	pub id: String,
	pub name: String,
	pub description: String,
	pub default: [f32; 3],
}

#[derive(Deserialize)]
pub struct TypSingle {
	pub id: String,
	pub name: String,
	pub description: String,
	pub default: f32,
}

#[derive(Deserialize)]
pub struct TypPenumbra {
	pub name: String,
	pub description: String,
	pub options: Vec<PenumbraOption>,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct PenumbraOption {
	pub name: String,
	pub files: HashMap<String, PenumbraFile>,
	#[serde(alias = "FileSwaps")] pub swaps: HashMap<String, String>,
	pub manipulations: Vec<u32>, // TODO: check if this is actually u32
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum PenumbraFile {
	Simple(String),
	Complex(Vec<Vec<Option<String>>>),
}

impl PenumbraFile {
	pub fn complex(&self) -> Vec<Vec<Option<String>>> {
		match self {
			Self::Simple(v) => vec![vec![None, Some(v.to_string())]],
			Self::Complex(v) => v.clone(),
		}
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
			Self::Rgba(v) => imgui::color_edit4(label, v, imgui::ColorEditFlags::PickerHueWheel | imgui::ColorEditFlags::NoInputs),
			_ => todo!(),
		}
	}
}

#[derive(Clone, Debug)]
pub struct Layer {
	pub value: Option<ConfSetting>,
	pub files: Vec<String>,
}

// Might not want to return tex, idk yet
pub fn resolve_layer(layer: &Layer, mut load_file: impl FnMut(&str) -> Option<Vec<u8>>) -> Option<Tex> {
	Some(if let Some(v) = &layer.value {
		match v {
			ConfSetting::Rgb(val) => {
				let mut tex = Tex::read(&mut Cursor::new(&load_file(&layer.files[0])?));
				tex.as_pixels_mut().iter_mut().for_each(|pixel| {
					pixel.b = (pixel.b as f32 * val[2]) as u8;
					pixel.g = (pixel.g as f32 * val[1]) as u8;
					pixel.r = (pixel.r as f32 * val[0]) as u8;
				});
				tex
			},
			ConfSetting::Rgba(val) => {
				let mut tex = Tex::read(&mut Cursor::new(&load_file(&layer.files[0])?));
				tex.as_pixels_mut().iter_mut().for_each(|pixel| {
					pixel.b = (pixel.b as f32 * val[2]) as u8;
					pixel.g = (pixel.g as f32 * val[1]) as u8;
					pixel.r = (pixel.r as f32 * val[0]) as u8;
					pixel.a = (pixel.r as f32 * val[3]) as u8;
				});
				tex
			},
			ConfSetting::Grayscale(val) => {
				let mut tex = Tex::read(&mut Cursor::new(&load_file(&layer.files[0])?));
				tex.as_pixels_mut().iter_mut().for_each(|pixel| {
					pixel.b = (pixel.b as f32 * val) as u8;
					pixel.g = (pixel.g as f32 * val) as u8;
					pixel.r = (pixel.r as f32 * val) as u8;
				});
				tex
			},
			ConfSetting::Opacity(val) => {
				let mut tex = Tex::read(&mut Cursor::new(&load_file(&layer.files[0])?));
				tex.as_pixels_mut().iter_mut().for_each(|pixel| {
					pixel.a = (pixel.a as f32 * val) as u8;
				});
				tex
			},
			ConfSetting::Mask(val) => {
				let val = (val * 255f32) as u8;
				let mut tex = Tex::read(&mut Cursor::new(&load_file(&layer.files[0])?));
				let mask = Tex::read(&mut Cursor::new(&load_file(&layer.files[1])?));
				let mask_pixels = mask.as_pixels();
				tex.as_pixels_mut().iter_mut().enumerate().for_each(|(i, pixel)| {
					pixel.a = if val >= mask_pixels[i].r {pixel.a} else {0};
				});
				tex
			},
		}
	} else {
		Tex::read(&mut Cursor::new(&load_file(&layer.files[0])?))
	})
}