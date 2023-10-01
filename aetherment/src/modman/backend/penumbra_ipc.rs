use std::{io::{Read, Write}, fs::File, collections::HashMap, borrow::Cow};
use serde::{Deserialize, Serialize};

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
	pub default_collection: Box<dyn Fn() -> String>,
}

pub struct Penumbra {
	funcs: PenumbraFunctions,
}

impl Penumbra {
	pub fn new(funcs: PenumbraFunctions) -> Self {
		Self {
			funcs
		}
	}
	
	fn get_file<F>(&self, path: &str, _priority: i32) -> Option<F> where
	F: noumenon::File {
		// TODO: support files from mods!!! gonna be suffering but its important
		crate::noumenon()?.file::<F>(path).ok()
	}
}

impl super::Backend for Penumbra {
	fn name(&self) -> &'static str {
		"Penumbra (IPC)"
	}
	
	fn description(&self) -> &'static str {
		"Penumbra mod loader using IPC"
	}
	
	// TODO: thread and progress bar
	fn install_mod(&self, file: &std::path::Path) -> Result<String, crate::resource_loader::BacktraceError> {
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
			Description: String::new(),
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
	
	fn get_all_mods(&self) -> Vec<String> {
		(self.funcs.mod_list)()
	}
	
	// TODO: thread and progress bar
	fn apply_mod_settings(&self, mod_id: &str, collection: &str, settings: Option<&crate::modman::settings::Settings>) -> Result<(), crate::resource_loader::BacktraceError> {
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
		
		let priority = 0; // TODO: GetAvailableModSettings
		let meta = serde_json::from_reader::<_, crate::modman::meta::Meta>(std::io::BufReader::new(File::open(aeth_dir.join("meta.json"))?))?;
		let remap = serde_json::from_reader::<_, HashMap<String, String>>(std::io::BufReader::new(File::open(aeth_dir.join("remap"))?))?;
		
		let mut p_option = POption {
			Name: collection.to_owned(),
			Description: String::new(),
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
									self.get_file::<noumenon::format::game::Tex>(v, priority).map(|v| Cow::Owned(v))
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
											self.get_file::<noumenon::format::game::Tex>(v, priority).map(|v| Cow::Owned(v))
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
							AttributeAndSound: *attribute_and_sound,
							MaterialId: *material_id,
							DecalId: *decal_id,
							VfxId: *vfx_id,
							MaterialAnimationId: *material_animation_id,
							AttributeMask: *attribute_mask,
							SoundId: *sound_id,
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
							Enabled: *enabled,
							Animated: *animated,
							RotationA: *rotation_a,
							RotationB: *rotation_b,
							RotationC: *rotation_c,
							UnknownA: *unknown_a,
							UnknownB: *unknown_b,
							UnknownTotal: *unknown_total,
							Value: *value,
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
		
		let mut group = match File::open(mod_dir.join("group_001_collection.json")) {
			Ok(f) => serde_json::from_reader::<_, PGroup>(std::io::BufReader::new(f))?,
			Err(_) => PGroup {
				Name: "_collection".to_string(),
				Description: "Aetherment managed collection\nDON'T TOUCH THIS".to_string(),
				Priority: 1,
				Type: "Single".to_string(),
				DefaultSettings: 0,
				Options: Vec::new(),
			}
		};
		
		if let Some(option) = group.Options.iter_mut().find(|v| v.Name == collection) {
			*option = p_option;
		} else {
			group.Options.push(p_option);
		}
		
		File::create(mod_dir.join("group_001_collection.json"))?.write_all(crate::json_pretty(&group)?.as_bytes())?;
		
		(self.funcs.reload_mod)(mod_id);
		(self.funcs.set_mod_settings)(collection, mod_id, "_collection", vec![collection]);
		
		Ok(())
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
	Description: String,
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
	DefaultSettings: u32,
	Options: Vec<POption>,
}

#[derive(Debug, Deserialize, Serialize)]
struct POption {
	Name: String,
	Description: String,
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

#[derive(Debug, Deserialize, Serialize)]
struct ImcEntry {
	AttributeAndSound: i32,
	MaterialId: i32,
	DecalId: i32,
	VfxId: i32,
	MaterialAnimationId: i32,
	AttributeMask: i32,
	SoundId: i32,
}

#[derive(Debug, Deserialize, Serialize)]
struct GmpEntry {
	Enabled: bool,
	Animated: bool,
	RotationA: i32,
	RotationB: i32,
	RotationC: i32,
	UnknownA: i32,
	UnknownB: i32,
	UnknownTotal: i32,
	Value: u64,
}