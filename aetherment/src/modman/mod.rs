use std::{collections::{HashSet, HashMap}, io::{Read, Write}};
use serde::{Deserialize, Serialize};
use crate::render_helper::EnumTools;

use self::composite::Composite;

pub mod backend;
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

pub fn get_mod_files(meta: &meta::Meta, files_path: &std::path::Path) -> HashMap<String, Vec<String>> {
	let mut files = HashMap::new();
	let mut insert = |path: Option<&str>, real_path: &str| {
		let entry = files.entry(real_path.to_owned()).or_insert_with(|| Vec::new());
		if let Some(path) = path {
			entry.push(path.to_owned());
		}
	};
	
	let mut add_file = |path: &str, real_path: &str| {
		// files.insert(real_path.to_owned());
		insert(Some(path), real_path);
		
		if path.ends_with(".comp") {
			match path.trim_end_matches(".comp").split(".").last().unwrap() {
				"tex" | "atex" => {
					let Ok(mut f) = std::fs::File::open(files_path.join(real_path)) else {return};
					let mut buf = Vec::new();
					f.read_to_end(&mut buf).unwrap();
					let comp: composite::tex::Tex = match serde_json::from_slice(&buf) {
						Ok(v) => v,
						Err(e) => {
							log!(err, "Failed to parse tex comp file: {e}\ndata: {}", String::from_utf8_lossy(&buf));
							return;
						}
					};
					
					for file in comp.get_files() {
						// files.insert(file.to_owned());
						insert(None, file);
					}
				}
				
				_ => {return}
			}
		}
	};
	
	for (path, real_path) in &meta.files {
		add_file(path, real_path);
	}
	
	for option in &meta.options {
		if let meta::OptionSettings::SingleFiles(v) | meta::OptionSettings::MultiFiles(v) = &option.settings {
			for sub in &v.options {
				for (path, real_path) in &sub.files {
					add_file(path, real_path);
				}
			}
		}
	}
	
	files
}

pub fn game_files_hashes(files: HashSet<&str>) -> HashMap<String, [u8; blake3::OUT_LEN]> {
	let mut hashes = HashMap::new();
	let Some(noum) = crate::noumenon() else {return hashes};
	
	for file in files {
		if let Ok(f) = noum.file::<Vec<u8>>(file) {
			log!("hashing game file of {file}");
			hashes.insert(file.to_string(), blake3::hash(&f).as_bytes().to_owned());
		}
	}
	
	hashes
}

// pub fn cleanup(mod_path: &std::path::Path) -> Result<(), crate::resource_loader::BacktraceError> {
// 	let meta: meta::Meta = serde_json::from_reader(std::io::BufReader::new(std::fs::File::open(mod_path.join("meta.json"))?))?;
// 	let files = get_mod_files(&meta, &mod_path.join("files"));
// 	
// 	// TODO: cleanup here
// 	
// 	Ok(())
// }

pub struct ModCreationSettings {
	/// Used to be able to check changes the game has made to files this mod overrides, useful for ui
	pub current_game_files_hash: bool,
}

// TODO: use proper error
pub fn create_mod(mod_path: &std::path::Path, settings: ModCreationSettings) -> Result<std::path::PathBuf, crate::resource_loader::BacktraceError> {
	let meta_buf = {
		let mut buf = Vec::new();
		std::fs::File::open(mod_path.join("meta.json"))?.read_to_end(&mut buf)?;
		buf
	};
	let meta: meta::Meta = serde_json::from_slice(&meta_buf)?;
	let packs_path = mod_path.join("packs");
	_ = std::fs::create_dir(&packs_path);
	
	let files_path = mod_path.join("files");
	let files = get_mod_files(&meta, &files_path);
	
	log!("all files: {files:?}");
	
	// TODO: add name of the mod to the name, cba atm cuz of potential invalid names and characters
	let pack_path = packs_path.join(format!("{}.aeth", meta.version));
	if pack_path.exists() {return Err("Path with this version already exists".into())}
	let mut writer = zip::ZipWriter::new(std::io::BufWriter::new(std::fs::File::create(&pack_path)?));
	let options = zip::write::FileOptions::default()
		.compression_method(zip::CompressionMethod::Deflated)
		.compression_level(Some(9))
		.large_file(true); // oh no, the horror of losing 20b
	
	if settings.current_game_files_hash {
		let hashes = game_files_hashes(files.values().flat_map(|v| v.iter().map(|v| v.as_str())).collect());
		writer.start_file("hashes", options)?;
		writer.write_all(&serde_json::to_vec(&hashes)?)?;
	}
	
	writer.add_directory("files", options)?;
	writer.start_file("meta.json", options)?;
	writer.write_all(&meta_buf)?;
	
	let mut buf = Vec::new();
	let mut files_done = HashSet::new();
	let mut files_remap = HashMap::new();
	for (real_path, _) in &files {
		log!("packing file {real_path}");
		let mut f = std::fs::File::open(files_path.join(&real_path))?;
		f.read_to_end(&mut buf)?;
		let hash = blake3::hash(&buf);
		if files_done.contains(&hash) {continue}
		files_done.insert(hash);
		let hash_str = crate::hash_str(hash);
		files_remap.insert(real_path.to_string(), hash_str.clone());
		let name = format!("files/{}", hash_str);
		
		// TODO: perhabs use this instead of a seperate file? idk
		// if settings.current_game_files_hash {
		// 	writer.start_file_with_extra_data(name, options)?;
		// 	let game_file = crate::noumenon().unwrap().file::<Vec<u8>>()?;
		// 	writer.end_extra_data()?;
		// } else {
		// 	writer.start_file(name, options)?;
		// }
		writer.start_file(name, options)?;
		writer.write_all(&buf)?;
		buf.clear();
	}
	
	// honestly, we really should just mod the meta, but that also requires modding all .comp files
	writer.start_file("remap", options)?;
	writer.write_all(&serde_json::to_vec(&files_remap)?)?;
	
	writer.finish()?;
	
	Ok(pack_path)
}