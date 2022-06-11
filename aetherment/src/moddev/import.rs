use std::{fs::{self, File}, path::Path, io::{Write, Read, Seek, SeekFrom}, collections::HashMap};
use serde_json::json;
use crate::serialize_json;

ffi!(fn import_penumbra(penumbra_path: &str, target_path: &str) {
	log!(log, "{}", penumbra_path);
	log!(log, "{}", target_path);
	
	let penumbra_path = Path::new(penumbra_path);
	let target_path = Path::new(target_path);
	
	fs::create_dir_all(target_path).unwrap();
	
	let meta: serde_json::Value = serde_json::from_reader(File::open(penumbra_path.join("meta.json")).unwrap()).unwrap();
	let mut new_meta = File::create(target_path.join("meta.json")).unwrap();
	new_meta.write_all(serialize_json(json!({
		"name": meta["Name"],
		"description": meta["Description"],
		"contributors": ([0i32; 0]),
		"dependencies": ([0i32; 0]),
		"main_mod": None::<i32>,
		"nsfw": false,
	})).as_bytes()).unwrap();
	
	fs::create_dir_all(target_path.join("files")).unwrap();
	
	let mut groups = Vec::<(i64, serde_json::Value)>::new();
	
	'options: for p in fs::read_dir(penumbra_path).unwrap() {
		let p = p.unwrap();
		let n = p.file_name();
		let filename = n.to_str().unwrap();
		if p.file_type().unwrap().is_file() && filename.len() > 6 && &filename[..6] == "group_" {
			let group: serde_json::Value = serde_json::from_reader(File::open(p.path().as_path()).unwrap()).unwrap();
			let priority = group["Priority"].as_i64().unwrap();
			let mut options = Vec::<serde_json::Value>::new();
			for option in group["Options"].as_array().unwrap() {
				options.push(import_block(option, &penumbra_path, &target_path));
			}
			log!(log, "{}", filename);
			let block = json!({
				"type": group["Type"].as_str().unwrap().to_lowercase(),
				"name": group["Name"],
				"description": group["Description"],
				"options": options,
			});
			
			for (i, g) in groups.iter().enumerate() {
				if priority < g.0 {
					groups.insert(i, (priority, block));
					continue 'options;
				}
			}
			
			groups.push((priority, block));
		}
	}
	
	let no_group_block = import_block(
		&serde_json::from_reader(File::open(penumbra_path.join("default_mod.json")).unwrap()).unwrap(),
		&penumbra_path,
		&target_path
	);
	
	// TODO: probably dont make it reread all the files again, simply compress inside import block
	crate::moddev::compress::compress(target_path);
	
	let mut new_datas = File::create(target_path.join("datas.json")).unwrap();
	new_datas.write_all(serialize_json(json!({
		"penumbra": {
			"options": groups.into_iter().map(|f| f.1).collect::<serde_json::Value>(),
			"files": no_group_block["files"],
			"swaps": no_group_block["swaps"],
			"manipulations": no_group_block["manipulations"],
		}
	})).as_bytes()).unwrap();
});

fn import_block(block: &serde_json::Value, penumbra_path: &Path, target_path: &Path) -> serde_json::Value {
	let mut files = HashMap::<&str, String>::new(); // value can be a array of arrays aswell, but not used when importing
	let mut hasher = blake3::Hasher::new();
	let mut buf = [0u8; 4096];
	
	for (gamepath, realpath) in block["Files"].as_object().unwrap() {
		let mut f = File::open(penumbra_path.join(realpath.as_str().unwrap())).unwrap();
		while f.read(&mut buf).unwrap() != 0 {
			hasher.update(&buf);
		}
		
		// Im sure 96/256 bits is enough, right? right??
		let hash = &hasher.finalize().to_hex().as_str()[..24].to_string();
		let newrelpath = format!("files/{}", hash);
		let newpath = target_path.join(&newrelpath);
		
		// We read the file for a 2nd time, idk if this is the best approach
		if !newpath.exists() {
			log!(log, "{} Importing {}", target_path.file_name().unwrap().to_str().unwrap(), newpath.file_name().unwrap().to_str().unwrap());
			let mut f2 = File::create(newpath).unwrap();
			f.seek(SeekFrom::Start(0)).unwrap();
			while f.read(&mut buf).unwrap() != 0 {
				f2.write_all(&buf).unwrap();
			}
			f2.flush().unwrap();
		}
		
		files.insert(
			gamepath,
			newrelpath
		);
		
		hasher.reset();
	}
	
	json!({
		"name": block["Name"],
		"files": files,
		"swaps": block["FileSwaps"],
		"manipulations": block["Manipulations"],
	})
}