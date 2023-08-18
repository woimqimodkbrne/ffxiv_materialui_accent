use std::{io::{Read, Seek, SeekFrom}, collections::HashMap, ops::{Deref, DerefMut}, fs::File, rc::Rc, cmp::Ordering};
use binrw::BinWrite;
use flate2::read::GzDecoder;

// TODO: seperate website as to not overload perchbird and know when to download a new version
// also perhabs add a logger to the plugin to contribute
// and probably put the path file creation on the server so it only has to be done once
const PATHSURL: &'static str = "https://rl2.perchbird.dev/download/export/CurrentPathList.gz";

#[derive(Debug, Default)]
struct Branch<'a>(HashMap<&'a str, Branch<'a>>);
impl<'a> Deref for Branch<'a> {
	type Target = HashMap<&'a str, Branch<'a>>;
	
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<'a> DerefMut for Branch<'a> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}


struct LazyBranch {
	name: Rc<str>,
	offset: u32,
	branches: Option<Vec<LazyBranch>>,
}

pub struct Tree {
	data: Vec<u8>,
	branches: Vec<LazyBranch>,
	entryfn: Box<dyn Fn(String, egui::Response)>,
}

impl Tree {
	pub fn new() -> Self {
		Self {
			branches: Vec::new(),
			data: Vec::new(),
			entryfn: Box::new(|_, _| {}),
		}
	}
	
	pub fn load(&mut self, entryfn: Box<dyn Fn(String, egui::Response)>) -> Result<(), super::BacktraceError> {
		let mut f = File::open(dirs::cache_dir().unwrap().join("Aetherment").join("paths"))?;
		let mut data = Vec::new();
		f.read_to_end(&mut data)?;
		
		self.branches = Self::load_branch(&data, 0)?;
		self.data = data;
		self.entryfn = Box::new(entryfn);
		
		Ok(())
	}
	
	fn load_branch(data: &[u8], offset: u32) -> Result<Vec<LazyBranch>, super::BacktraceError> {
		let mut offset = offset as usize;
		let mut branch = Vec::new();
		
		offset += 2;
		for _ in 0..u16::from_le_bytes(data[offset - 2 .. offset].try_into()?) {
			let name_len = data[offset] as usize;
			offset += 1;
			let name = std::str::from_utf8(&data[offset .. offset + name_len])?;
			offset += name_len;
			let sub_offset = u32::from_le_bytes(data[offset .. offset + 4].try_into()?);
			offset += 4;
			
			branch.push(LazyBranch {
				name: Rc::from(name),
				offset: sub_offset,
				branches: None,
			});
		}
		
		branch.sort_by(|a, b| 
			(if a.offset == 0 && b.offset != 0 {Ordering::Greater} else if a.offset != 0 && b.offset == 0 {Ordering::Less} else {Ordering::Equal})
			.then(a.name.to_ascii_lowercase().cmp(&b.name.to_ascii_lowercase()))
		);
		
		Ok(branch)
	}
	
	fn render_branch(data: &[u8], ui: &mut egui::Ui, branch: &mut LazyBranch, path: String, entryfn: &Box<dyn Fn(String, egui::Response)>) {
		if branch.offset != 0 {
			ui.collapsing(branch.name.as_ref(), |ui| {
				let branches = branch.branches.get_or_insert_with(|| Self::load_branch(data, branch.offset).unwrap());
				for branch in branches {
					Self::render_branch(data, ui, branch, format!("{path}/{}", branch.name), entryfn);
				}
			});
		} else {
			entryfn(path, ui.button(branch.name.as_ref()));
		}
	}
}

impl super::View for Tree {
	fn is_tree(&self) -> bool {
		true
	}
	
	fn name(&self) -> &'static str {
		"File Tree"
	}
	
	fn path(&self) -> &'static str {
		"_filetree"
	}
	
	fn render(&mut self, ui: &mut egui::Ui) -> Result<(), super::BacktraceError> {
		for branch in &mut self.branches {
			Self::render_branch(&self.data, ui, branch, branch.name.to_string(), &self.entryfn);
		}
		
		Ok(())
	}
}

pub fn update_paths() {
	std::thread::spawn(move || {
		if let Err(e) = (|| -> Result<(), super::BacktraceError> {
			log!("downloading");
			let data = reqwest::blocking::get(PATHSURL)?.bytes()?;
			// let mut resp = reqwest::blocking::get(PATHSURL)?;
			// let mut data = Vec::with_capacity(40000);
			// let mut buf = [0u8; 16384];
			// loop {
			// 	let readcount = resp.read(&mut buf)?;
			// 	if readcount == 0 {break}
			// 	data.extend_from_slice(&buf[..readcount]);
			// }
			
			log!("decoding");
			let mut decoder = GzDecoder::new(&data[..]);
			let mut paths = String::new();
			decoder.read_to_string(&mut paths)?;
			
			log!("making tree");
			let mut tree = Branch::default();
			for path in paths.split("\n") {
				let mut branch = &mut tree;
				for seg in path.split("/") {
					branch = branch.entry(seg).or_insert_with(|| Branch::default());
				}
			}
			
			log!("writing tree");
			let cache_dir = dirs::cache_dir().ok_or("No Cache Dir (???)").unwrap().join("Aetherment");
			std::fs::create_dir_all(&cache_dir)?;
			let mut paths_file = File::create(cache_dir.join("paths"))?;
			fn write_branch(branch: &Branch, mut file: &mut File) -> Result<(), super::BacktraceError> {
				let mut offsets = HashMap::new();
				
				(branch.len() as u16).write_le(&mut file)?;
				for (name, sub_branch) in branch.iter() {
					(name.len() as u8).write_le(&mut file)?;
					name.as_bytes().write_le(&mut file)?;
					offsets.insert(file.stream_position()?, sub_branch);
					0u32.write_le(&mut file)?; // list offset, we write over this later
				}
				
				// now that we wrote the list we can write the lists of the items
				for (offset, sub_branch) in offsets {
					if sub_branch.len() > 0 {
						// overwrite the offset
						let pos = file.stream_position()? as u32;
						file.seek(SeekFrom::Start(offset))?;
						pos.write_le(&mut file)?;
						file.seek(SeekFrom::End(0))?;
						write_branch(sub_branch, file)?;
					}
				}
				
				Ok(())
			}
			
			write_branch(&tree, &mut paths_file)?;
			
			Ok(())
		})() {
			log!(err, "Failed fetching paths {e:?}");
		}
		
		log!("done");
	});
}