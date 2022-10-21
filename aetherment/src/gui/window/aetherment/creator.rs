use std::{path::PathBuf, fs::File, sync::{Arc, Mutex}, io::{Write, Cursor}, collections::HashMap, thread};
use binrw::{BinReaderExt, BinWrite};
use imgui::aeth::{Texture, TextureOptions, DrawList};
use serde::{Deserialize, Serialize, Deserializer, Serializer, ser::SerializeTuple};
use serde_json::json;
use crate::{gui::aeth::{self, F2}, CLIENT, SERVER, creator::modpack, SERVERCDN};

const MAX_NAME_LEN: usize = 64;
const MAX_DESC_LEN: usize = 5000;
const MAX_CONTRIBUTORS: usize = 8;
const MAX_DEPENDENCIES: usize = 8;
const PREVIEW_RESOLUTION: [u32; 2] = [1620, 1080];

const CONTRIBUTOR_IMG: TextureOptions = TextureOptions {
	width: 32,
	height: 32,
	format: 28, // DXGI_FORMAT_R8G8B8A8_UNORM
	usage: 1, // D3D11_USAGE_IMMUTABLE
	cpu_access_flags: 0,
};

const DEPENDENCY_IMG: TextureOptions = TextureOptions {
	width: 45,
	height: 30,
	format: 28, // DXGI_FORMAT_R8G8B8A8_UNORM
	usage: 1, // D3D11_USAGE_IMMUTABLE
	cpu_access_flags: 0,
};

#[derive(Debug)]
struct ContributorTexture(Texture, Vec<u8>);

impl std::ops::Deref for ContributorTexture {
	type Target = Texture;
	
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<'de> Deserialize<'de> for ContributorTexture {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
	D: serde::Deserializer<'de> {
		let v: String = Deserialize::deserialize(deserializer)?;
		let data = base64::decode(v).unwrap();
		Ok(if data.len() == 0 {
			ContributorTexture(Texture::empty(), data)
		} else {
			ContributorTexture(Texture::with_data(CONTRIBUTOR_IMG, &data), data)
		})
	}
}

impl Serialize for ContributorTexture {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
	S: Serializer {
		serializer.serialize_str(&base64::encode(&self.1))
	}
}

#[derive(Debug)]
struct DependencyTexture(Texture, Vec<u8>);

impl std::ops::Deref for DependencyTexture {
	type Target = Texture;
	
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<'de> Deserialize<'de> for DependencyTexture {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
	D: serde::Deserializer<'de> {
		let v: String = Deserialize::deserialize(deserializer)?;
		let data = base64::decode(v).unwrap();
		Ok(if data.len() == 0 {
			DependencyTexture(Texture::empty(), data)
		} else {
			DependencyTexture(Texture::with_data(DEPENDENCY_IMG, &data), data)
		})
	}
}

impl Serialize for DependencyTexture {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
	S: Serializer {
		serializer.serialize_str(&base64::encode(&self.1))
	}
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

#[derive(Deserialize, Serialize, Debug, Default)]
struct Meta {
	name: String,
	description: String,
	contributors: Vec<(i32, String, ContributorTexture)>,
	dependencies: Vec<(i32, String, String, DependencyTexture)>,
	nsfw: bool,
	previews: Vec<String>,
}

struct CurMod {
	meta: Meta,
	online: Option<OnlineMod>,
	tags: HashMap<String, Vec<String>>,
	version: [i32; 4],
	contributors: UserSearch,
	dependencies: ModSearch,
	previews: HashMap<String, Texture>,
	importing_preview: bool,
	path: PathBuf,
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
			let mut save = false;
			
			save |= imgui::input_text("Name", &mut m.meta.name, imgui::InputTextFlags::None);
			let limit = m.meta.name.len() >= MAX_NAME_LEN;
			if limit {imgui::push_style_color(imgui::Col::Text, 0xFF3030B0)}
			imgui::text(&format!("{}/{}", m.meta.name.len(), MAX_NAME_LEN));
			if limit {imgui::pop_style_color(1)}
			
			aeth::offset([0.0, 16.0]);
			save |= imgui::input_text_multiline("Description", &mut m.meta.description, [0.0, 400.0], imgui::InputTextFlags::None);
			let limit = m.meta.name.len() >= MAX_DESC_LEN;
			if limit {imgui::push_style_color(imgui::Col::Text, 0xFF3030B0)}
			imgui::text(&format!("{}/{}", m.meta.description.len(), MAX_DESC_LEN));
			if limit {imgui::pop_style_color(1)}
			
			aeth::offset([0.0, 16.0]);
			imgui::text("Contributors");
			let h = imgui::get_font_size() * 2.0;
			let rounding = imgui::get_style().frame_rounding;
			let col = imgui::get_color(imgui::Col::Text);
			let colframe = imgui::get_color(imgui::Col::FrameBg);
			let mut draw = imgui::get_window_draw_list();
			
			let mut rem = None;
			save |= aeth::orderable_list2("##contributors", [300.0, h], &mut m.meta.contributors, |i, _| {
				if imgui::button("Remove", [0.0, 0.0]) {
					rem = Some(i);
					save = true;
				}
			}, |_, entry| {
				let pos = imgui::get_cursor_screen_pos();
				let size = [h; 2];
				let w = imgui::get_column_width(-1);
				imgui::dummy([w, h]);
				draw.add_rect_filled(pos, pos.add([w, h]), colframe, rounding, imgui::DrawFlags::RoundCornersAll);
				draw.push_texture_id(entry.2.resource());
				draw.add_rect_rounded(pos, pos.add(size), [0.0; 2], [1.0; 2], 0xFFFFFFFF, rounding);
				draw.pop_texture_id();
				draw.add_text(pos.add([h + 2.0, h * 0.25]), col, &entry.1);
			});
			if let Some(i) = rem {m.meta.contributors.remove(i);}
			
			if m.meta.contributors.len() < MAX_CONTRIBUTORS {
				if imgui::input_text_with_hint("##contributors_search", "Search User", &mut m.contributors.query(), imgui::InputTextFlags::None) {m.contributors.search()}
				let mut clear = false;
				for entry in m.contributors.list().iter() {
					if m.meta.contributors.iter().any(|v| v.0 == entry.id) {continue}
					
					let pos = imgui::get_cursor_screen_pos();
					let size = [h; 2];
					imgui::dummy([300.0, h]);
					if imgui::is_item_clicked(imgui::MouseButton::Left) {
						let data = entry.avatar.lock().unwrap().1.clone();
						m.meta.contributors.push((
							entry.id,
							entry.name.clone(),
							if data.len() == 0 {
								ContributorTexture(Texture::empty(), data)
							} else {
								ContributorTexture(Texture::with_data(CONTRIBUTOR_IMG, &data), data)
							},
						));
						save = true;
						clear = true;
					}
					draw.add_rect_filled(pos, pos.add([300.0, h]), colframe, rounding, imgui::DrawFlags::RoundCornersAll);
					draw.push_texture_id(entry.avatar.lock().unwrap().0.resource());
					draw.add_rect_rounded(pos, pos.add(size), [0.0; 2], [1.0; 2], 0xFFFFFFFF, rounding);
					draw.pop_texture_id();
					draw.add_text(pos.add([h + 2.0, h * 0.25]), col, &entry.name);
				}
				
				if clear {
					m.contributors.query().clear();
					m.contributors.list().clear();
				}
			}
			
			aeth::offset([0.0, 16.0]);
			imgui::text("Dependencies");
			let size = [h / 2.0 * 3.0, h];
			let mut rem = None;
			save |= aeth::orderable_list2("##dependencies", [500.0, h], &mut m.meta.dependencies, |i, _| {
				if imgui::button("Remove", [0.0, 0.0]) {
					rem = Some(i);
					save = true;
				}
			}, |_, entry| {
				let pos = imgui::get_cursor_screen_pos();
				let w = imgui::get_column_width(-1);
				imgui::dummy([w, h]);
				draw.add_rect_filled(pos, pos.add([w, h]), colframe, rounding, imgui::DrawFlags::RoundCornersAll);
				draw.push_texture_id(entry.3.resource());
				draw.add_rect_rounded(pos, pos.add(size), [0.0; 2], [1.0; 2], 0xFFFFFFFF, rounding);
				draw.pop_texture_id();
				draw.add_text(pos.add([size.x() + 2.0, 0.0]), col, &entry.1);
				draw.add_text(pos.add([size.x() + h * 0.5 + 2.0, h * 0.5]), col, &format!("by {}", entry.2));
			});
			if let Some(i) = rem {m.meta.dependencies.remove(i);}
			
			if m.meta.dependencies.len() < MAX_DEPENDENCIES {
				if imgui::input_text_with_hint("##dependencies_search", "Search Mod", &mut m.dependencies.query(), imgui::InputTextFlags::None) {m.dependencies.search()}
				let mut clear = false;
				for entry in m.dependencies.list().iter() {
					if m.meta.dependencies.iter().any(|v| v.0 == entry.id) {continue}
					
					let pos = imgui::get_cursor_screen_pos();
					imgui::dummy([500.0, h]);
					if imgui::is_item_clicked(imgui::MouseButton::Left) {
						let data = entry.thumbnail.lock().unwrap().1.clone();
						m.meta.dependencies.push((
							entry.id,
							entry.name.clone(),
							entry.author.clone(),
							if data.len() == 0 {
								DependencyTexture(Texture::empty(), data)
							} else {
								DependencyTexture(Texture::with_data(DEPENDENCY_IMG, &data), data)
							},
						));
						save = true;
						clear = true;
					}
					draw.add_rect_filled(pos, pos.add([500.0, h]), colframe, rounding, imgui::DrawFlags::RoundCornersAll);
					draw.push_texture_id(entry.thumbnail.lock().unwrap().0.resource());
					draw.add_rect_rounded(pos, pos.add(size), [0.0; 2], [1.0; 2], 0xFFFFFFFF, rounding);
					draw.pop_texture_id();
					draw.add_text(pos.add([size.x() + 2.0, 0.0]), col, &entry.name);
					draw.add_text(pos.add([size.x() + h * 0.5 + 2.0, h * 0.5]), col, &format!("by {}", entry.author));
				}
				
				if clear {
					m.dependencies.query().clear();
					m.dependencies.list().clear();
				}
			}
			
			aeth::offset([0.0, 16.0]);
			save |= imgui::checkbox("NSFW", &mut m.meta.nsfw);
			
			aeth::offset([0.0, 16.0]);
			imgui::text("Previews (1620x1080 for best results, anything else will be resized)");
			// imgui::begin_child("previews", [0.0, 400.0], false, imgui::WindowFlags::None);
			aeth::child("previews", [0.0, 400.0], false, imgui::WindowFlags::AlwaysHorizontalScrollbar, || {
				let h = imgui::get_content_region_avail().y();
				let w = h / 2.0 * 3.0;
				for id in &m.meta.previews {
					let preview = &m.previews[id];
					imgui::image(preview.resource(), [w, h], [0.0; 2], [1.0; 2], [1.0; 4], [0.0; 4]);
					imgui::same_line();
				}
				
				if imgui::button("+", [w, h]) {m.importing_preview = true}
				if m.importing_preview {
					match aeth::file_dialog(aeth::FileDialogMode::OpenFile, "Importing Preview".to_owned(), "".to_owned(), vec![".png".to_owned(), ".jpg".to_owned(), ".jpeg".to_owned()]) {
						aeth::FileDialogResult::Success(path) => {
							use image::imageops::FilterType;
							m.importing_preview = false;
							let preview = image::io::Reader::new(std::io::BufReader::new(File::open(path).unwrap()))
								.with_guessed_format()
								.unwrap()
								.decode()
								.unwrap();
							
							let w = preview.width();
							let h = preview.height();
							let tw = PREVIEW_RESOLUTION[0];
							let th = PREVIEW_RESOLUTION[1];
							
							let img = if w == tw && h == th {
								preview
							} else if (w as f32 / 3.0) / (h as f32 / 2.0) == 1.0 {
								preview.resize_exact(tw, th, FilterType::Triangle)
							} else {
								let mut img = preview.resize_to_fill(tw, th, FilterType::Triangle)
									.blur(32.0);
								
								let scale = (th as f32 / h as f32).min(tw as f32 / w as f32);
								let x = ((tw as f32 - w as f32 * scale) / 2.0) as i64;
								let y = ((th as f32 - h as f32 * scale) / 2.0) as i64;
								let w = (w as f32 * scale) as u32;
								let h = (h as f32 * scale) as u32;
								let preview = if scale == 1.0 {preview} else {preview.resize_exact(w, h, FilterType::Triangle)};
								image::imageops::overlay(&mut img, &preview, x, y);
								
								img
							};
							
							let mut data = Vec::new();
							img.write_to(&mut Cursor::new(&mut data), image::ImageFormat::Jpeg).unwrap();
							let hash = crate::hash_str(blake3::hash(&data).as_bytes());
							
							let previews_dir = m.path.join("previews");
							std::fs::create_dir_all(&previews_dir).unwrap();
							File::create(previews_dir.join(&hash)).unwrap().write_all(&data).unwrap();
							
							m.meta.previews.push(hash.clone());
							m.previews.insert(hash, Texture::with_data(TextureOptions {
								width: img.width() as i32,
								height: img.height() as i32,
								format: 28, // DXGI_FORMAT_R8G8B8A8_UNORM
								usage: 1, // D3D11_USAGE_IMMUTABLE
								cpu_access_flags: 0,
							}, &img.into_rgba8()));
							
							save = true;
						},
						aeth::FileDialogResult::Failed => m.importing_preview = false,
						aeth::FileDialogResult::Busy => {},
					}
				}
			});
			// imgui::end_child();
			
			// aeth::orderable_list2("##previews", [500.0, 180.0], &mut vec![1, 2, 3, 4, 5, 6], |i, _| {
			// 	
			// }, |_, entry| {
			// 	imgui::checkbox("##thumbnail", &mut false);
			// 	let pos = imgui::get_cursor_screen_pos();
			// 	draw.add_rect_filled(pos, pos.add([270.0, 180.0]), 0xFF0000FF, 0.0, imgui::DrawFlags::None);
			// 	imgui::text(&format!("{entry}"));
			// });
			
			aeth::offset([0.0, 16.0]);
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
				
				save = true;
			}
			
			aeth::offset([0.0, 16.0]);
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
			
			if save {
				File::create(m.path.join("meta.json")).unwrap().write_all(crate::serialize_json(json!(m.meta)).as_bytes()).unwrap();
			}
			
			// let version = (m.version[0] << 24) + (m.version[1] << 16) + (m.version[2] << 8) + m.version[3];
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
			// 		
			// 		// let pack;
			// 		let mod_path = PathBuf::from(&state.config.local_path).join(&self.selected_mod);
			// 		let releases_path = mod_path.join("releases");
			// 		if let Some(online) = &m.online {
			// 			if !releases_path.exists() {
			// 				// TODO: popup
			// 				log!(err, "Local copy out of sync from remote");
			// 				return
			// 			}
			// 			
			// 			req = req.header("Mod-Id", online.id);
			// 			
			// 			let version_str = modpack::version_to_string(online.version);
			// 			let latest = match std::fs::read_dir(&releases_path)
			// 				.unwrap()
			// 				.into_iter()
			// 				.find(|e| {
			// 					let name = e.as_ref().unwrap().file_name();
			// 					let name = name.to_str().unwrap();
			// 					
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
			// 		
			// 		req = req.header("Content-Length", pack.metadata().unwrap().len().to_string());
			// 		let refresh = self.refresh.clone();
			// 		
			// 		std::thread::spawn(move || {
			// 			let resp = req
			// 				.body(File::open(pack).unwrap())
			// 				.send()
			// 				.unwrap()
			// 				.text()
			// 				.unwrap();
			// 			
			// 			log!("upload resp: {resp}");
			// 			
			// 			let resp: serde_json::Value = serde_json::from_str(&resp).unwrap();
			// 			
			// 			if let Some(err) = resp["error"].as_str() {
			// 				log!(err, "{err}");
			// 				return
			// 			}
			// 			
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
			// 
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
			
			match CLIENT.get(format!("{}/api/user/stats", SERVER))
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
		
		m.name.reserve(MAX_NAME_LEN * 4); // *4 cuz unicode
		m.description.reserve(MAX_DESC_LEN * 4);
		
		let name = path.file_name().unwrap().to_str().unwrap();
		let online = self.mod_entries.iter().find_map(|m| if m.0 == name && m.1.is_some() {Some(m.1.as_ref().unwrap().clone())} else {None}).clone();
		
		log!("{:?}", online);
		self.curmod = Some(CurMod {
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
			contributors: UserSearch::new(),
			dependencies: ModSearch::new(),
			previews: m.previews.iter().map(|v| {
				// let img = image::open(path.join("previews").join(v)).unwrap().into_rgb8();
				let mut img = image::io::Reader::new(std::io::BufReader::new(File::open(path.join("previews").join(v)).unwrap()));
				img.set_format(image::ImageFormat::Jpeg);
				let img = img.decode().unwrap();
				
				(
					v.to_owned(),
					Texture::with_data(TextureOptions {
						width: img.width() as i32,
						height: img.height() as i32,
						format: 28, // DXGI_FORMAT_R8G8B8A8_UNORM
						usage: 1, // D3D11_USAGE_IMMUTABLE
						cpu_access_flags: 0,
					}, &img.into_rgba8())
				)
			}).collect(),
			importing_preview: false,
			path: path,
			meta: m,
		});
	}
}

// played around with traits and the generic_associated_types feature, but its simply not worth it for something this small
// thats why nearly the same thing exists twice
struct SearchedUser {
	id: i32,
	name: String,
	avatar: Arc<Mutex<(Texture, Vec<u8>)>>,
}

struct UserSearch {
	next: Arc<Mutex<String>>,
	fetching: Arc<Mutex<bool>>,
	list: Arc<Mutex<Vec<SearchedUser>>>,
}

impl UserSearch {
	pub fn new() -> Self {
		Self {
			next: Arc::new(Mutex::new(String::with_capacity(64))),
			fetching: Arc::new(Mutex::new(false)),
			list: Arc::new(Mutex::new(Vec::new())),
		}
	}
	
	pub fn list(&self) -> std::sync::MutexGuard<Vec<SearchedUser>> {
		self.list.lock().unwrap()
	}
	
	pub fn query(&self) -> std::sync::MutexGuard<String> {
		self.next.lock().unwrap()
	}
	
	pub fn search(&self) {
		if *self.fetching.lock().unwrap() {return}
		
		let next = self.next.clone();
		let fetching = self.fetching.clone();
		let list = self.list.clone();
		
		thread::spawn(move || {
			loop {
				let search = next.lock().unwrap().clone();
				if search.len() == 0 {
					list.lock().unwrap().clear();
					*fetching.lock().unwrap() = false;
					break;
				}
				
				#[derive(Deserialize)]
				struct JUser {
					id: i32,
					name: String,
					avatar: String,
				}
				
				let new: Vec<SearchedUser> = match CLIENT.get(format!("{}/api/searchuser/{}", SERVER, search)).send() {
					Ok(v) => match v.json::<Vec<JUser>>() {
						Ok(new) => new.into_iter()
							.map(|v| {
								let avatar_url = v.avatar;
								let v = SearchedUser {
									avatar: Arc::new(Mutex::new((Texture::empty(), Vec::new()))),
									id: v.id,
									name: v.name,
								};
								let avatar = v.avatar.clone();
								
								thread::spawn(move || {
									let data = image::io::Reader::new(Cursor::new(CLIENT.get(format!("{SERVERCDN}{avatar_url}")).send().unwrap()
										.bytes()
										.unwrap()
										.to_vec()))
										.with_guessed_format()
										.unwrap()
										.decode()
										.unwrap()
										.resize_exact(32, 32, image::imageops::FilterType::Triangle)
										.into_rgba8();
									
									*avatar.lock().unwrap() = (Texture::with_data(CONTRIBUTOR_IMG, &data), data.to_vec());
								});
								
								v
							}).collect(),
						Err(_) => {
							log!("json decode failed");
							*fetching.lock().unwrap() = false;
							break;
						}
					},
					Err(_) => {
							log!("fetch failed");
						*fetching.lock().unwrap() = false;
						break;
					}
				};
				
				*list.lock().unwrap() = new;
				
				if &search == next.lock().unwrap().as_str() {
					*fetching.lock().unwrap() = false;
					break;
				}
			}
		});
	}
}

// ----------
struct SearchedMod {
	id: i32,
	name: String,
	author: String,
	thumbnail: Arc<Mutex<(Texture, Vec<u8>)>>,
}

struct ModSearch {
	next: Arc<Mutex<String>>,
	fetching: Arc<Mutex<bool>>,
	list: Arc<Mutex<Vec<SearchedMod>>>,
}

impl ModSearch {
	pub fn new() -> Self {
		Self {
			next: Arc::new(Mutex::new(String::with_capacity(64))),
			fetching: Arc::new(Mutex::new(false)),
			list: Arc::new(Mutex::new(Vec::new())),
		}
	}
	
	pub fn list(&self) -> std::sync::MutexGuard<Vec<SearchedMod>> {
		self.list.lock().unwrap()
	}
	
	pub fn query(&self) -> std::sync::MutexGuard<String> {
		self.next.lock().unwrap()
	}
	
	pub fn search(&self) {
		if *self.fetching.lock().unwrap() {return}
		
		let next = self.next.clone();
		let fetching = self.fetching.clone();
		let list = self.list.clone();
		
		thread::spawn(move || {
			loop {
				let search = next.lock().unwrap().clone();
				if search.len() == 0 {
					list.lock().unwrap().clear();
					*fetching.lock().unwrap() = false;
					break;
				}
				
				#[derive(Deserialize)]
				struct JMod {
					id: i32,
					name: String,
					author_name: String,
					thumbnail: String,
				}
				
				let new: Vec<SearchedMod> = match CLIENT.get(format!("{}/api/search", SERVER)).query(&[("query", &search)]).send() {
					Ok(v) => match v.json::<Vec<JMod>>() {
						Ok(new) => new.into_iter()
							.map(|v| {
								let thumbnail_url = v.thumbnail;
								let v = SearchedMod {
									thumbnail: Arc::new(Mutex::new((Texture::empty(), Vec::new()))),
									id: v.id,
									name: v.name,
									author: v.author_name,
								};
								let thumbnail = v.thumbnail.clone();
								
								thread::spawn(move || {
									let data = image::io::Reader::new(Cursor::new(CLIENT.get(format!("{SERVERCDN}{thumbnail_url}")).send().unwrap()
										.bytes()
										.unwrap()
										.to_vec()))
										.with_guessed_format()
										.unwrap()
										.decode()
										.unwrap()
										.resize_exact(135, 90, image::imageops::FilterType::Triangle)
										.into_rgba8();
									
									*thumbnail.lock().unwrap() = (Texture::with_data(DEPENDENCY_IMG, &data), data.to_vec());
								});
								
								v
							}).collect(),
						Err(_) => {
							*fetching.lock().unwrap() = false;
							break;
						}
					},
					Err(_) => {
						*fetching.lock().unwrap() = false;
						break;
					}
				};
				
				*list.lock().unwrap() = new;
				
				if &search == next.lock().unwrap().as_str() {
					*fetching.lock().unwrap() = false;
					break;
				}
			}
		});
	}
}