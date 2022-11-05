use std::{fs::{self, File}, collections::HashMap, io::Write};
use crate::{CLIENT, SERVER, SERVERCDN};

pub struct DownloadInfo<'a> {
	pub id: i32,
	pub name: &'a str,
	pub description: &'a str,
	pub author: &'a str,
	pub version: i32,
	pub tags: Vec<&'a str>,
}

// TODO: queue system
pub fn download_mod(info: DownloadInfo, token: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
	let mut req = CLIENT.get(format!("{SERVER}/api/mod/{}/download/{}", info.id, info.version));
	if let Some(token) = token {
		req = req.header("Authorization", token);
	}
	
	let downloads = req.send()?.json::<Vec<String>>()?;
	let mod_dir = crate::api::penumbra::root_path().join(info.id.to_string());
	let files_dir = mod_dir.join("files");
	fs::create_dir_all(&files_dir)?;
	let downloads_dir = mod_dir.join("downloads");
	fs::create_dir_all(&downloads_dir)?;
	
	let mut map = HashMap::new();
	let mut files = HashMap::new();
	for (i, path) in downloads.into_iter().enumerate() {
		let file_path = downloads_dir.join(crate::hash_str(blake3::hash(path.as_bytes()).as_bytes()));
		if !file_path.exists() {
			let mut file = File::create(&file_path)?;
			CLIENT.get(format!("{SERVERCDN}{path}")).send()?.copy_to(&mut file)?;
		}
		
		let pack = crate::creator::modpack::ModPack::load(File::open(&file_path)?)?;
		let index = pack.index();
		if i == 0 {
			for (path, hash) in &index.paths {
				map.insert(path.clone(), crate::hash_str(hash));
			}
			
			for (hash, _offset) in &index.locations {
				files.insert(hash.clone(), false);
			}
		}
		
		for (hash, offset) in &index.locations {
			if *offset > 0 && let Some(done) = files.get_mut(hash) && !*done {
				*done = true;
				let data = pack.read_file_offset(*offset)?;
				File::create(files_dir.join(crate::hash_str(hash)))?.write_all(&data)?;
			}
		}
		
		fs::remove_file(file_path)?;
	}
	
	File::create(mod_dir.join("map.json"))?.write_all(&serde_json::to_vec(&map)?)?;
	
	File::create(mod_dir.join("meta.json"))?.write_all(&serde_json::to_vec(&serde_json::json!({
		"FileVersion": 3,
		"Name": info.name,
		"Author": info.author,
		"Description": info.description,
		"Version": crate::creator::modpack::version_to_string(info.version),
		"Website": format!("{SERVER}/mod/{}", info.id),
		"ModTags": ({
			let mut t = info.tags;
			t.insert(0, "Aetherment");
			t
		}),
	}))?)?;
	
	crate::api::penumbra::add_mod_entry(format!("{}", info.id));
	
	Ok(())
}