use std::{io::{Read, Seek, SeekFrom}, collections::HashMap, ops::{Deref, DerefMut}, fs::File, rc::Rc, cmp::Ordering, path::{PathBuf, Path}};
use binrw::BinWrite;
use flate2::read::GzDecoder;

struct LazyBranch {
	name: Rc<str>,
	offset: u32,
	branches: Option<Vec<LazyBranch>>,
}

pub struct LazyTree {
	data: Vec<u8>,
	branches: Vec<LazyBranch>,
	entryfn: Box<dyn Fn(String, egui::Response)>,
}

impl LazyTree {
	pub fn new(entryfn: Box<dyn Fn(String, egui::Response)>) -> Self {
		Self {
			branches: Vec::new(),
			data: Vec::new(),
			entryfn,
		}
	}
	
	pub fn load(&mut self, data: impl Into<Vec<u8>>) -> Result<(), super::BacktraceError> {
		let data = data.into();
		
		self.branches = Self::load_branch(&data, 0)?;
		self.data = data;
		
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

// ----------

struct StaticBranch {
	name: String,
	option: Option<(String, String)>,
	real_path: Option<String>,
	disabled: bool,
	branches: Vec<StaticBranch>,
}

pub struct StaticTree {
	branches: Vec<StaticBranch>,
	entryfn: Box<dyn Fn(String, Option<String>, Option<(String, String)>, egui::Response)>,
}

impl StaticTree {
	pub fn new(entryfn: Box<dyn Fn(String, Option<String>, Option<(String, String)>, egui::Response)>) -> Self {
		Self {
			branches: Vec::new(),
			entryfn,
		}
	}
	
	// pub fn set_disabled(&mut self, disabled: bool, path: &str) {
	// 	// todo?
	// }
	
	pub fn add_path(&mut self, path: &str, real_path: &str, option: Option<(String, String)>) {
		fn sort_branch(branches: &mut Vec<StaticBranch>) {
			// TODO: include option strings in sort
			branches.sort_by(|a, b|
				(if a.branches.len() == 0 && b.branches.len() != 0 {Ordering::Greater} else if a.branches.len() != 0 && b.branches.len() == 0 {Ordering::Less} else {Ordering::Equal})
				.then(a.name.to_ascii_lowercase().cmp(&b.name.to_ascii_lowercase())));
		}
		
		let segs = path.split("/").collect::<Vec<_>>();
		let mut branches = &mut self.branches;
		for i in 0..(segs.len() - 1) {
			let seg = segs[i];
			if !branches.iter().any(|branch| branch.name == seg) {
				branches.push(StaticBranch {
					name: seg.to_string(),
					option: None,
					real_path: None,
					disabled: false,
					branches: Vec::new(),
				});
				sort_branch(branches);
			}
			
			branches = &mut branches.iter_mut().find(|branch| branch.name == seg).unwrap().branches;
		}
		
		branches.push(StaticBranch {
			name: segs.last().unwrap().to_string(),
			option: option,
			real_path: Some(real_path.to_string()),
			disabled: false,
			branches: Vec::new(),
		});
		sort_branch(branches);
	}
	
	fn render_branch(ui: &mut egui::Ui, branch: &StaticBranch, path: String, mut disabled: bool, entryfn: &Box<dyn Fn(String, Option<String>, Option<(String, String)>, egui::Response)>) {
		disabled = disabled || branch.disabled;
		if branch.branches.len() != 0 {
			ui.collapsing(&branch.name, |ui| {
				for branch in &branch.branches {
					Self::render_branch(ui, branch, format!("{path}/{}", branch.name), disabled, entryfn);
				}
			});
		} else {
			let resp = if let Some((option, sub_option)) = &branch.option {
				egui::Button::new(format!("{} [{option}][{sub_option}]", branch.name))
			} else {
				egui::Button::new(&branch.name)
			};
			
			entryfn(path, branch.real_path.clone(), branch.option.clone(), ui.add_enabled(!disabled, resp));
		}
	}
}

// ----------


pub struct TreeData {
	pub mod_trees: Vec<(String, PathBuf, StaticTree)>,
	pub game_paths: LazyTree,
}

pub struct Tree {
	data: Rc<std::cell::RefCell<TreeData>>,
	openmodfn: Box<dyn Fn(&Path)>,
	opening_mod: super::DialogStatus,
}

impl Tree {
	pub fn new(data: Rc<std::cell::RefCell<TreeData>>, openmodfn: Box<dyn Fn(&Path)>) -> Self {
		Self {
			data,
			openmodfn,
			opening_mod: super::DialogStatus::None,
		}
	}
	
	fn create_mod(&mut self, path: &std::path::Path) {
		crate::modman::meta::Meta::default().save(&path.join("meta.json")).unwrap();
		
		(self.openmodfn)(path);
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
		let mut delete = None;
		for (mod_name, mod_path, mod_tree) in &self.data.borrow().mod_trees {
			ui.collapsing(mod_name, |ui| {
				(mod_tree.entryfn)("\0meta".to_string(), None, None, ui.button("Meta"));
				
				ui.collapsing("Files", |ui| {
					for branch in &mod_tree.branches {
						StaticTree::render_branch(ui, branch, branch.name.to_string(), false, &mod_tree.entryfn);
					}
				});
			}).header_response.context_menu(|ui| {
				if ui.button("Remove from list").clicked() {
					delete = Some(mod_path.clone());
				}
			});
		}
		
		if let Some(mod_path) = delete {
			self.data.borrow_mut().mod_trees.retain(|(_, path, _)| path != &mod_path);
			crate::config().config.mod_paths.retain(|path| path != &mod_path);
			_ = crate::config().save_forced();
		}
		
		if ui.button("📂 Open mod").clicked() && matches!(self.opening_mod, super::DialogStatus::None) {
			let mut dialog = egui_file::FileDialog::select_folder(Some(crate::config().config.file_dialog_path.clone()))
				.title("Open mod folder");
			dialog.open();
			self.opening_mod = super::DialogStatus::Dialog(dialog);
		}
		
		if let super::DialogStatus::Dialog(dialog) = &mut self.opening_mod {
			if dialog.show(ui.ctx()).selected() {
				if let Some(path) = dialog.path() {
					let path = path.to_owned();
					
					if let Some(parent) = path.parent() {
						crate::config().config.file_dialog_path = parent.to_owned();
						_ = crate::config().save_forced();
					}
					
					if path.join("meta.json").exists() {
						(self.openmodfn)(&path);
					} else {
						self.opening_mod = super::DialogStatus::CreateReq(path);
					}
				}
			}
		} else if let super::DialogStatus::CreateReq(path) = &self.opening_mod {
			let path = path.to_owned();
			egui::Window::new("Create mod").show(ui.ctx(), |ui| {
				ui.label("This folder is not a mod, do you want to create one?");
				// TODO: warning here if the target folder is not empty
				ui.label(format!("Path: {}", path.display()));
				ui.horizontal(|ui| {
					if ui.button("Create mod").clicked() {
						self.create_mod(&path);
						self.opening_mod = super::DialogStatus::None;
					}
					
					if ui.button("Cancel").clicked() {
						self.opening_mod = super::DialogStatus::None;
					}
				})
			});
		}
		
		ui.add_space(20.0);
		
		ui.collapsing("Game Paths", |ui| {
			let game_paths = &mut self.data.borrow_mut().game_paths;
			for branch in &mut game_paths.branches {
				LazyTree::render_branch(&game_paths.data, ui, branch, branch.name.to_string(), &game_paths.entryfn);
			}
		});
		
		Ok(())
	}
}

// ----------

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