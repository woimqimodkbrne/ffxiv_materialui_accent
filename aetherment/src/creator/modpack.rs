use std::{path::PathBuf, fs::{self, File}, collections::{HashMap, HashSet}, io::{Seek, Read, Cursor, Write, SeekFrom}};
use binrw::{BinWrite, BinReaderExt};
use crate::apply::penumbra::ConfOption;

// .amp: Aetherment Mod Pack
// .amp.patch: Aetherment Mod Pack Patch
// both are the same internal structure

pub fn version_to_i32(mut version: &str) -> i32 {
	log!("{}", version);
	if version.ends_with(".amp.patch") {
		version = version.split('-').last().unwrap();
	}
	log!("{}", version);
	
	let mut segs = version.split('.');
	(segs.next().unwrap().parse::<i32>().unwrap() << 24) +
	(segs.next().unwrap().parse::<i32>().unwrap() << 16) +
	(segs.next().unwrap().parse::<i32>().unwrap() << 8) +
	segs.next().unwrap().parse::<i32>().unwrap()
}

pub fn version_to_string(version: i32) -> String {
	format!("{}.{}.{}.{}",
		(version >> 24) & 0xFF,
		(version >> 16) & 0xFF,
		(version >> 8) & 0xFF,
		(version) & 0xFF
	)
}

pub fn pack_latest(mod_path: PathBuf, version: i32, patch_only: bool) -> (Option<PathBuf>, Option<PathBuf>) {
	let releases_path = mod_path.join("releases");
	if !releases_path.exists() {
		fs::create_dir(&releases_path).unwrap();
	}
	
	let mut latest = None;
	let mut latest_version = 0;
	fs::read_dir(&releases_path)
		.unwrap()
		.into_iter()
		.for_each(|e| {
			let e = e.unwrap();
			let name = e.file_name().to_str().unwrap().to_owned();
			if !name.ends_with(".amp") && !name.ends_with(".amp.patch") {return}
			
			let version = version_to_i32(&name);
			if version > latest_version {
				latest = Some(e.path());
				latest_version = version;
			}
		});
	
	pack(mod_path, version, patch_only, latest)
}

pub fn pack(mod_path: PathBuf, version: i32, mut patch_only: bool, latest: Option<PathBuf>) -> (Option<PathBuf>, Option<PathBuf>) {
	let releases_path = mod_path.join("releases");
	if !releases_path.exists() {
		fs::create_dir(&releases_path).unwrap();
	}
	
	// get latest release in order to create a patch file
	let latest_version = if let Some(latest) = &latest {
		version_to_i32(latest.file_name().unwrap().to_str().unwrap())
	} else {
		0
	};
	// let mut latest = None;
	// let mut latest_version = 0;
	// fs::read_dir(&releases_path)
	// 	.unwrap()
	// 	.into_iter()
	// 	.for_each(|e| {
	// 		let e = e.unwrap();
	// 		let name = e.file_name().to_str().unwrap().to_owned();
	// 		if !name.ends_with(".amp") && !name.ends_with(".amp.patch") {return}
			
	// 		let version = version_to_u32(name.split(".").last().unwrap());
	// 		if version > latest_version {
	// 			latest = Some(e.path());
	// 			latest_version = version;
	// 		}
	// 	});
	
	log!("{latest_version}");
	
	// -1 as offset means file exists in mod but not this pack (patch)
	let mut hash_location_latest = HashMap::<[u8; 32], i64>::new();
	if let Some(latest) = &latest {
		let mut latest = File::open(latest).unwrap();
		latest.seek(SeekFrom::End(-4)).unwrap();
		let index_len = latest.read_le::<u32>().unwrap() as i64;
		log!("{index_len}");
		latest.seek(SeekFrom::End(-index_len - 4)).unwrap();
		let mut read_len = 0i64;
		loop {
			let path_len = latest.read_le::<u8>().unwrap() as usize;
			let mut buf = vec![0u8; path_len];
			latest.read_exact(&mut buf).unwrap();
			// let path = String::from_utf8(buf).unwrap();
			let mut hash = [0u8; 32];
			latest.read_exact(&mut hash).unwrap();
			let offset = latest.read_le::<i64>().unwrap();
			hash_location_latest.insert(hash, offset);
			read_len += 1 + path_len as i64 + 32 + 8;
			log!("{read_len}");
			if read_len == index_len {break}
		}
	} else {
		patch_only = false;
	}
	
	let version = version_to_string(version);
	let release_path = releases_path.join(format!("{}.amp", version));
	let mut release = if !patch_only {Some(File::create(&release_path).unwrap())} else {None};
	let mut release_len = 0;
	let patch_path = releases_path.join(format!("{}-{}.amp.patch", version_to_string(latest_version), version));
	let mut patch = if latest.is_some() {Some(File::create(&patch_path).unwrap())} else {None};
	let mut patch_len = 0;
	
	let mut hash_location_release = HashMap::<[u8; 32], i64>::new();
	let mut hash_location_patch = HashMap::<[u8; 32], i64>::new();
	let mut file_hashes = HashMap::<String, [u8; 32]>::new();
	
	let datas: crate::apply::Datas = serde_json::from_reader(File::open(mod_path.join("datas.json")).unwrap()).unwrap();
	let mut todo = Vec::<String>::new();
	datas.penumbra.as_ref().unwrap().files.iter().for_each(|(_, layers)| layers.0.iter().for_each(|layer| layer.paths.iter().for_each(|p| todo.push(p.to_owned()))));
	datas.penumbra.as_ref().unwrap().options.iter().for_each(|o| match o {
			ConfOption::Single(opt) | ConfOption::Multi(opt) => opt.options.iter().for_each(|o| o.files.iter().for_each(|(_, layers)| layers.0.iter().for_each(|layer| layer.paths.iter().for_each(|p| todo.push(p.to_owned()))))),
			_ => {},
		}
	);
	
	// write all files to release pack and all needed files to patch file
	let mut buf = Vec::new();
	let mut compress_buf = Vec::new();
	
	let mut do_file = |subpath: String| {
		if let Ok(mut f) = File::open(mod_path.join(&subpath)) {
			log!("packing {}", subpath);
			f.read_to_end(&mut buf).unwrap();
			
			let hash = blake3::hash(&buf).as_bytes().to_owned();
			file_hashes.insert(subpath.to_owned(), hash.clone());
			
			if !hash_location_release.contains_key(&hash) {
				if release.is_some() || (patch.is_some() && !hash_location_latest.contains_key(&hash)) {
					let mut writer = flate2::write::ZlibEncoder::new(Cursor::new(&mut compress_buf), flate2::Compression::best());
					writer.write_all(&buf).unwrap();
					writer.finish().unwrap();
				}
				
				if let Some(release) = &mut release {
					hash_location_release.insert(hash.clone(), release_len);
					(compress_buf.len() as u32).write_to(release).unwrap();
					release.write_all(&compress_buf).unwrap();
					release_len += 4 + compress_buf.len() as i64;
				}
				
				if let Some(patch) = &mut patch && !hash_location_latest.contains_key(&hash) {
					hash_location_patch.insert(hash.clone(), patch_len);
					(compress_buf.len() as u32).write_to(patch).unwrap();
					patch.write_all(&compress_buf).unwrap();
					patch_len += 4 + compress_buf.len() as i64;
				}
			}
			
			buf.clear();
			compress_buf.clear();
		}
	};
	
	do_file("datas.json".to_owned());
	todo.into_iter().for_each(do_file);
	
	// write the index footer
	let mut footer_len = 0;
	for (path, hash) in file_hashes {
		footer_len += 1 + path.len() as u32 + 32 + 8;
		let len = path.len() as u8;
		
		if let Some(release) = &mut release {
			len.write_to(release).unwrap();
			path.as_bytes().write_to(release).unwrap();
			hash.write_to(release).unwrap();
			hash_location_release[&hash].write_to(release).unwrap();
		}
		
		
		if let Some(patch) = &mut patch {
			len.write_to(patch).unwrap();
			path.as_bytes().write_to(patch).unwrap();
			hash.write_to(patch).unwrap();
			(if let Some(o) = hash_location_patch.get(&hash) {*o} else {-1}).write_to(patch).unwrap();
		}
	}
	
	if let Some(release) = &mut release {footer_len.write_to(release).unwrap()}
	if let Some(patch) = &mut patch {footer_len.write_to(patch).unwrap()}
	
	(
		if release.is_some() {Some(release_path)} else {None},
		if patch.is_some() {Some(patch_path)} else {None}
	)
}

pub fn unpack(pack_path: PathBuf, taget_dir: PathBuf) {
	let files_dir = taget_dir.join("files");
	fs::create_dir_all(&files_dir).unwrap();
	
	let mut pack = File::open(pack_path).unwrap();
	
	let mut index = HashMap::<String, ([u8; 32], i64)>::new();
	let mut written_files = HashSet::<[u8; 32]>::new();
	let mut paths = HashMap::<String, String>::new();
	
	// read index
	pack.seek(SeekFrom::End(-4)).unwrap();
	let index_len = pack.read_le::<u32>().unwrap() as i64;
	pack.seek(SeekFrom::End(-index_len - 4)).unwrap();
	let mut read_len = 0i64;
	loop {
		let path_len = pack.read_le::<u8>().unwrap() as usize;
		let mut buf = vec![0u8; path_len];
		pack.read_exact(&mut buf).unwrap();
		let path = String::from_utf8(buf).unwrap();
		let mut hash = [0u8; 32];
		pack.read_exact(&mut hash).unwrap();
		let offset = pack.read_le::<i64>().unwrap();
		index.insert(path, (hash, offset));
		read_len += 1 + path_len as i64 + 32 + 8;
		if read_len == index_len {break}
	}
	
	// write all files
	let datas_path = taget_dir.join("datas.json");
	for (path, (hash, offset)) in index {
		if written_files.contains(&hash) {continue}
		written_files.insert(hash.clone());
		
		if offset == -1 {continue}
		
		let hash_str = base64::encode_config(&hash, base64::URL_SAFE_NO_PAD);
		paths.insert(path.clone(), format!("files/{hash_str}"));
		
		pack.seek(SeekFrom::Start(offset as u64)).unwrap();
		let mut f = flate2::write::ZlibDecoder::new(File::create(files_dir.join(&hash_str)).unwrap());
		let len = pack.read_le::<u32>().unwrap();
		let mut buf = vec![0u8; len as usize];
		pack.read_exact(&mut buf).unwrap();
		f.write_all(&buf).unwrap();
		f.finish().unwrap();
		
		if path == "datas.json" {
			fs::rename(files_dir.join(hash_str), &datas_path).unwrap();
		}
	}
	
	// remove unused files
	fs::read_dir(&files_dir)
		.unwrap()
		.into_iter()
		.for_each(|e| {
			let e = e.unwrap();
			let name = e.file_name().to_str().unwrap().to_owned();
			let hash = base64::decode_config(name, base64::URL_SAFE_NO_PAD).unwrap();
			if !written_files.contains(&hash[0..32]) {fs::remove_file(e.path()).unwrap()}
		});
	
	// fix paths
	let mut datas: crate::apply::Datas = serde_json::from_reader(File::open(&datas_path).unwrap()).unwrap();
	datas.penumbra.as_mut().unwrap().files.iter_mut().for_each(|(_, layers)|
		layers.0.iter_mut().for_each(|layer|
			layer.paths.iter_mut().for_each(|p| if let Some(p2) = paths.get(p) {*p = p2.clone()})
		)
	);
	datas.penumbra.as_mut().unwrap().options.iter_mut().for_each(|o| match o {
			ConfOption::Single(opt) | ConfOption::Multi(opt) => opt.options.iter_mut().for_each(|o|
				o.files.iter_mut().for_each(|(_, layers)|
					layers.0.iter_mut().for_each(|layer|
						layer.paths.iter_mut().for_each(|p| if let Some(p2) = paths.get(p) {*p = p2.clone()})
					)
				)
			),
			_ => {},
		}
	);
	
	File::create(&datas_path).unwrap().write_all(serde_json::json!(datas).to_string().as_bytes()).unwrap();
}