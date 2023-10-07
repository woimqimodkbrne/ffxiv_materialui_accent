use std::{io::{Read, Write}, fs::File, collections::HashMap, borrow::Cow};
use serde::{Deserialize, Serialize};
use crate::{log, resource_loader::read_json};

#[repr(packed)]
pub struct GetModSettings {
	pub exists: bool,
	pub enabled: bool,
	pub inherit: bool,
	pub priority: i32,
	pub options: HashMap<String, Vec<String>>,
}

pub struct PenumbraFunctions {
	pub redraw: Box<dyn Fn()>,
	pub redraw_self: Box<dyn Fn()>,
	pub root_path: Box<dyn Fn() -> std::path::PathBuf>,
	pub mod_list: Box<dyn Fn() -> Vec<String>>,
	pub add_mod_entry: Box<dyn Fn(&str) -> u8>,
	pub reload_mod: Box<dyn Fn(&str) -> u8>,
	pub set_mod_enabled: Box<dyn Fn(&str, &str, bool) -> u8>,
	pub set_mod_priority: Box<dyn Fn(&str, &str, i32) -> u8>,
	pub set_mod_inherit: Box<dyn Fn(&str, &str, bool) -> u8>,
	pub set_mod_settings: Box<dyn Fn(&str, &str, &str, Vec<&str>) -> u8>,
	pub get_mod_settings: Box<dyn Fn(&str, &str, bool) -> GetModSettings>,
	pub default_collection: Box<dyn Fn() -> String>,
	pub get_collections: Box<dyn Fn() -> Vec<String>>,
}

// struct ModCache {
// 	priority: i32,
// 	
// 	files: HashMap<String, String>,
// }

pub struct Penumbra {
	funcs: PenumbraFunctions,
	mod_file_cache: HashMap<String, HashMap<(String, i32), HashMap<String, String>>>, // <collection, <(mod_id, priority), <game_path, real_path>>>
}

impl Penumbra {
	pub fn new(funcs: PenumbraFunctions) -> Self {
		Self {
			funcs,
			mod_file_cache: HashMap::new(),
		}
	}
	
	fn update_mod_cache(&mut self) {
		self.mod_file_cache.clear();
		let collections = (self.funcs.get_collections)();
		for col in &collections {
			self.mod_file_cache.insert(col.to_owned(), HashMap::new());
		}
		
		let root = (self.funcs.root_path)();
		for mod_id in (self.funcs.mod_list)() {
			let Ok(dir_entries) = std::fs::read_dir(root.join(&mod_id)) else {continue};
			let dir_entries = dir_entries.filter_map(|v| v.ok())
				.map(|v| v.file_name().to_string_lossy().into_owned())
				.collect::<Vec<_>>();
			
			let default = match read_json::<PDefaultMod>(&root.join(&mod_id).join("default_mod.json")) {
				Ok(v) => v,
				Err(e) => {log!(err, "Failed to load or parse default_mod.json for mod {mod_id}\n{e:?}"); continue},
			};
			
			let mut group_cache = HashMap::new();
			
			for col in &collections {
				let settings = (self.funcs.get_mod_settings)(col, &mod_id, false);
				if !settings.exists {continue}
				if !settings.enabled {continue}
				let options = settings.options;
				let entry = self.mod_file_cache.get_mut(col).unwrap().entry((mod_id.clone(), settings.priority)).or_insert_with(|| HashMap::new());
				
				for (g, r) in &default.Files {
					entry.insert(g.to_owned(), r.to_owned());
				}
				
				for (option, enabled_sub_options) in &options {
					if enabled_sub_options.len() == 0 {continue}
					
					let group_name = format!("{}.json", clean_path(&option));
					let Some(group) = group_cache.entry(group_name.clone()).or_insert_with(|| {
						let Some(group_name) = dir_entries.iter().find(|v| v.ends_with(&group_name)) else {log!(err, "Failed to find group file ({group_name}) for mod ({mod_id})"); return None};
						match read_json::<PGroup>(&root.join(&mod_id).join(&group_name)) {
							Ok(v) => Some(v),
							Err(e) => {log!(err, "Failed to load or parse group file {group_name} for mod {mod_id}\n{e:?}"); None},
						}
					}) else {continue};
					
					for o in &group.Options {
						if enabled_sub_options.contains(&o.Name) {
							for (g, r) in &o.Files {
								entry.insert(g.to_owned(), r.to_owned());
							}
						}
					}
				}
			}
		}
	}
	
	fn get_file<F: noumenon::File>(&self, path: &str, collection: &str, priority: i32) -> Option<F> {
		let mods = self.mod_file_cache.get(collection)?;
		let mut real_path = None;
		let mut real_path_prio = 0;
		for ((mod_id, prio), files) in mods {
			if *prio >= priority {continue}
			if *prio < real_path_prio {continue}
			if let Some(v) = files.get(path) {
				real_path = Some((mod_id, v));
				real_path_prio = *prio;
			}
		}
		
		if let Some((mod_id, path)) = real_path {
			let root = (self.funcs.root_path)();
			log!("Loading file {path} from mod {mod_id} to overlay onto");
			crate::resource_loader::load_file_disk(&root.join(&mod_id).join(path)).ok()
		} else {
			crate::noumenon()?.file::<F>(path).ok()
		}
	}
}

impl super::Backend for Penumbra {
	fn name(&self) -> &'static str {
		"Penumbra (IPC)"
	}
	
	fn description(&self) -> &'static str {
		"Penumbra mod loader using IPC"
	}
	
	fn get_mods(&self) -> Vec<String> {
		let root = (self.funcs.root_path)();
		(self.funcs.mod_list)().into_iter().filter(|v| root.join(v).join("aetherment").exists()).collect()
	}
	
	fn get_collections(&self) -> Vec<String> {
		(self.funcs.get_collections)()
	}
	
	// TODO: thread and progress bar
	fn install_mod(&mut self, file: &std::path::Path) -> Result<String, crate::resource_loader::BacktraceError> {
		let mut file = zip::ZipArchive::new(std::io::BufReader::new(std::fs::File::open(file)?))?;
		
		let mut buf = Vec::new();
		file.by_name("meta.json")?.read_to_end(&mut buf)?;
		let meta = serde_json::from_slice::<crate::modman::meta::Meta>(&buf)?;
		
		let mod_id = meta.name.clone();
		let mod_dir = (self.funcs.root_path)().join(&mod_id);
		_ = std::fs::create_dir(&mod_dir);
		let aeth_dir = mod_dir.join("aetherment");
		_ = std::fs::create_dir(&aeth_dir);
		let files_dir = mod_dir.join("files");
		_ = std::fs::create_dir(&files_dir);
		File::create(aeth_dir.join("meta.json"))?.write_all(&buf)?;
		// buf.clear();
		
		File::create(mod_dir.join("meta.json"))?.write_all(crate::json_pretty(&PMeta {
			FileVersion: 3,
			Name: &meta.name,
			Author: &meta.author,
			Description: &meta.description,
			Version: &meta.version,
			Website: &meta.website,
			ModTags: meta.tags.iter().map(|v| v.as_str()).collect(),
		})?.as_bytes())?;
		
		File::create(mod_dir.join("default_mod.json"))?.write_all(crate::json_pretty(&PDefaultMod {
			Name: String::new(),
			Description: Some(String::new()),
			Files: HashMap::new(),
			FileSwaps: HashMap::new(),
			Manipulations: Vec::new(),
		})?.as_bytes())?;
		
		std::io::copy(&mut file.by_name("remap")?, &mut File::create(aeth_dir.join("remap"))?)?;
		
		if let Ok(mut hashes) = file.by_name("hashes") {
			std::io::copy(&mut hashes, &mut File::create(aeth_dir.join("hashes"))?)?;
		}
		
		for i in 0..file.len() {
			let mut f = file.by_index(i)?;
			if f.is_dir() {continue};
			let Some(name) = f.enclosed_name() else {continue};
			let Ok(name) = name.strip_prefix("files/") else {continue};
			if name.components().count() > 1 {continue};
			let name = name.to_owned();
			// f.read_to_end(&mut buf)?;
			std::io::copy(&mut f, &mut File::create(files_dir.join(name))?)?;
		}
		
		let settings = crate::modman::settings::Settings::from_meta(&meta);
		self.apply_mod_settings(&mod_id, &(self.funcs.default_collection)(), Some(&settings))?;
		(self.funcs.add_mod_entry)(&mod_id);
		
		// TODO: possibly clean up mod files if it failed
		
		Ok(mod_id)
	}
	
	// TODO: thread and progress bar
	// TODO: update mods that composite textures that we just changed!!
	fn apply_mod_settings(&mut self, mod_id: &str, collection: &str, settings: Option<&crate::modman::settings::Settings>) -> Result<(), crate::resource_loader::BacktraceError> {
		self.update_mod_cache();
		
		let mod_dir = (self.funcs.root_path)().join(mod_id);
		let aeth_dir = mod_dir.join("aetherment");
		let files_dir = mod_dir.join("files");
		let files_comp_dir = mod_dir.join("files_comp");
		std::fs::create_dir(&files_comp_dir).ok();
		
		// TODO: cleanup unused options/collections
		
		let Some(settings) = settings else {
			(self.funcs.set_mod_inherit)(mod_id, collection, false);
			return Ok(())
		};
		
		let priority = (self.funcs.get_mod_settings)(collection, &mod_id, false).priority;
		let meta = serde_json::from_reader::<_, crate::modman::meta::Meta>(std::io::BufReader::new(File::open(aeth_dir.join("meta.json"))?))?;
		let remap = serde_json::from_reader::<_, HashMap<String, String>>(std::io::BufReader::new(File::open(aeth_dir.join("remap"))?))?;
		
		let mut p_option = POption {
			Name: collection.to_owned(),
			Description: Some(String::new()),
			Files: HashMap::new(),
			FileSwaps: HashMap::new(),
			Manipulations: Vec::new(),
		};
		
		let mut add_datas = |files: &HashMap<String, String>, swaps: &HashMap<String, String>, manips: &Vec<crate::modman::meta::Manipulation>| -> Result<(), crate::resource_loader::BacktraceError> {
			for (path, real_path) in files {
				if p_option.Files.contains_key(path) {continue}
				let Some(real_path) = remap.get(real_path) else {continue};
				let mut new_real_path = format!("files/{real_path}");
				let mut path = path.clone();
				if path.ends_with(".comp") {
					path = path.trim_end_matches(".comp").to_owned();
					match path.split(".").last().unwrap() {
						"tex" | "atex" => {
							let comp = serde_json::from_reader::<_, crate::modman::composite::tex::Tex>(std::io::BufReader::new(File::open(files_dir.join(real_path))?))?;
							let (width, height, pixels) = comp.composite(settings, |path| match path {
								crate::modman::Path::Mod(v) => {
									let Some(v) = remap.get(v) else {return None};
									crate::resource_loader::load_file_disk::<noumenon::format::game::Tex>(&files_dir.join(v)).ok().map(|v| Cow::Owned(v))
								}
								
								crate::modman::Path::Game(v) => {
									self.get_file::<noumenon::format::game::Tex>(v, collection, priority).map(|v| Cow::Owned(v))
								}
								
								crate::modman::Path::Option(id) => {
									let Some(setting) = settings.get(id) else {return None};
									let crate::modman::settings::Value::Path(i) = setting else {return None};
									let Some(option) = meta.options.iter().find(|v| v.name == *id) else {return None};
									let crate::modman::meta::OptionSettings::Path(v) = &option.settings else {return None};
									let Some((_, path)) = v.options.get(*i as usize) else {return None};
									match path {
										crate::modman::Path::Mod(v) => {
											let Some(v) = remap.get(v) else {return None};
											crate::resource_loader::load_file_disk::<noumenon::format::game::Tex>(&files_dir.join(v)).ok().map(|v| Cow::Owned(v))
										}
										
										crate::modman::Path::Game(v) => {
											self.get_file::<noumenon::format::game::Tex>(v, collection, priority).map(|v| Cow::Owned(v))
										}
										
										_ => None
									}
								}
							}).ok_or("Failed to composite texture")?;
							
							let mut data = std::io::Cursor::new(Vec::new());
							noumenon::format::game::Tex {
								header: noumenon::format::game::tex::Header {
									flags: 0x00800000,
									format: noumenon::format::game::tex::Format::A8R8G8B8,
									width: width as u16,
									height: height as u16,
									depths: 0,
									mip_levels: 1,
									lod_offsets: [0, 1, 2],
									mip_offsets: [80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
								},
								data: pixels,
							}.write(&mut data)?;
							let data = data.into_inner();
							
							let hash = crate::hash_str(blake3::hash(&data));
							File::create(files_comp_dir.join(&hash))?.write_all(&data)?;
							new_real_path = format!("files_comp/{hash}");
						}
						
						_ => {}
					}
				}
				
				p_option.Files.insert(path, new_real_path);
			}
			
			for (a, b) in swaps {
				if p_option.FileSwaps.contains_key(a) {continue}
				p_option.FileSwaps.insert(a.to_owned(), b.to_owned());
			}
			
			for manip in manips {
				match manip {
					crate::modman::meta::Manipulation::Imc {
						attribute_and_sound,
						material_id,
						decal_id,
						vfx_id,
						material_animation_id,
						attribute_mask,
						sound_id,
						
						primary_id,
						secondary_id,
						variant,
						object_type,
						equip_slot,
						body_slot,
					} => p_option.Manipulations.push(PManipulation::Imc {
						Entry: ImcEntry {
							AttributeAndSound: Some(*attribute_and_sound),
							MaterialId: Some(*material_id),
							DecalId: Some(*decal_id),
							VfxId: Some(*vfx_id),
							MaterialAnimationId: Some(*material_animation_id),
							AttributeMask: Some(*attribute_mask),
							SoundId: Some(*sound_id),
						},
						PrimaryId: *primary_id,
						SecondaryId: *secondary_id,
						Variant: *variant,
						ObjectType: object_type.to_owned(),
						EquipSlot: equip_slot.to_owned(),
						BodySlot: body_slot.to_owned(),
					}),
					
					crate::modman::meta::Manipulation::Eqdp {
						entry,
						set_id,
						slot,
						race,
						gender,
					} => p_option.Manipulations.push(PManipulation::Eqdp {
						Entry: *entry,
						SetId: *set_id,
						Slot: slot.to_owned(),
						Race: race.to_owned(),
						Gender: gender.to_owned(),
					}),
					
					crate::modman::meta::Manipulation::Eqp {
						entry,
						set_id,
						slot,
					} => p_option.Manipulations.push(PManipulation::Eqp {
						Entry: *entry,
						SetId: *set_id,
						Slot: slot.to_owned(),
					}),
					
					crate::modman::meta::Manipulation::Est {
						entry,
						set_id,
						slot,
						race,
						gender,
					} => p_option.Manipulations.push(PManipulation::Est {
						Entry: *entry,
						SetId: *set_id,
						Slot: slot.to_owned(),
						Race: race.to_owned(),
						Gender: gender.to_owned(),
					}),
					
					crate::modman::meta::Manipulation::Gmp {
						enabled,
						animated,
						rotation_a,
						rotation_b,
						rotation_c,
						unknown_a,
						unknown_b,
						unknown_total,
						value,
						
						set_id,
					} => p_option.Manipulations.push(PManipulation::Gmp {
						Entry: GmpEntry {
							Enabled: Some(*enabled),
							Animated: Some(*animated),
							RotationA: Some(*rotation_a),
							RotationB: Some(*rotation_b),
							RotationC: Some(*rotation_c),
							UnknownA: Some(*unknown_a),
							UnknownB: Some(*unknown_b),
							UnknownTotal: Some(*unknown_total),
							Value: Some(*value),
						},
						SetId: *set_id,
					}),
					
					crate::modman::meta::Manipulation::Rsp {
						entry,
						sub_race,
						attribute,
					} => p_option.Manipulations.push(PManipulation::Rsp {
						Entry: *entry,
						SubRace: sub_race.to_owned(),
						Attribute: attribute.to_owned(),
					})
				}
			}
			
			Ok(())
		};
		
		for option in meta.options.iter().rev() {
			let Some(value) = settings.get(&option.name) else {continue};
			
			match &option.settings {
				crate::modman::meta::OptionSettings::SingleFiles(v) => {
					let crate::modman::settings::Value::SingleFiles(value) = value else {continue};
					for (i, o) in v.options.iter().enumerate() {
						if value & (1 << i) != 0 {
							add_datas(&o.files, &o.file_swaps, &o.manipulations)?;
						}
					}
				}
				
				crate::modman::meta::OptionSettings::MultiFiles(v) => {
					let crate::modman::settings::Value::MultiFiles(value) = value else {continue};
					if let Some(o) = v.options.get(*value as usize) {
						add_datas(&o.files, &o.file_swaps, &o.manipulations)?;
					}
				}
				
				_ => {}
			}
		}
		
		add_datas(&meta.files, &meta.file_swaps, &meta.manipulations)?;
		
		let mut group = match read_json::<PGroup>(&mod_dir.join("group_001__collection.json")) {
		// let mut group = match File::open(mod_dir.join("group_001__collection.json")) {
			// Ok(f) => serde_json::from_reader::<_, PGroup>(std::io::BufReader::new(f))?,
			Ok(v) => v,
			Err(_) => PGroup {
				Name: "_collection".to_string(),
				Description: "Aetherment managed collection\nDON'T TOUCH THIS".to_string(),
				Priority: 1,
				Type: "Single".to_string(),
				DefaultSettings: Some(0),
				Options: Vec::new(),
			}
		};
		
		if let Some(option) = group.Options.iter_mut().find(|v| v.Name == collection) {
			*option = p_option;
		} else {
			group.Options.push(p_option);
		}
		
		File::create(mod_dir.join("group_001__collection.json"))?.write_all(crate::json_pretty(&group)?.as_bytes())?;
		
		(self.funcs.reload_mod)(mod_id);
		(self.funcs.set_mod_settings)(collection, mod_id, "_collection", vec![collection]);
		
		Ok(())
	}
	
	fn get_aeth_meta(&self, mod_id: &str) -> Option<crate::modman::meta::Meta> {
		read_json(&(self.funcs.root_path)().join(mod_id).join("aetherment").join("meta.json")).ok()
	}
	
	fn debug_renderer(&self, ui: &mut egui::Ui) {
		if ui.button("Redraw").clicked() {
			(self.funcs.redraw)();
		}
		
		if ui.button("Redraw self").clicked() {
			(self.funcs.redraw_self)();
		}
		
		if ui.button("Root path").clicked() {
			log!("Root path: {:?}", (self.funcs.root_path)());
		}
		
		if ui.button("Mod list").clicked() {
			log!("Mod list: {:?}", (self.funcs.mod_list)());
		}
	}
}

// output of c# Path.GetInvalidFileNameChars()
static INVALID_CHARS: [u8; 41] = [34, 60, 62, 124, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 58, 42, 63, 92, 47];
fn clean_path(path: &str) -> String {
	path.chars().filter_map(|v| if !INVALID_CHARS.contains(&(v as u8)) {Some(v.to_ascii_lowercase())} else {None}).collect::<String>()
}

#[derive(Debug, Deserialize, Serialize)]
struct PMeta<'a> {
	FileVersion: i32,
	Name: &'a str,
	Author: &'a str,
	Description: &'a str,
	Version: &'a str,
	Website: &'a str,
	ModTags: Vec<&'a str>,
}

#[derive(Debug, Deserialize, Serialize)]
struct PDefaultMod {
	Name: String,
	// Description: String,
	Description: Option<String>,
	Files: HashMap<String, String>,
	FileSwaps: HashMap<String, String>,
	Manipulations: Vec<PManipulation>,
}

#[derive(Debug, Deserialize, Serialize)]
struct PGroup {
	Name: String,
	Description: String,
	Priority: i32,
	Type: String,
	DefaultSettings: Option<u32>,
	Options: Vec<POption>,
}

#[derive(Debug, Deserialize, Serialize)]
struct POption {
	Name: String,
	Description: Option<String>,
	Files: HashMap<String, String>,
	FileSwaps: HashMap<String, String>,
	Manipulations: Vec<PManipulation>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "Type", content = "Manipulation")]
enum PManipulation {
	Imc {
		Entry: ImcEntry,
		PrimaryId: i32,
		SecondaryId: i32,
		Variant: i32,
		ObjectType: String,
		EquipSlot: String,
		BodySlot: String,
	},
	
	Eqdp {
		Entry: u64,
		SetId: i32,
		Slot: String,
		Race: String,
		Gender: String,
	},
	
	Eqp {
		Entry: u64,
		SetId: i32,
		Slot: String,
	},
	
	Est {
		Entry: u64,
		SetId: i32,
		Slot: String,
		Race: String,
		Gender: String,
	},
	
	Gmp {
		Entry: GmpEntry,
		SetId: i32,
	},
	
	Rsp {
		Entry: f32,
		SubRace: String,
		Attribute: String,
	},
}

// No clue if the options are needed, but during testing i had 1 group of 1 mod that didnt contain AttributeAndSound in the ImcEntry
// and i just dont want to figure out what is optional and what isnt since the penumbra meta structs(if you can even call it that) arent cleanly laid out
#[derive(Debug, Deserialize, Serialize)]
struct ImcEntry {
	AttributeAndSound: Option<i32>,
	MaterialId: Option<i32>,
	DecalId: Option<i32>,
	VfxId: Option<i32>,
	MaterialAnimationId: Option<i32>,
	AttributeMask: Option<i32>,
	SoundId: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
struct GmpEntry {
	Enabled: Option<bool>,
	Animated: Option<bool>,
	RotationA: Option<i32>,
	RotationB: Option<i32>,
	RotationC: Option<i32>,
	UnknownA: Option<i32>,
	UnknownB: Option<i32>,
	UnknownTotal: Option<i32>,
	Value: Option<u64>,
}