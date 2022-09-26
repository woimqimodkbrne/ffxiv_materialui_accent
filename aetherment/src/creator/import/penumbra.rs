use std::{path::Path, fs::{self, File}, io::{Write, Read}, collections::HashMap};
use serde::Deserialize;
use serde_json::json;
use crate::apply::penumbra::Manipulation;
use super::super::modpack::Error; // TODO: dont use modpack result, idk

pub fn import<P, P2>(penumbra_dir: P, target_dir: P2) -> Result<(), Error> where
P: AsRef<Path>,
P2: AsRef<Path> {
	let penumbra_dir = penumbra_dir.as_ref();
	let target_dir = target_dir.as_ref();
	let files_dir = target_dir.join("files");
	if !files_dir.exists() {fs::create_dir_all(&files_dir)?}
	
	let penumbra_meta: serde_json::Value = serde_json::from_reader(File::open(penumbra_dir.join("meta.json"))?)?;
	File::create(target_dir.join("meta.json"))?.write_all(
		crate::serialize_json(json!({
			"name": penumbra_meta["Name"].as_str().unwrap(),
			"description": penumbra_meta["Description"].as_str().unwrap(),
			"nsfw": false,
			"previews": [],
			"contributors": [],
			"dependencies": [],
		})).as_bytes()
	)?;
	
	let mut hashed_files = HashMap::<String, String>::new();
	
	#[derive(Deserialize)]
	#[serde(rename_all = "PascalCase")]
	struct PenumbraGroup {
		name: String,
		description: String,
		r#type: String,
		options: Vec<PenumbraBlock>,
	}
	
	#[derive(Deserialize)]
	#[serde(rename_all = "PascalCase")]
	struct PenumbraBlock {
		name: String,
		files: HashMap<String, String>,
		file_swaps: HashMap<String, String>,
		manipulations: Vec<Manipulation>,
	}
	
	// TODO: multithread
	let mut handle_block = |block: PenumbraBlock| -> Result<serde_json::Value, std::io::Error> {
		let mut files = HashMap::new();
		for (game, real) in block.files {
			if let Some(hash) = hashed_files.get(&real) {
				files.insert(game, hash.clone());
			} else {
				let mut buf = [0u8; 4096];
				let mut hasher = blake3::Hasher::new();
				let mut file = File::open(penumbra_dir.join(&real))?;
				while let readcount = file.read(&mut buf)? && readcount != 0 {
					hasher.update(&buf[0..readcount]);
				}
				let hash = crate::hash_str(hasher.finalize().as_bytes());
				let path = format!("files/{hash}");
				files.insert(game, hash.clone());
				hashed_files.insert(real.to_owned(), path);
				fs::copy(penumbra_dir.join(&real), files_dir.join(hash))?;
			}
		}
		
		Ok(json!({
			"name": block.name,
			"files": files,
			"swaps": block.file_swaps,
			"manipulations": block.manipulations,
		}))
	};
	
	log!("default_mod.json");
	let default: serde_json::Value = handle_block(serde_json::from_reader(File::open(penumbra_dir.join("default_mod.json"))?)?)?;
	File::create(target_dir.join("datas.json"))?.write_all(
		crate::serialize_json(json!({
			"penumbra": {
				"files": default["files"],
				"swaps": default["swaps"],
				"manipulations": default["manipulations"],
				"options": ({
					let mut options = Vec::new();
					
					// TODO: care about priority, or dont, idk
					for entry in fs::read_dir(&penumbra_dir)?.filter_map(|v| {let v = v.ok()?; if v.file_name().to_str()?.starts_with("group_") {Some(v)} else {None}}) {
						log!("{}", entry.path().file_name().unwrap().to_str().unwrap());
						let group: PenumbraGroup = serde_json::from_reader(File::open(entry.path())?)?;
						options.push(json!({
							"name": group.name,
							"description": group.description,
							"type": group.r#type.to_lowercase(),
							"options": ({
								let mut options = Vec::new();
								for block in group.options {
									options.push(handle_block(block)?);
								}
								options
							})
						}));
					}
					
					options
				}),
			}
		})).as_bytes()
	)?;
	
	Ok(())
}