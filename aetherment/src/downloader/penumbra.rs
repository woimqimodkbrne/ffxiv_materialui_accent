use std::{collections::{HashMap, HashSet}, path::{Path, PathBuf}, fs::{File, self}, io::{Write, Seek, Read, Cursor}, time::SystemTime};
use noumenon::formats::game::tex::Tex;
use path_slash::PathExt;
use serde::Deserialize;
use serde_json::json;
use crate::{serialize_json, SERVER, IRONWORKS};
use super::download::{Meta, Settings};

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
	Single(TypPenumbra),
	Multi(TypPenumbra),
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConfSettings {
	Rgb([f32; 3]),
	Rgba([f32; 4]),
	Grayscale(f32),
	Opacity(f32),
	Mask(f32),
}

#[derive(Deserialize)]
pub struct TypRgb {
	pub id: String,
	pub name: String,
	pub description: String,
	pub default: [f32; 3],
}

#[derive(Deserialize)]
pub struct TypRgba {
	pub id: String,
	pub name: String,
	pub description: String,
	pub default: [f32; 4],
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

fn get_mod_settings<'a>(mod_id: &str, config: &'a Config, settings: &Settings) -> HashMap<String, ConfSettings> {
	let settings_path = Path::new(settings.config_dir).join("penumbra").join(format!("{}.json", mod_id));
	
	let mut conf_settings: HashMap<String, ConfSettings> = if settings_path.exists() {
		serde_json::from_reader(File::open(&settings_path).unwrap()).unwrap()
	} else {HashMap::new()};
	
	config.options.iter()
		.filter(|o| !matches!(o, ConfOption::Single(_)) && !matches!(o, ConfOption::Single(_)))
		.for_each(|o| match o {
			ConfOption::Rgb(v) => {conf_settings.entry(v.id.to_string()).or_insert(ConfSettings::Rgb(v.default));},
			ConfOption::Rgba(v) => {conf_settings.entry(v.id.to_string()).or_insert(ConfSettings::Rgba(v.default));},
			ConfOption::Grayscale(v) => {conf_settings.entry(v.id.to_string()).or_insert(ConfSettings::Grayscale(v.default));},
			ConfOption::Opacity(v) => {conf_settings.entry(v.id.to_string()).or_insert(ConfSettings::Opacity(v.default));},
			ConfOption::Mask(v) => {conf_settings.entry(v.id.to_string()).or_insert(ConfSettings::Mask(v.default));},
			_ => {},
		});
	
	conf_settings
}

pub fn download<'a>(settings: &'a Settings, meta: &'a Meta, config: &'a Config, file_hashes: &HashMap<&str, &str>, downloader: impl Fn(&str, &Path) -> Option<PathBuf>) {
	let mod_path = Path::new(settings.penumbra_dir).join(&meta.id);
	let files_path = mod_path.join("files");
	let conf_settings = get_mod_settings(&meta.id, config, settings);
	let mut changed_files: HashSet<&str> = HashSet::new();
	let mut download_files = |f: &'a HashMap<String, PenumbraFile>| -> HashMap::<&str, String> {
		let mut files = HashMap::<&str, String>::new();
		
		f.into_iter().for_each(|(gamepath, p)| match p {
			PenumbraFile::Simple(path) => {
				if let Some(&hash) = file_hashes.get(&path.as_ref()) {
					files.insert(gamepath.as_ref(), files_path.join(hash).strip_prefix(&mod_path).unwrap().to_slash_lossy());
				}
				
				if let Some(_file_path) = downloader(path, &files_path) {
					changed_files.insert(path);
					// changed_files.insert(file_path.strip_prefix(&mod_path).unwrap().to_slash_lossy());
				}
			},
			PenumbraFile::Complex(paths) => {
				paths.iter()
					.for_each(|layer| layer
						.iter()
						.enumerate()
						.filter_map(|(i, e)| (i > 0).then(|| e.as_ref().unwrap()))
						.for_each(|path| if let Some(_file_path) = downloader(&path, &files_path) {
							changed_files.insert(path);
							// changed_files.insert(file_path.strip_prefix(&mod_path).unwrap().to_slash_lossy());
						}));
				
				files.insert(gamepath.as_ref(), resolve_customizability(&mod_path, file_hashes, &conf_settings, gamepath, paths)
				                                	.strip_prefix(&mod_path).unwrap().to_slash_lossy());
			},
		});
		
		files
	};
	
	fs::create_dir_all(&mod_path).unwrap();
	File::create(mod_path.join("default_mod.json"))
		.unwrap()
		.write_all(serialize_json(json!({
			"Name": "Default",
			"Priority": 0,
			"Files": download_files(&config.files),
			"FileSwaps": config.swaps,
			"Manipulations": config.manipulations,
		})).as_bytes())
		.unwrap();
	
	config.options.iter()
		.enumerate()
		.filter_map(|(i, o)| match o {
			ConfOption::Multi(opt) => Some((i, true, opt)),
			ConfOption::Single(opt) => Some((i, false, opt)),
			_ => None,
		})
		.for_each(|(i, is_multi, opt)| {
			let mut options = Vec::<serde_json::Value>::new();
			
			opt.options.iter()
				.for_each(|option| options.push(json!({
					"Name": option.name,
					"Files": download_files(&option.files),
					"FileSwaps": option.swaps,
					"Manipulations": option.manipulations,
				})));
			
			File::create(mod_path.join(format!("group_{:03}.json", i)))
				.unwrap()
				.write_all(serialize_json(json!({
					"Name": opt.name,
					"Description": opt.description,
					"Priority": i + 1,
					"Type": if is_multi {"multi"} else {"single"},
					"Options": options,
				})).as_bytes())
				.unwrap();
		});
	
	File::create(mod_path.join("meta.json"))
		.unwrap()
		.write_all(serialize_json(json!({
			"FileVersion": 1,
			"Name": meta.name,
			"Description": meta.description,
			"Author": ({
				let mut authors = vec![meta.author.name.as_ref()];
				authors.extend(meta.contributors.iter().map(|v| -> &str {v.name.as_ref()}));
				authors.join(", ")
			}),
			"Version": "TODO", // TODO: add version histroy to server mod api return
			"Website": format!("{}/mod/{}", SERVER, meta.id),
			"ImportDate": SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as i64
		})).as_bytes())
		.unwrap();
}

fn resolve_customizability<'a>(mod_path: &Path, file_hashes: &HashMap<&str, &str>, settings: &HashMap<String, ConfSettings>, gamepath: &str, path: &'a Vec<Vec<Option<String>>>) -> PathBuf {
	let files_path = mod_path.join("files");
	
	let load_file = |path: &str| -> Vec<u8> {
		if let Some(hash) = file_hashes.get(path) {
			let mut f = File::open(files_path.join(format!("{}.{}", hash, path.split(".").last().unwrap())))
				.unwrap();
			
			let mut buf = Vec::with_capacity(f.stream_len().unwrap() as usize);
			f.read_to_end(&mut buf).unwrap();
			
			buf
		} else {
			// TODO: allow reading from mods with lower priority
			// TODO: handle cases where the file doesnt exist, probably
			IRONWORKS.file::<Vec<u8>>(path).unwrap()
		}
	};
	
	let resolve_layer = |layer: &'a Vec<Option<String>>| -> Vec<u8> {
		// let layer = layers.next().unwrap();
		let mut data = load_file(layer[1].as_ref().unwrap().as_ref());
		log!(log, "data len {}", data.len());
		if let Some(id) = &layer[0] {
			match settings[id] {
				ConfSettings::Rgb(val) => {
					let mut tex = Tex::read(&mut Cursor::new(&data));
					tex.as_pixels_mut().iter_mut().for_each(|pixel| {
						pixel.b = (pixel.b as f32 * val[2]) as u8;
						pixel.g = (pixel.g as f32 * val[1]) as u8;
						pixel.r = (pixel.r as f32 * val[0]) as u8;
					});
					tex.write(&mut Cursor::new(&mut data));
				},
				ConfSettings::Rgba(val) => {
					let mut tex = Tex::read(&mut Cursor::new(&data));
					tex.as_pixels_mut().iter_mut().for_each(|pixel| {
						pixel.b = (pixel.b as f32 * val[2]) as u8;
						pixel.g = (pixel.g as f32 * val[1]) as u8;
						pixel.r = (pixel.r as f32 * val[0]) as u8;
						pixel.a = (pixel.r as f32 * val[3]) as u8;
					});
					tex.write(&mut Cursor::new(&mut data));
				},
				ConfSettings::Grayscale(val) => {
					let mut tex = Tex::read(&mut Cursor::new(&data));
					tex.as_pixels_mut().iter_mut().for_each(|pixel| {
						pixel.b = (pixel.b as f32 * val) as u8;
						pixel.g = (pixel.g as f32 * val) as u8;
						pixel.r = (pixel.r as f32 * val) as u8;
					});
					tex.write(&mut Cursor::new(&mut data));
				},
				ConfSettings::Opacity(val) => {
					let mut tex = Tex::read(&mut Cursor::new(&data));
					tex.as_pixels_mut().iter_mut().for_each(|pixel| {
						pixel.a = (pixel.a as f32 * val) as u8;
					});
					tex.write(&mut Cursor::new(&mut data));
				},
				ConfSettings::Mask(val) => {
					let val = (val * 255f32) as u8;
					let mask = Tex::read(&mut Cursor::new(&load_file(&layer[2].as_ref().unwrap())));
					let mask_pixels = mask.as_pixels();
					let mut tex = Tex::read(&mut Cursor::new(&data));
					tex.as_pixels_mut().iter_mut().enumerate().for_each(|(i, pixel)| {
						pixel.a = if val >= mask_pixels[i].r {pixel.a} else {0};
					});
					tex.write(&mut Cursor::new(&mut data));
				},
			}
		}
		
		data
	};
	
	let mut layers = path.iter();
	let mut result = resolve_layer(layers.next().as_ref().unwrap());
	while let Some(layer) = layers.next() {
		// TODO: This assumes additional layers are always texture based, mby not a good idea for the future?
		// constantly reconverting the result and the layer for a 2nd time is dumb, TODO: fix that
		let layer = Tex::read(&mut Cursor::new(&resolve_layer(layer)));
		let overlay = layer.as_pixels();
		let mut res = Tex::read(&mut Cursor::new(&result));
		res.as_pixels_mut().iter_mut().enumerate().for_each(|(i, pixel)| {
			let ar = pixel.a as f32 / 255.0;
			let ao = overlay[i].a as f32 / 255.0;
			let a = ao + ar * (1.0 - ao);
			
			pixel.b = ((overlay[i].b as f32 * ao + pixel.b as f32 * ar * (1.0 - ao)) / a) as u8;
			pixel.g = ((overlay[i].g as f32 * ao + pixel.g as f32 * ar * (1.0 - ao)) / a) as u8;
			pixel.r = ((overlay[i].r as f32 * ao + pixel.r as f32 * ar * (1.0 - ao)) / a) as u8;
			pixel.a = (a * 255.0) as u8;
		});
		res.write(&mut Cursor::new(&mut result));
	}
	
	fs::create_dir_all(&mod_path.join("files2")).unwrap();
	let path = mod_path.join("files2").join(format!("{}.{}", blake3::hash(&result).to_hex().as_str()[..24].to_string(), gamepath.split(".").last().unwrap()));
	
	File::create(&path)
		.unwrap()
		.write_all(&result)
		.unwrap();
	
	path
}