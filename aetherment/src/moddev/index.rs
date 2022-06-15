use std::{path::Path, collections::HashMap, fs::{self, File}, io::{Read, Write}};
use path_slash::PathExt;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde_json::json;

use crate::serialize_json;

ffi!(fn index_mod(mod_path: &str) {
	index(Path::new(mod_path));
});

// TODO: dont recalculate the hash every time, save last file write in the index or smth
pub fn index(mod_path: &Path) {
	let mod_name = mod_path.file_name().unwrap().to_str().unwrap();
	
	let mut index = HashMap::new();
	
	let mut hasher = blake3::Hasher::new();
	let mut buf = [0u8; 4096];
	
	for entry in walkdir::WalkDir::new(mod_path.join("files")).into_iter().filter(|e| e.as_ref().unwrap().path().is_file()) {
		let path = entry.unwrap().into_path();
		log!(log, "{} Hashing {}", mod_name, path.to_str().unwrap());
		let mut file = File::open(&path).unwrap();
		while file.read(&mut buf).unwrap() != 0 {
			hasher.update(&buf);
		}
		
		let hash = hasher.finalize().to_hex().as_str()[..24].to_string();
		index.entry(hash).or_insert_with(|| Vec::new()).push(path.strip_prefix(mod_path).unwrap().to_slash_lossy());
		
		hasher.reset();
	}
	
	// File::create(mod_path.join("index.json"))
	// 	.unwrap()
	// 	.write_all(serialize_json(json!(index
	// 		.iter()
	// 		.flat_map(|(k, v)|
	// 			v.iter()
	// 			 .map(|p| (p.to_str().unwrap(), k))
	// 			 .collect::<Vec<(&str, &String)>>()
	// 		)
	// 		.collect::<HashMap<_, _>>()
	// 	)).as_bytes()).unwrap();
	
	File::create(mod_path.join("index.json"))
		.unwrap()
		.write_all(serialize_json(json!(index)).as_bytes())
		.unwrap();
	
	let compressed_path = mod_path.join("files_compressed");
	fs::create_dir_all(&compressed_path).unwrap();
	
	// get rid of old files
	fs::read_dir(&compressed_path).unwrap().into_iter().for_each(|entry| {
		let entry = entry.unwrap();
		if !index.contains_key(entry.file_name().to_str().unwrap()) {
			fs::remove_file(entry.path()).unwrap();
		}
	});
	
	// brotli 11 quality, 22 lg_window_size
	// zlib default, no clue what that is
	// brotli single threaded: Material UI: 168  seconds, 178 MB > 22.4 MB
	// brotli multi  threaded: Material UI: 26   seconds, 178 MB > 22.4 MB
	// brotli single threaded: TBSE:        1955 seconds, 1.44 GB > 311 MB
	// brotli multi  threaded: TBSE:        270  seconds, 1.44 GB > 311 MB
	// brotli multi  threaded: Bibo+:       364  seconds, 1.33 GB > 233 MB
	// zlib   single threaded: Material UI: 5    seconds, 178 MB > 27.8 MB
	// zlib   multi  threaded: Material UI: 3    seconds, 178 MB > 27.8 MB
	// zlib   single threaded: TBSE:        73   seconds, 1.44 GB > 421 MB
	// zlib   multi  threaded: TBSE:        10   seconds, 1.44 GB > 421 MB
	// zlib   multi  threaded: Bibo+:       6    seconds, 1.33 GB > 325 MB
	// while brotli has better compression ratio and is supposed to be faster
	// decompression (haven't tested myself), it's really fucking slow
	// TODO: mby allow the author to select which method they'd like to use?
	// probably not worth it tho
	
	index.into_par_iter().for_each(|(hash, path)| {
		let new_path = compressed_path.join(hash);
		if new_path.exists() { // no need to redo
			return;
		}
		
		log!(log, "{} Compressing {}", mod_name, path[0]);
		let mut file = File::open(mod_path.join(&path[0])).unwrap();
		let file_new = File::create(new_path).unwrap();
		let mut buf = [0u8; 4096];
		
		let mut writer = flate2::write::ZlibEncoder::new(file_new, flate2::Compression::default());
		while file.read(&mut buf).unwrap() != 0 {
			writer.write_all(&buf).unwrap();
		}
		writer.finish().unwrap();
	});
}