use std::{path::Path, fs::{File, self}, collections::{HashMap, HashSet}, io::{Seek, Read, Cursor, Write, SeekFrom}, sync::{Mutex, Arc}};
use binrw::{BinWrite, BinReaderExt};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use crate::apply::penumbra::ConfOption;

// .amp: Aetherment Mod Pack
// .amp.patch: Aetherment Mod Pack Patch
// both are the same internal structure, only named differently for user convenience

/* modpack layout version 1
u8  layout version
u32 mod version
u32 required mod version (0 if not patch)
[ zlib compressed file blobs
	u32  blob size
	[u8] blob
]
[ index
	u8       path length
	[u8]     utf8 path
	[u8; 32] uncompressed blake3 hash
	i64      file blob offset, -1 for existing file without including it (patch)
]
u32 index size
*/

// TODO: proper error type
pub type Error = Box<dyn std::error::Error>;

// Having this as woth write+read is bad, as it wont work with BufWriter and BufReader
#[derive(Debug)]
pub struct ModPack<W: Write + Read + Seek + std::fmt::Debug> {
	inner: Mutex<W>,
	index: Mutex<Index>,
	prev_files: HashMap<[u8; 32], Vec<String>>,
}

#[derive(Debug, Default, Clone)]
pub struct Index {
	locations: HashMap<[u8; 32], i64>,
	paths: HashMap<String, [u8; 32]>,
}

impl<'a, W: Write + Read + Seek + std::fmt::Debug> ModPack<W> {
	pub fn new(mut writer: W, version: i32) -> Result<ModPack<W>, Error> {
		let w = &mut writer;
		1u8.write_to(w)?;
		version.write_to(w)?;
		0u32.write_to(w)?;
		
		Ok(ModPack {
			inner: Mutex::new(writer),
			index: Mutex::new(Index::default()),
			prev_files: HashMap::new(),
		})
	}
	
	pub fn new_patch<R>(mut writer: W, version: i32, mut prev: R) -> Result<ModPack<W>, Error> where
	R: Read + Seek {
		let _prev_modpack_version = prev.read_le::<u8>()?;
		let prev_version = prev.read_le::<u32>()?;
		let mut prev_files = HashMap::new();
		
		prev.seek(SeekFrom::End(-4))?;
		let index_len = prev.read_le::<u32>()? as i64;
		prev.seek(SeekFrom::End(-4 - index_len))?;
		let mut read_len = 0;
		loop {
			let path_len = prev.read_le::<u8>()? as i64;
			// prev.seek(SeekFrom::Current(path_len))?;
			let mut buf = vec![0u8; path_len as usize];
			prev.read_exact(&mut buf)?;
			let path = String::from_utf8(buf)?;
			
			let mut hash = [0u8; 32];
			prev.read_exact(&mut hash)?;
			
			prev.seek(SeekFrom::Current(8))?;
			
			prev_files.entry(hash).or_insert_with(|| Vec::new()).push(path);
			
			read_len += 1 + path_len + 32 + 8;
			if read_len == index_len {break}
		}
		
		let w = &mut writer;
		1u8.write_to(w)?;
		version.write_to(w)?;
		prev_version.write_to(w)?;
		
		Ok(ModPack {
			inner: Mutex::new(writer),
			index: Mutex::new(Index::default()),
			prev_files,
		})
	}
	
	pub fn load(mut reader: W) -> Result<ModPack<W>, Error> {
		let mut index = Index::default();
		
		reader.seek(SeekFrom::End(-4))?;
		log!("{}", reader.stream_position().unwrap());
		let index_len = reader.read_le::<u32>()? as i64;
		log!("{index_len}");
		reader.seek(SeekFrom::End(-4 - index_len))?;
		let mut read_len = 0;
		loop {
			let path_len = reader.read_le::<u8>()? as i64;
			let mut buf = vec![0u8; path_len as usize];
			reader.read_exact(&mut buf)?;
			let path = String::from_utf8(buf)?;
			
			let mut hash = [0u8; 32];
			reader.read_exact(&mut hash)?;
			
			let offset = reader.read_le::<i64>()? as i64;
			
			// index.entry(hash).or_insert_with(|| HashMap::new()).insert(path, offset);
			index.locations.insert(hash.clone(), offset);
			index.paths.insert(path, hash);
			
			read_len += 1 + path_len + 32 + 8;
			if read_len == index_len {break}
		}
		
		Ok(ModPack {
			inner: Mutex::new(reader),
			index: Mutex::new(index),
			prev_files: HashMap::new(),
		})
	}
	
	pub fn write_file<S, R>(&self, path: S, mut file: R) -> Result<(), Error> where
	S: Into<String>,
	R: Read + Seek {
		let mut buf = [0u8; 4096];
		let mut hasher = blake3::Hasher::new();
		while let readcount = file.read(&mut buf)? && readcount != 0 {
			hasher.update(&buf[0..readcount]);
		}
		
		let hash = hasher.finalize().as_bytes().to_owned();
		if self.prev_files.contains_key(&hash) {
			// self.index.lock().unwrap().entry(hash).or_insert_with(|| HashMap::new()).insert(path.into(), -1);
			let mut index = self.index.lock().unwrap();
			index.locations.insert(hash.clone(), -1);
			index.paths.insert(path.into(), hash);
		} else {
			let mut writer = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::best());
			
			file.seek(SeekFrom::Start(0))?;
			while let readcount = file.read(&mut buf)? && readcount != 0 {
				writer.write_all(&buf[0..readcount])?;
			}
			
			let mut inner = self.inner.lock().unwrap();
			let offset = inner.stream_position()? as i64;
			let mut index = self.index.lock().unwrap();
			if index.locations.insert(hash.clone(), offset).is_none() {
				let blob = writer.finish()?;
				inner.write_all(&(blob.len() as u32).to_le_bytes())?;
				inner.write_all(&blob)?;
			}
			index.paths.insert(path.into(), hash);
		}
		
		Ok(())
	}
	
	pub fn read_file<S>(&self, path: S) -> Result<Vec<u8>, Error> where
	S: AsRef<str> {
		let index = self.index.lock().unwrap();
		let offset = *index.locations.get(index.paths.get(path.as_ref()).ok_or("Invalid Path")?).expect("Hash was invalid, how, what");
		if offset == -1 {Err("File is patch file")?}
		
		self.read_file_offset(offset)
	}
	
	pub fn read_file_offset(&self, offset: i64) -> Result<Vec<u8>, Error> {
		let mut inner = self.inner.lock().unwrap();
		inner.seek(SeekFrom::Start(offset as u64))?;
		let mut buf = vec![0; inner.read_le::<u32>()? as usize];
		inner.read_exact(&mut buf)?;
		drop(inner);
		
		let mut f = flate2::write::ZlibDecoder::new(Cursor::new(Vec::new()));
		f.write_all(&buf)?;
		Ok(f.finish()?.into_inner())
	}
	
	// Cloning this stuff isnt great. TODO: figure out how to return a reference from a mutex locked item
	pub fn paths_valid(&'a self) -> Vec<String> {
		let index = self.index.lock().unwrap();
		index.paths.iter().filter_map(|(path, hash)| index.locations.get(hash).map_or(None, |o| if *o == -1 {None} else {Some(path.to_owned())})).collect()
	}
	
	pub fn index(&self) -> Index {
		self.index.lock().unwrap().clone()
	}
	
	pub fn version(&self) -> i32 {
		let mut inner = self.inner.lock().unwrap();
		inner.seek(SeekFrom::Start(1)).unwrap();
		inner.read_le::<i32>().unwrap()
	}
	
	pub fn finish(self) -> Result<W, Error> {
		let mut inner = self.inner.into_inner().unwrap();
		let index = self.index.into_inner().unwrap();
		let w = &mut inner;
		
		let mut index_len = 0u32;
		for (path, hash) in index.paths {
			(path.len() as u8).write_to(w)?;
			path.as_bytes().write_to(w)?;
			hash.write_to(w)?;
			index.locations[&hash].write_to(w)?;
			
			index_len += 1 + path.len() as u32 + 32 + 8;
		}
		// for (hash, files) in self.index.into_inner().unwrap() {
		// 	for (path, offset) in files {
		// 		(path.len() as u8).write_to(w)?;
		// 		path.as_bytes().write_to(w)?;
		// 		hash.write_to(w)?;
		// 		offset.write_to(w)?;
				
		// 		index_len += 1 + path.len() as u32 + 32 + 8;
		// 	}
		// }
		index_len.write_to(w)?;
		
		Ok(inner)
	}
	
	pub fn into_inner(self) -> W {
		self.inner.into_inner().unwrap()
	}
	
	/// Manually mark a file as existing.
	/// 
	/// Useful for when creating a patch and are certain a file is the same to avoid reading and hashing it.
	pub fn mark_file<S>(&self, path: S, hash: [u8; 32]) where
	S: Into<String> {
		// self.index.lock().unwrap().entry(hash).or_insert_with(|| HashMap::new()).insert(path.into(), -1);
		let mut index = self.index.lock().unwrap();
		index.locations.insert(hash.clone(), -1);
		index.paths.insert(path.into(), hash);
	}
	
	/// Mark all files as existing, only works if creating a path
	/// 
	/// Useful for when files are only changed or added, and none removed.
	pub fn mark_all(&self) {
		let mut index = self.index.lock().unwrap();
		for (hash, paths) in &self.prev_files {
			index.locations.insert(hash.clone(), -1);
			for path in paths {
				index.paths.insert(path.to_owned(), hash.clone());
			}
			// let p = index.entry(hash.to_owned()).or_insert_with(|| HashMap::new());
			// for path in paths {
			// 	p.insert(path.to_owned(), -1);
			// }
		}
	}
}

pub fn pack<W, P, R>(writer: W, mod_path: P, version: i32, prev: Option<R>, progress: Arc<Mutex<(usize, usize)>>) -> Result<(), Error> where
W: Write + Read + Seek + std::marker::Send + std::fmt::Debug,
P: AsRef<Path>,
R: Read + Seek {
	let mod_path = mod_path.as_ref();
	let datas: crate::apply::Datas = serde_json::from_reader(File::open(mod_path.join("datas.json")).unwrap()).unwrap();
	let mut files = Vec::new();
	
	// Penumbra files
	if let Some(penumbra) = &datas.penumbra {
		penumbra.files.iter().for_each(|(_, layers)| layers.0.iter().for_each(|layer| layer.paths.iter().for_each(|p| files.push(p.as_str()))));
		penumbra.options.iter().for_each(|o| match o {
				ConfOption::Single(opt) | ConfOption::Multi(opt) => opt.options.iter().for_each(|o| o.files.iter().for_each(|(_, layers)| layers.0.iter().for_each(|layer| layer.paths.iter().for_each(|p| files.push(p.as_str()))))),
				_ => {},
			}
		);
	}
	
	// Generic files
	files.push("datas.json");
	
	// Modpack creation
	for path in &files {
		if !mod_path.join(path).exists() {Err(format!("{path} is an invalid file"))?}
	}
	
	// let releases_path = mod_path.join("releases");
	// if !releases_path.exists() {fs::create_dir(&releases_path)?}
	
	// let mut modpack = Arc::new(ModPack::new(File::create(releases_path.join(version_to_string(version)))?, version)?);
	let modpack = Arc::new(match prev {
		Some(prev) => ModPack::new_patch(writer, version, prev)?,
		None => ModPack::new(writer, version)?,
	});
	
	progress.lock().unwrap().1 = files.len();
	files.into_par_iter().for_each(|path| {
		modpack.write_file(path, File::open(mod_path.join(path)).unwrap()).unwrap();
		// *progress = (progress.0 + 1, total);
		let mut p = progress.lock().unwrap();
		p.0 += 1;
	});
	Arc::try_unwrap(modpack).unwrap().finish()?;
	
	Ok(())
}

pub fn unpack<W, F>(reader: W, file_handler: F) -> Result<(), Error> where
W: Write + Read + Seek + std::marker::Send + std::fmt::Debug,
F: Fn(HashSet<&str>, &[u8; 32], &[u8]) + std::marker::Sync {
	let modpack = ModPack::load(reader)?;
	let mut files = HashMap::new();
	
	let index = modpack.index();
	for (hash, offset) in &index.locations {
		files.entry(hash).or_insert_with(|| (offset, HashSet::new()));
	}
	for (path, hash) in &index.paths {
		files.get_mut(hash).expect("Hash was invalid, how, what").1.insert(path.as_str());
	}
	
	files.into_par_iter().for_each(|(hash, (offset, paths))| {
		let data = modpack.read_file_offset(*offset).unwrap();
		file_handler(paths, hash, &data);
	});
	
	Ok(())
}

pub fn unpack_to<W, P>(reader: W, mod_path: P) -> Result<(), Error> where
W: Write + Read + Seek + std::marker::Send + std::fmt::Debug,
P: AsRef<Path> {
	let mod_path = mod_path.as_ref();
	let files_path = mod_path.join("files");
	if !files_path.exists() {fs::create_dir_all(&files_path)?}
	
	unpack(reader, |paths, hash, data| {
		// TODO: handle non penumbra files in a better way in the future
		// TODO: deal with errors
		if paths.contains("datas.json") {
			File::create(mod_path.join("datas.json")).unwrap().write_all(data).unwrap();
		} else {
			File::create(files_path.join(crate::hash_str(hash))).unwrap().write_all(data).unwrap();
		}
	})
}

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