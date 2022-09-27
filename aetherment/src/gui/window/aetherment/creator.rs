use std::{path::PathBuf, fs::File, sync::{Arc, Mutex}, io::Write, collections::HashMap};
use binrw::{BinReaderExt, BinWrite};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::{gui::aeth::{self, F2}, CLIENT, SERVER, creator::modpack};

const NAMEMAX: usize = 64;
const DESCMAX: usize = 5000;

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
struct Meta {
	name: String,
	description: String,
	contributors: Vec<i32>,
	dependencies: Vec<i32>,
	nsfw: bool,
	previews: Vec<String>,
}

#[derive(Deserialize, Clone, Debug)]
struct OnlineMod {
	id: i32,
	name: String,
	description: String,
	tags: Vec<i16>,
	previews: Vec<String>,
	nsfw: bool,
	version: i32,
}

struct CurMod {
	meta: Meta,
	online: Option<OnlineMod>,
	tags: HashMap<String, Vec<String>>,
	version: [i32; 4],
}

pub struct Tab {
	refresh: Arc<Mutex<bool>>,
	mod_entries: Vec<(String, Option<OnlineMod>)>,
	online_mods: Vec<OnlineMod>,
	storage: Option<(i64, i64)>,
	selected_mod: String,
	curmod: Option<CurMod>,
	newmod: String,
	importing: bool,
}

impl Tab {
	pub fn new(state: &mut crate::Data) -> Self {
		let mut t = Tab {
			refresh: Arc::new(Mutex::new(false)),
			mod_entries: Vec::new(),
			online_mods: Vec::new(),
			storage: None,
			selected_mod: "".to_owned(),
			curmod: None,
			newmod: String::with_capacity(64),
			importing: false,
		};
		
		t.load_mods(state);
		
		t
	}
	
	pub fn draw(&mut self, state: &mut crate::Data) {
		if *self.refresh.lock().unwrap() {
			*self.refresh.lock().unwrap() = false;
			self.load_mods(state);
			self.load_mod(PathBuf::from(&state.config.local_path).join(&self.selected_mod));
		}
		
		if let Some(storage) = &self.storage {
			imgui::text(&format!("{} / {}", storage.1, storage.0));
		}
		
		aeth::divider("div", false)
		.left(100.0, || {
			aeth::child("mods", [0.0, -aeth::frame_height() - imgui::get_style().item_spacing.y()], false, imgui::WindowFlags::None, || {
				for i in 0..self.mod_entries.len() {
					let e = self.mod_entries.get(i).unwrap();
					let name = &e.0;
					
					if e.1.is_some() {
						aeth::icon(""); // fa-globe
						imgui::same_line();
					}
					
					if imgui::selectable(name, name == &self.selected_mod, imgui::SelectableFlags::None, [0.0, 0.0]) {
						self.selected_mod = name.to_owned();
						self.load_mod(PathBuf::from(&state.config.local_path).join(name));
					}
				}
				
				aeth::offset([0.0, 30.0]);
				for m in &self.online_mods {
					aeth::icon("");
					imgui::same_line();
					// TODO: make this selectable, show page to download the mod to local machine
					imgui::selectable(&m.name, false, imgui::SelectableFlags::None, [0.0, 0.0]);
				}
			});
			
			// Footer
			if aeth::button_icon("") { // fa-redo-alt
				self.load_mods(state);
			}
			aeth::tooltip("Reload Modlist");
			
			imgui::same_line();
			if aeth::button_icon("") { // fa-file-import
				self.importing = true;
			}
			aeth::tooltip("Import Mod");
			
			imgui::same_line();
			if aeth::button_icon("") { // fa-plus
				let path = PathBuf::from(&state.config.local_path).join(&self.newmod);
				std::fs::create_dir_all(&path).unwrap();
				File::create(path.join("datas.json")).unwrap().write_all(crate::serialize_json(json!(crate::apply::Datas::default())).as_bytes()).unwrap();
				self.newmod.clear();
				self.load_mods(state);
				self.load_mod(path);
			}
			aeth::tooltip("Create Mod");
			
			imgui::same_line();
			aeth::next_max_width();
			imgui::input_text_with_hint("##newmod", "New Mod", &mut self.newmod, imgui::InputTextFlags::None);
			
			// Mod Importing
			if self.importing {
				// dalamud filedialog doesnt support selecting a folder or file at the same time unless im stupid
				// TODO: find a sollution
				match aeth::file_dialog(aeth::FileDialogMode::OpenFile, "Importing Mod".to_owned(), "".to_owned(), Vec::new()) {
					aeth::FileDialogResult::Success(path_s) => {
						self.importing = false;
						let path = PathBuf::from(&path_s);
						if path.is_dir() {
							if path.join("meta.json").exists() && path.join("default_mod.json").exists() {
								crate::creator::import::penumbra::import(&path, PathBuf::from(&state.config.local_path).join(path.file_name().unwrap())).unwrap();
							} else if path.join("options.json").exists() && path.join("elements_black").exists() {
								crate::creator::import::v1::import(&path, PathBuf::from(&state.config.local_path).join(path.file_name().unwrap())).unwrap();
							} else {
								aeth::show_error("Mod Import Failed", format!("{path_s} Is not a valid penumbra directory."));
							}
						} else {
							let ext = path.extension().unwrap().to_str().unwrap();
							match ext {
								"pap" => aeth::show_error("Mod Import Failed", "todo"),
								"ttmp" | "ttmp2" => aeth::show_error("Mod Import Failed", "TexTool modpacks are currently unsupported."),
								_ => aeth::show_error("Mod Import Failed", "Invalid file for importing.")
							}
						}
					},
					aeth::FileDialogResult::Failed => self.importing = false, // TODO: display that it failed
					aeth::FileDialogResult::Busy => {},
						}
			}
		}).right(400.0, || {
			if self.curmod.is_none() {return}
			let m = self.curmod.as_mut().unwrap();
			
			imgui::input_text("Name", &mut m.meta.name, imgui::InputTextFlags::None);
			let limit = m.meta.name.len() >= NAMEMAX;
			if limit {imgui::push_style_color(imgui::Col::Text, 0xFF3030B0)}
			imgui::text(&format!("{}/{}", m.meta.name.len(), NAMEMAX));
			if limit {imgui::pop_style_color(1)}
			
			imgui::input_text_multiline("Description", &mut m.meta.description, [0.0, 400.0], imgui::InputTextFlags::None);
			let limit = m.meta.name.len() >= DESCMAX;
			if limit {imgui::push_style_color(imgui::Col::Text, 0xFF3030B0)}
			imgui::text(&format!("{}/{}", m.meta.description.len(), DESCMAX));
			if limit {imgui::pop_style_color(1)}
			
			imgui::text("Contributors: TODO");
			imgui::text("Dependencies: TODO");
			
			imgui::checkbox("NSFW", &mut m.meta.nsfw);
			
			imgui::text("Previews: TODO");
			
			if imgui::input_int4("Version", &mut m.version, imgui::InputTextFlags::None) {
				let (a, b, c, d) = if let Some(online) = &m.online {(
					(online.version >> 24) & 0xFF,
					(online.version >> 16) & 0xFF,
					(online.version >> 8) & 0xFF,
					(online.version) & 0xFF
				)} else {
					(0, 0, 0, 0)
				};
				
				m.version[0] = m.version[0].clamp(a, 100);
				m.version[1] = m.version[1].clamp(b, 100);
				m.version[2] = m.version[2].clamp(c, 100);
				m.version[3] = m.version[3].clamp(d, 100);
			}
			
			imgui::text("Tags: ");
			imgui::same_line();
			for (tag, paths) in &m.tags {
				imgui::text(tag);
				imgui::same_line();
				if imgui::is_item_hovered() {
					imgui::begin_tooltip();
					for path in paths {
						imgui::text(path);
					}
					imgui::end_tooltip();
				}
			}
			
			imgui::new_line();
			
			let version = (m.version[0] << 24) + (m.version[1] << 16) + (m.version[2] << 8) + m.version[3];
			// if let Some(user) = &state.user && (m.online.is_none() || version > m.online.as_ref().unwrap().version) {
			// 	if imgui::button("Upload", [0.0, 0.0]) {
			// 		let mut req = CLIENT.post(format!("{}/mod/upload", SERVER))
			// 			.header("Authorization", user.token.clone())
			// 			.header("Mod-Name", m.meta.name.clone())
			// 			.header("Mod-Description", m.meta.description.clone())
			// 			.header("Mod-Nsfw", if m.meta.nsfw {"true"} else {"false"})
			// 			.header("Mod-Version", version)
			// 			.header("Mod-Patch-Notes", "") // TODO: patchnotes
			// 			.header("Mod-Patch", if m.online.is_some() {"true"} else {"false"});
					
			// 		// let pack;
			// 		let mod_path = PathBuf::from(&state.config.local_path).join(&self.selected_mod);
			// 		let releases_path = mod_path.join("releases");
			// 		if let Some(online) = &m.online {
			// 			if !releases_path.exists() {
			// 				// TODO: popup
			// 				log!(err, "Local copy out of sync from remote");
			// 				return
			// 			}
						
			// 			req = req.header("Mod-Id", online.id);
						
			// 			let version_str = modpack::version_to_string(online.version);
			// 			let latest = match std::fs::read_dir(&releases_path)
			// 				.unwrap()
			// 				.into_iter()
			// 				.find(|e| {
			// 					let name = e.as_ref().unwrap().file_name();
			// 					let name = name.to_str().unwrap();
								
			// 					return name.ends_with(&format!("{version_str}.amp")) || name.ends_with(&format!("{version_str}.amp.patch"));
			// 				}) {
			// 				Some(v) => v.unwrap().path(),
			// 				None => {
			// 					log!(err, "Local copy out of sync from remote");
			// 					return
			// 				}
			// 			};
			// 			log!("online!");
			// 			let paths = modpack::pack(mod_path.clone(), version, true, Some(latest));
			// 			pack = paths.1.unwrap_or_else(|| paths.0.unwrap());
			// 		} else {
			// 			let paths = modpack::pack(mod_path.clone(), version, true, None);
			// 			pack = paths.1.unwrap_or_else(|| paths.0.unwrap());
			// 		}
					
			// 		req = req.header("Content-Length", pack.metadata().unwrap().len().to_string());
			// 		let refresh = self.refresh.clone();
					
			// 		std::thread::spawn(move || {
			// 			let resp = req
			// 				.body(File::open(pack).unwrap())
			// 				.send()
			// 				.unwrap()
			// 				.text()
			// 				.unwrap();
						
			// 			log!("upload resp: {resp}");
						
			// 			let resp: serde_json::Value = serde_json::from_str(&resp).unwrap();
						
			// 			if let Some(err) = resp["error"].as_str() {
			// 				log!(err, "{err}");
			// 				return
			// 			}
						
			// 			#[derive(Deserialize, Clone, Debug)]
			// 			struct Mod {
			// 				id: i32,
			// 			}
			// 			let uploaded_mod: Mod = serde_json::from_value(resp).unwrap();
			// 			uploaded_mod.id.write_to(&mut File::create(mod_path.join("aeth")).unwrap()).unwrap();
			// 			*refresh.lock().unwrap() = true;
			// 		});
			// 	}
			// }
			
			// if imgui::button("create modpack", [0.0, 0.0]) {
			// 	let path = PathBuf::from(&state.config.local_path).join(&self.selected_mod);
			// 	std::thread::spawn(move || {
			// 		crate::creator::modpack::pack(path, 1 << 24, true);
			// 		// crate::creator::modpack::pack(path, (1 << 24) + (1 << 16), true);
			// 	});
			// }
		});
	}
	
	pub fn load_mods(&mut self, state: &mut crate::Data) {
		let online_mods = if let Some(user) = &state.user {
			#[derive(Deserialize, Clone, Debug)]
			struct Stats {
				total_storage: i64,
				used_storage: i64,
				mods: Vec<OnlineMod>,
			}
			
			match CLIENT.get(format!("{}/user/stats", SERVER))
				.header("Authorization", &user.token)
				.send() {
				Ok(v) => {
					let stats: serde_json::Value = v.json().unwrap();
					
					log!("{:?}", stats);
					if stats["error"].is_string() {
						state.user = None;
						return;
					}
					let stats: Stats = serde_json::from_value(stats).unwrap();
					
					self.storage = Some((stats.total_storage, stats.used_storage));
					stats.mods
				},
				Err(_) => Vec::new(),
			}
		} else {
			Vec::new()
		};
		
		self.mod_entries = std::fs::read_dir(&state.config.local_path)
			.unwrap()
			.into_iter()
			.filter_map(|e| {
				let e = e.unwrap();
				if e.metadata().unwrap().is_dir() {
					Some((
						e.file_name().to_str().unwrap().to_owned(),
						if let Ok(mut f) = File::open(e.path().join("aeth")) {
							// Some(f.read_le::<i32>().unwrap())
							let id = f.read_le::<i32>().unwrap();
							match online_mods.iter().find(|m| m.id == id) {
								Some(v) => Some(v.clone()),
								None => None,
							}
						} else {
							None
						}
					))
				} else {
					None
				}
			})
			.collect();
		
		self.online_mods = online_mods.into_iter().filter(|m| self.mod_entries.iter().find(|v| v.1.is_some() && v.1.as_ref().unwrap().id == m.id).is_none()).collect();
	}
	
	pub fn load_mod(&mut self, path: PathBuf) {
		let mut m = match File::open(path.join("meta.json")) {
			Ok(f) => serde_json::from_reader(f).unwrap(),
			Err(_) => Meta {
				name: path.file_name().unwrap().to_str().unwrap().to_owned(),
				description: String::new(),
				contributors: Vec::new(),
				dependencies: Vec::new(),
				nsfw: false,
				previews: Vec::new(),
			}
		};
		
		m.name.reserve(NAMEMAX * 4); // *4 cuz unicode
		m.description.reserve(DESCMAX * 4);
		
		let name = path.file_name().unwrap().to_str().unwrap();
		let online = self.mod_entries.iter().find_map(|m| if m.0 == name && m.1.is_some() {Some(m.1.as_ref().unwrap().clone())} else {None}).clone();
		
		log!("{:?}", online);
		self.curmod = Some(CurMod {
			meta: m,
			tags: serde_json::from_reader::<File, crate::apply::Datas>(File::open(path.join("datas.json")).unwrap()).unwrap().tags(),
			version: match &online {
				Some(v) => {
					[
						(v.version >> 24) & 0xFF,
						(v.version >> 16) & 0xFF,
						(v.version >> 8) & 0xFF,
						(v.version) & 0xFF
					]
				},
				None => [1, 0, 0, 0],
			},
			online: online,
		});
	}
}