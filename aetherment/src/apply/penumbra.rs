use std::{io::{Cursor, Read, Seek}, collections::{HashMap, HashSet}, fs::File};
use noumenon::formats::game::tex::Tex;
use serde::{Deserialize, Serialize, Serializer, ser::SerializeSeq};
use crate::GAME;

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct Config {
	pub settings: HashMap<String, ConfSetting>,
	pub filter: HashSet<String>,
	pub filter_is_whitelist: bool,
	
	#[serde(skip)] pub map: Option<HashMap<String, String>>,
	#[serde(skip)] pub options: Option<Vec<MetaOptionUnique>>,
}

impl Config {
	pub fn load_optionals(&mut self, mod_path: &std::path::Path) -> std::io::Result<()> {
		let mut buf = Vec::new();
		
		if self.map.is_none() {
			log!("loading map {:?}", mod_path.join("map.json"));
			let mut f = File::open(mod_path.join("map.json"))?;
			f.read_to_end(&mut buf)?;
			self.map = Some(serde_json::from_slice(&buf)?);
		}
		
		if self.options.is_none() && let Some(map) = &self.map && let Some(datas_path) = map.get("datas.json") {
			log!("loading options {:?}", mod_path.join("files").join(datas_path));
			buf.clear();
			let mut f = File::open(mod_path.join("files").join(datas_path))?;
			f.read_to_end(&mut buf)?;
			self.options = Some(serde_json::from_slice::<crate::apply::Datas>(&buf)?.penumbra.unwrap().options);
		}
		
		Ok(())
	}
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct MetaOptionUnique(String, MetaOption);
impl MetaOptionUnique {
	pub fn new(option: MetaOption) -> Self {
		Self(crate::random_str(8), option)
	}
	
	pub fn new_raw<S>(unique: S, option: MetaOption) -> Self where
	S: Into<String> {
		Self(unique.into(), option)
	}
	
	pub fn unique(&self) -> &str {
		&self.0
	}
	
	// needed because deref doesnt work on enums and i dont feel like importing deref everywhere
	pub fn deref(&self) -> &MetaOption {
		&self.1
	}
	
	pub fn deref_mut(&mut self) -> &mut MetaOption {
		&mut self.1
	}
}

impl std::ops::Deref for MetaOptionUnique {
	type Target = MetaOption;
	
	fn deref(&self) -> &Self::Target {
		&self.1
	}
}

impl std::ops::DerefMut for MetaOptionUnique {
    fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.1
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct Meta {
	// tuple since that way its orderable
	pub options: Vec<MetaOptionUnique>, // (unique, option)
	pub files: HashMap<String, PenumbraFile>,
	pub swaps: HashMap<String, String>,
	pub manipulations: Vec<Manipulation>,
}

impl Meta {
	pub fn update_file<S>(&mut self, opt: &str, subopt: &str, path: S, file: Option<PenumbraFile>) where
	S: Into<String> {
		let files = if opt == "" { // No option
			&mut self.files
		} else {
			match self.options.iter_mut().find(|o| o.name() == opt).unwrap().deref_mut() {
				MetaOption::Single(v) | MetaOption::Multi(v) => &mut v.options.iter_mut().find(|o| o.name == subopt).unwrap().files,
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
			match self.options.iter_mut().find(|o| o.name() == opt).unwrap().deref_mut() {
				MetaOption::Single(v) | MetaOption::Multi(v) => v.options.iter_mut().find(|o| o.name == subopt).unwrap().files.get_mut(path),
				_ => None,
			}
		}
	}
	
	pub fn file_ref(&self, opt: &str, subopt: &str, path: &str) -> Option<&PenumbraFile> {
		if opt == "" { // No option
			self.files.get(path)
		} else {
			match self.options.iter().find(|o| o.name() == opt).unwrap().deref() {
				MetaOption::Single(v) | MetaOption::Multi(v) => v.options.iter().find(|o| o.name == subopt).unwrap().files.get(path),
				_ => None,
			}
		}
	}
	
	pub fn files_ref(&self, opt: &str, subopt: &str) -> Option<&HashMap<String, PenumbraFile>> {
		if opt == "" {
			Some(&self.files)
		} else {
			match self.options.iter().find(|o| o.name() == opt).unwrap().deref() {
				MetaOption::Single(v) | MetaOption::Multi(v) => Some(&v.options.iter().find(|o| o.name == subopt).unwrap().files),
				_ => None,
			}
		}
	}
}

// i dont like that it will use PascalCase even in aeth datas.json, but oh well
#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(tag = "Type", content = "Manipulation")]
pub enum Manipulation {
	Eqp(ManiEqp),
	Eqdp(ManiEqdp),
	Imc(ManiImc),
	Est(ManiEst),
	Gmp(ManiGmp),
	Rsp(ManiRsp),
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ManiEqp {
	pub entry: u64,
	pub set_id: u16,
	pub slot: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ManiEqdp {
	pub entry: u16,
	pub set_id: u16,
	pub slot: String,
	pub gender: String,
	pub race: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ManiImc {
	pub primary_id: u16,
	pub secondary_id: u16,
	pub variant: u16,
	pub body_slot: String,
	pub equip_slot: String,
	pub object_type: String,
	pub entry: ManiImcEntry,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ManiImcEntry {
	pub material_id: u8,
	pub decal_id: u8,
	pub vfx_id: u8,
	pub material_animation_id: u8,
	pub sound_id: u8,
	pub attribute_mask: u16,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ManiEst {
	pub entry: u16,
	pub set_id: u16,
	pub slot: String,
	pub gender: String,
	pub race: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ManiGmp {
	pub set_id: u16,
	pub entry: ManiGmpEntry,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ManiGmpEntry {
	pub enabled: bool,
	pub animated: bool,
	pub rotation_a: u16,
	pub rotation_b: u16,
	pub rotation_c: u16,
	pub unknown_a: u8,
	pub unknown_b: u8,
	pub unknown_total: u8,
	pub value: u64,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ManiRsp {
	pub entry: f32,
	pub sub_race: String,
	pub attribute: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum MetaOption {
	Rgb(TypRgb),
	Rgba(TypRgba),
	Grayscale(TypSingle),
	Opacity(TypSingle),
	Mask(TypSingle),
	
	// TODO: probably change these 2 since we gonna use the temp mod api
	Single(TypPenumbra),
	Multi(TypPenumbra),
}

impl<'a> MetaOption {
	pub fn name(&'a self) -> &'a str {
		match self {
			MetaOption::Rgb(v) => &v.name,
			MetaOption::Rgba(v) => &v.name,
			MetaOption::Grayscale(v) => &v.name,
			MetaOption::Opacity(v) => &v.name,
			MetaOption::Mask(v) => &v.name,
			MetaOption::Single(v) => &v.name,
			MetaOption::Multi(v) => &v.name,
		}
	}
	
	// pub fn id(&'a self) -> Option<&'a str> {
	// 	match self {
	// 		MetaOption::Rgb(v) => Some(&v.id),
	// 		MetaOption::Rgba(v) => Some(&v.id),
	// 		MetaOption::Grayscale(v) => Some(&v.id),
	// 		MetaOption::Opacity(v) => Some(&v.id),
	// 		MetaOption::Mask(v) => Some(&v.id),
	// 		_ => None,
	// 	}
	// }
	
	pub fn type_name(&'a self) -> &'static str {
		match self {
			MetaOption::Rgb(_) => "Rgb",
			MetaOption::Rgba(_) => "Rgba",
			MetaOption::Grayscale(_) => "Grayscale",
			MetaOption::Opacity(_) => "Opacity",
			MetaOption::Mask(_) => "Mask",
			MetaOption::Single(_) => "Single",
			MetaOption::Multi(_) => "Multi",
		}
	}
	
	pub fn is_penumbra(&'a self) -> bool {
		match self {
			MetaOption::Single(_) | MetaOption::Multi(_) => true,
			_ => false,
		}
	}
	
	pub fn default(&'a self) -> ConfSetting {
		match self {
			MetaOption::Rgb(v) => ConfSetting::Rgb(v.default),
			MetaOption::Rgba(v) => ConfSetting::Rgba(v.default),
			MetaOption::Grayscale(v) => ConfSetting::Grayscale(v.default),
			MetaOption::Opacity(v) => ConfSetting::Opacity(v.default),
			MetaOption::Mask(v) => ConfSetting::Mask(v.default),
			MetaOption::Single(_) => ConfSetting::Single(0),
			MetaOption::Multi(_) => ConfSetting::Multi(0),
		}
	}
}

pub type ValueRgb = [f32; 3];
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct TypRgb {
	// pub id: String,
	pub name: String,
	pub description: String,
	pub default: ValueRgb,
}

pub type ValueRgba = [f32; 4];
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct TypRgba {
	// pub id: String,
	pub name: String,
	pub description: String,
	pub default: ValueRgba,
}

pub type ValueSingle = f32;
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct TypSingle {
	// pub id: String,
	pub name: String,
	pub description: String,
	pub default: ValueSingle,
}

// TODO: possbily add default to this and rename it to `TypSelect`
pub type ValuePenumbra = i32;
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct TypPenumbra {
	pub name: String,
	pub description: String,
	pub options: Vec<PenumbraOption>,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct PenumbraOption {
	pub name: String,
	pub files: HashMap<String, PenumbraFile>,
	pub swaps: HashMap<String, String>,
	pub manipulations: Vec<Manipulation>,
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

impl std::ops::Deref for PenumbraFile {
	type Target = Vec<FileLayer>;
	
	fn deref(&self) -> &Self::Target {
		&self.0
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
		let mut layers = serializer.serialize_seq(Some(self.len()))?;
		for layer in self.iter() {
			let mut paths = vec![layer.id.as_ref()];
			for p in &layer.paths {
				paths.push(Some(p));
			}
			layers.serialize_element(&paths)?;
		}
		layers.end()
	}
}

#[derive(Deserialize, Serialize, Clone, Copy, Debug)]
#[serde(rename_all = "lowercase", tag = "type", content = "value")]
pub enum ConfSetting {
	Rgb(ValueRgb),
	Rgba(ValueRgba),
	Grayscale(ValueSingle),
	Opacity(ValueSingle),
	Mask(ValueSingle),
	Single(ValuePenumbra),
	Multi(ValuePenumbra),
}

impl ConfSetting {
	pub fn draw(&mut self, option: &MetaOption) -> bool {
		use imgui::aeth;
		
		match self {
			Self::Rgb(v) => {
				if let MetaOption::Rgb(o) = option {
					imgui::color_edit3(&o.name, v, imgui::ColorEditFlags::PickerHueWheel | imgui::ColorEditFlags::NoInputs)
				} else {false}
			},
			Self::Rgba(v) => {
				if let MetaOption::Rgba(o) = option {
					imgui::color_edit4(&o.name, v, imgui::ColorEditFlags::PickerHueWheel | imgui::ColorEditFlags::NoInputs | imgui::ColorEditFlags::AlphaBar | imgui::ColorEditFlags::AlphaPreviewHalf)
				} else {false}
			},
			Self::Grayscale(v) => {
				if let MetaOption::Grayscale(o) = option {
					let mut v2 = *v * 255.0;
					let r = imgui::drag_float(&o.name, &mut v2, 0.0, 0.0, 255.0, "%f", imgui::SliderFlags::NoRoundToFormat);
					*v = v2 / 255.0;
					r
				} else {false}
			},
			Self::Opacity(v) => {
				if let MetaOption::Opacity(o) = option {
					let mut v2 = *v * 255.0;
					let r = imgui::drag_float(&o.name, &mut v2, 0.0, 0.0, 255.0, "%f", imgui::SliderFlags::NoRoundToFormat);
					*v = v2 / 255.0;
					r
				} else {false}
			},
			Self::Mask(v) => {
				if let MetaOption::Mask(o) = option {
					let mut v2 = *v * 100.0;
					let r = imgui::drag_float(&o.name, &mut v2, 0.0, 0.0, 100.0, "%.1f%%", imgui::SliderFlags::NoRoundToFormat);
					*v = v2 / 100.0;
					r
				} else {false}
			},
			Self::Single(v) => {
				if let MetaOption::Single(o) = option {
					aeth::combo(&o.name, &o.options[*v as usize].name, imgui::ComboFlags::None, || {
						for (i, s) in o.options.iter().enumerate() {
							if imgui::selectable(&s.name, i == *v as usize, imgui::SelectableFlags::None, [0.0; 2]) {
								*v = i as i32;
								return true;
							}
						}
						false
					}).unwrap_or(false)
				} else {false}
			},
			Self::Multi(v) => {
				if let MetaOption::Single(o) = option {
					let mut c = false;
					aeth::frame_sized(|| {
						imgui::text(&o.name);
						for (i, s) in o.options.iter().enumerate() {
							let offset = 1 << (i as i32);
							let mut state = (*v & offset) == offset;
							if imgui::checkbox(&s.name, &mut state) {
								*v ^= (if state {-1} else {0} ^ *v) & offset;
								c = true;
							}
						}
					});
					
					c
				} else {false}
			}
		}
	}
	
	pub fn draw_label(&mut self, label: &str) -> bool {
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
			},
			_ => {
				imgui::text(label);
				false
			},
		}
	}
}

#[derive(Clone, Debug)]
pub struct Layer {
	pub value: Option<ConfSetting>,
	pub files: Vec<String>,
}

pub fn get_load_file(root: Option<std::path::PathBuf>) -> impl Fn(&str) -> Option<Vec<u8>> {
	move |path| -> Option<Vec<u8>> {
		// TODO: allow reading from mods with lower priority
		if let Some(root) = &root && let Ok(mut f) = File::open(root.join(path)) {
			let mut buf = Vec::with_capacity(f.stream_len().unwrap() as usize);
			f.read_to_end(&mut buf).unwrap();
			Some(buf)
		} else {
			GAME.file::<Vec<u8>>(path).ok()
		}
	}
}

// pub fn load_file(path: &str) -> Option<Vec<u8>> {
// 	// TODO: allow reading from mods with lower priority
// 	if let Ok(mut f) = File::open(path) {
// 		let mut buf = Vec::with_capacity(f.stream_len().unwrap() as usize);
// 		f.read_to_end(&mut buf).unwrap();
// 		Some(buf)
// 	} else {
// 		GAME.file::<Vec<u8>>(path).ok()
// 	}
// }

// Might not want to return tex, idk yet
pub fn resolve_layer(layer: &Layer, mut load_file: impl FnMut(&str) -> Option<Vec<u8>>) -> Result<Tex, String> {
	if let Some(v) = &layer.value {
		match v {
			ConfSetting::Rgb(val) => {
				let mut tex = Tex::read(&mut Cursor::new(&load_file(&layer.files[0]).ok_or(layer.files[0].clone())?));
				tex.as_pixels_mut().iter_mut().for_each(|pixel| {
					pixel.b = (pixel.b as f32 * val[2]) as u8;
					pixel.g = (pixel.g as f32 * val[1]) as u8;
					pixel.r = (pixel.r as f32 * val[0]) as u8;
				});
				Ok(tex)
			},
			ConfSetting::Rgba(val) => {
				let mut tex = Tex::read(&mut Cursor::new(&load_file(&layer.files[0]).ok_or(layer.files[0].clone())?));
				tex.as_pixels_mut().iter_mut().for_each(|pixel| {
					pixel.b = (pixel.b as f32 * val[2]) as u8;
					pixel.g = (pixel.g as f32 * val[1]) as u8;
					pixel.r = (pixel.r as f32 * val[0]) as u8;
					pixel.a = (pixel.r as f32 * val[3]) as u8;
				});
				Ok(tex)
			},
			ConfSetting::Grayscale(val) => {
				let mut tex = Tex::read(&mut Cursor::new(&load_file(&layer.files[0]).ok_or(layer.files[0].clone())?));
				tex.as_pixels_mut().iter_mut().for_each(|pixel| {
					pixel.b = (pixel.b as f32 * val) as u8;
					pixel.g = (pixel.g as f32 * val) as u8;
					pixel.r = (pixel.r as f32 * val) as u8;
				});
				Ok(tex)
			},
			ConfSetting::Opacity(val) => {
				let mut tex = Tex::read(&mut Cursor::new(&load_file(&layer.files[0]).ok_or(layer.files[0].clone())?));
				tex.as_pixels_mut().iter_mut().for_each(|pixel| {
					pixel.a = (pixel.a as f32 * val) as u8;
				});
				Ok(tex)
			},
			ConfSetting::Mask(val) => {
				let val = (val * 255f32) as u8;
				let mut tex = Tex::read(&mut Cursor::new(&load_file(&layer.files[0]).ok_or(layer.files[0].clone())?));
				let mask = Tex::read(&mut Cursor::new(&load_file(&layer.files[1]).ok_or(layer.files[1].clone())?));
				let mask_pixels = mask.as_pixels();
				tex.as_pixels_mut().iter_mut().enumerate().for_each(|(i, pixel)| {
					pixel.a = if val >= mask_pixels[i].r {pixel.a} else {0};
				});
				Ok(tex)
			},
			_ => Err("Invalid layer value to resolve, how did it come to this situation?".to_owned()), // Others dont affect layers and should never come to this point anyways
		}
	} else {
		Ok(Tex::read(&mut Cursor::new(&load_file(&layer.files[0]).ok_or(layer.files[0].clone())?)))
	}
}