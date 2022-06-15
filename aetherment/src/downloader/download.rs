use std::{collections::HashMap, path::{PathBuf, Path}, fs::{File, self}, io::Write};
use serde::Deserialize;
use crate::{CLIENT, SERVER};
use super::penumbra;

#[derive(Deserialize)]
pub struct Meta {
	pub id: String,
	pub name: String,
	pub description: String,
	pub author: NameId,
	pub contributors: Vec<NameId>,
	pub dependencies: Vec<NameId>,
}

#[derive(Deserialize)]
pub struct NameId {
	pub name: String,
	pub id: String,
}

#[derive(Deserialize)]
pub struct Config {
	penumbra: penumbra::Config,
}

#[derive(Deserialize)]
struct FileEntry {
	path: String,
	hash: String,
	// size_compressed: i64,
	// size_uncompressed: i64,
}

pub struct Settings<'a> {
	pub config_dir: &'a str,
	pub penumbra_dir: &'a str,
}

ffi!(fn download_mod(id: &str, settings: &Settings) {
	// TODO: popup to select which parts to install
	// TODO: penumbra options in collections support (talk to otter about how to handle that stuff)
	// TODO: autoupdates
	// TODO: dependency popup
	
	// TODO: dalamud style support
	
	let meta: Meta = serde_json::from_str(&CLIENT.get(format!("{}/mod/{}.json", SERVER, id))
		.send()
		.unwrap()
		.text()
		.unwrap()).unwrap();
	
	let config: Config = serde_json::from_str(&CLIENT.get(format!("{}/mod/{}/datas.json", SERVER, id))
		.send()
		.unwrap()
		.text()
		.unwrap()).unwrap();
	
	let files: Vec<FileEntry> = serde_json::from_str(&CLIENT.get(format!("{}/mod/{}/files.json", SERVER, id))
		.send()
		.unwrap()
		.text()
		.unwrap()).unwrap();
	
	let file_hashes: HashMap<&str, &str> = files.iter().map(|f| (f.path.as_ref(), f.hash.as_ref())).collect();
	
	let downloader = |file: &'_ str, target_dir: &'_ Path| -> Option<PathBuf> {
		// TODO: gui popup or window or smth to show progress
		if !file_hashes.contains_key(file) {
			return None;
		}
		
		let target_path = target_dir.join(file_hashes[file]);
		if target_path.exists() {
			return None;
		}
		
		fs::create_dir_all(target_dir).unwrap();
		// TODO: stream this instead
		let mut f = flate2::write::ZlibDecoder::new(File::create(&target_path).unwrap());
		f.write_all(&CLIENT.get(format!("{}/mod/{}/{}", SERVER, id, file_hashes[file]))
		.send()
		.unwrap()
		.bytes()
		.unwrap()).unwrap();
		f.finish().unwrap();
		
		Some(target_path)
	};
	
	penumbra::download(settings, &meta, &config.penumbra, &file_hashes, downloader);
});