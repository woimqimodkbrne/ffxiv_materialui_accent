use std::{path::{PathBuf, Path}, fs::File, sync::{Arc, Mutex}, io::{Write, Cursor}, collections::HashMap, thread, time::SystemTime};
use binrw::BinReaderExt;
use imgui::aeth::{Texture, TextureOptions, DrawList};
use serde::Deserialize;
use serde_json::json;
use crate::{gui::aeth::{self, F2}, creator::{meta::*, modpack}, CLIENT, SERVER};

#[derive(Deserialize, Clone, Debug)]
struct OnlineMod {
	id: i32,
	name: String,
	description: String,
	tags: Vec<i16>,
	previews: Vec<String>,
	content_rating: i32,
	version: i32,
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
	packing: (String, Option<Arc<Mutex<(usize, usize)>>>), // (patch_path, (progress, total))
	path: PathBuf,
	uploading: (u8, String, PathBuf),
	save: Option<SystemTime>,
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
		// wanted to do a local modal popup, couldnt figure out how to disable inputs
		if let Some(m) = &mut self.curmod && let Some(progress) = &mut m.packing.1 {
			let (done, total) = *progress.lock().unwrap();
			imgui::text(&format!("Packing {} (v{}.{}.{}.{})", m.meta.name, m.version[0], m.version[1], m.version[2], m.version[3]));
			imgui::progress_bar(done as f32 / total as f32, [300.0, 20.0], &format!("{done}/{total}"));
			
			if done == total && imgui::button("close", [0.0; 2]) {
				m.packing.1 = None;
			}
			
			return;
		}
		
		if *self.refresh.lock().unwrap() {
			*self.refresh.lock().unwrap() = false;
			self.load_mods(state);
			self.load_mod(PathBuf::from(&state.config.local_path).join(&self.selected_mod), state);
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
						self.load_mod(PathBuf::from(&state.config.local_path).join(name), state);
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
				self.load_mod(path, state);
			}
			aeth::tooltip("Create Mod");
			
			imgui::same_line();
			aeth::next_max_width();
			imgui::input_text_with_hint("##newmod", "New Mod", &mut self.newmod, imgui::InputTextFlags::None);
			
			// Mod Importing
			if self.importing {
				match aeth::file_dialog("Importing Mod", || -> aeth::FileDialog {
					aeth::FileDialog::new(dirs::document_dir().unwrap().to_string_lossy(), "")
						.allow_directories(true)
						.add_extension(".amp", Some("Aetherment"))
						.add_extension(".amp.patch", Some("Aetherment"))
						.add_extension(".pap", Some("Penumbra"))
						.add_extension(".ttmp", Some("TexTools"))
						.add_extension(".ttmp2", Some("TexTools"))
						.finish()
				}) {
					aeth::FileDialogResult::Success(paths) => {
						self.importing = false;
						let path = &paths[0];
						if path.is_dir() {
							if path.join("meta.json").exists() && path.join("default_mod.json").exists() {
								crate::creator::import::penumbra::import(&path, PathBuf::from(&state.config.local_path).join(path.file_name().unwrap())).unwrap();
							} else if path.join("options.json").exists() && path.join("elements_black").exists() {
								crate::creator::import::v1::import(&path, PathBuf::from(&state.config.local_path).join(path.file_name().unwrap())).unwrap();
							} else {
								aeth::show_error("Mod Import Failed", format!("{path:?} Is not a valid penumbra directory."));
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
					aeth::FileDialogResult::Canceled => self.importing = false,
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
				draw.add_rect_rounded(pos, pos.add(size), [0.0; 2], [1.0; 2], 0xFFFFFFFF, rounding, imgui::DrawFlags::RoundCornersAll);
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
					draw.add_rect_rounded(pos, pos.add(size), [0.0; 2], [1.0; 2], 0xFFFFFFFF, rounding, imgui::DrawFlags::RoundCornersAll);
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
				draw.add_rect_rounded(pos, pos.add(size), [0.0; 2], [1.0; 2], 0xFFFFFFFF, rounding, imgui::DrawFlags::RoundCornersAll);
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
					draw.add_rect_rounded(pos, pos.add(size), [0.0; 2], [1.0; 2], 0xFFFFFFFF, rounding, imgui::DrawFlags::RoundCornersAll);
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
			// save |= imgui::checkbox("NSFW", &mut m.meta.nsfw);
			imgui::text("Content Rating");
			save |= imgui::radio_button("SFW", &mut m.meta.content_rating, 0);
			imgui::same_line();
			save |= imgui::radio_button("NSFW", &mut m.meta.content_rating, 1);
			imgui::same_line();
			save |= imgui::radio_button("NSFL", &mut m.meta.content_rating, 2);
			
			aeth::offset([0.0, 16.0]);
			imgui::text("Previews (1620x1080 for best results, anything else will be resized)");
			aeth::child("previews", [0.0, 400.0], false, imgui::WindowFlags::AlwaysHorizontalScrollbar, || {
				let h = imgui::get_content_region_avail().y();
				let w = h / 2.0 * 3.0;
				for id in &m.meta.previews {
					let preview = &m.previews[id];
					imgui::image(preview.resource(), [w, h], [0.0; 2], [1.0; 2], [1.0; 4], [0.0; 4]);
					if imgui::is_item_clicked(imgui::MouseButton::Right) {imgui::open_popup(id, imgui::PopupFlags::MouseButtonRight)}
					if imgui::begin_popup(id, imgui::WindowFlags::None) {
						if imgui::button("Remove", [0.0; 2]) {
							_ = std::fs::remove_file(m.path.join("previews").join(id));
							if let Some(i) = m.meta.previews.iter().position(|v| v == id) {m.meta.previews.remove(i);}
							save = true;
							break;
						}
						imgui::end_popup();
					}
					imgui::same_line();
				}
				
				if imgui::button("+", [w, h]) {m.importing_preview = true}
				if m.importing_preview {
					match aeth::file_dialog("Importing Preview", || -> aeth::FileDialog {
						aeth::FileDialog::new(dirs::document_dir().unwrap().to_string_lossy(), "")
							.limit(0)
							.add_extension(".png", Some("PNG"))
							.add_extension(".jpg", Some("JPEG"))
							.add_extension(".jpeg", Some("JPEG"))
							.finish()
					}) {
						aeth::FileDialogResult::Success(paths) => {
							use image::imageops::FilterType;
							m.importing_preview = false;
							
							for path in &paths {
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
							}
							
							save = true;
						},
						aeth::FileDialogResult::Canceled => m.importing_preview = false,
						aeth::FileDialogResult::Busy => {},
					}
				}
			});
			
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
			if save {m.save = Some(SystemTime::now())}
			
			if let Some(save) = m.save && SystemTime::now().duration_since(save).unwrap().as_secs() > 5 {
				save_mod(m, &state);
				m.save = None;
			}
			
			aeth::offset([0.0, 16.0]);
			imgui::text("Create Modpack");
			imgui::set_next_item_width(400.0);
			aeth::tab_bar("modpack tabs")
				.tab("Standalone", || {
					if imgui::button("Create", [0.0; 2]) {
						let progress = Arc::new(Mutex::new((0, 0)));
						m.packing.1 = Some(progress.clone());
						
						let path = m.path.join("modpacks");
						if !path.exists() {std::fs::create_dir_all(&path).unwrap()}
						
						let path = path.join(format!("{}.{}.{}.{}.amp", m.version[0], m.version[1], m.version[2], m.version[3]));
						let writer = File::create(path).unwrap();
						let path = m.path.clone();
						let version = (m.version[0] << 24) + (m.version[1] << 16) + (m.version[2] << 8) + m.version[3];
						thread::spawn(move || {
							modpack::pack(writer, path, version, None::<File>, progress).unwrap();
						});
					}
				})
				.tab("Patch", || {
					let patch_path = &mut m.packing.0;
					aeth::file_picker("Pick modpack", || -> aeth::FileDialog {
						aeth::FileDialog::new(m.path.to_string_lossy(), "")
							.add_extension(".amp", Some("Aetherment"))
							.add_extension(".amp.patch", Some("Aetherment"))
							.finish()
					}, patch_path);
					imgui::same_line();
					imgui::input_text("Patch path", patch_path, imgui::InputTextFlags::None);
					
					let patch_path = Path::new(patch_path);
					if imgui::button("Create", [0.0; 2]) {'create: {
						if !patch_path.exists() {
							aeth::show_error("Modpack Creation failed", "Patch file path is invalid");
							break 'create;
						}
						
						let pack = modpack::ModPack::load(File::open(&patch_path).unwrap()).unwrap();
						let patch_version = pack.version();
						let version = (m.version[0] << 24) + (m.version[1] << 16) + (m.version[2] << 8) + m.version[3];
						if version <= patch_version {
							aeth::show_error("Modpack Creation failed", "Version needs to be greater than patch version");
							break 'create;
						}
						
						let progress = Arc::new(Mutex::new((0, 0)));
						m.packing.1 = Some(progress.clone());
						
						let path = m.path.join("modpacks");
						if !path.exists() {std::fs::create_dir_all(&path).unwrap()}
						
						let patch_version = modpack::version_to_string(patch_version);
						let patch = Some(pack.into_inner());
						let path = path.join(format!("{patch_version}-{}.{}.{}.{}.amp.patch", m.version[0], m.version[1], m.version[2], m.version[3]));
						let writer = File::create(path).unwrap();
						let path = m.path.clone();
						thread::spawn(move || {
							modpack::pack(writer, path, version, patch, progress).unwrap();
						});
					}}
				})
				.finish();
			
			// TODO: make prettier
			if let Some(user) = &state.user {
				if imgui::button("Upload", [0.0; 2]) && m.uploading.0 == 0 {
					m.uploading.0 = 1;
					m.uploading.1.clear();
				}
				
				match m.uploading.0 {
					1 => match aeth::file_dialog("Select ModPack", || {
						// TODO: allow selecting patch files if mod is already uploaded
						aeth::FileDialog::new(m.path.to_string_lossy(), "")
							.add_extension(".amp", Some("Aetherment"))
							.finish()
					}) {
						aeth::FileDialogResult::Canceled => {
							m.uploading.0 = 0;
						},
						aeth::FileDialogResult::Success(paths) => {
							m.uploading.0 = 2;
							m.uploading.2 = paths[0].clone();
						}
						aeth::FileDialogResult::Busy => {},
					},
					2 => {
						imgui::set_next_window_pos(imgui::get_main_viewport_center(), imgui::Cond::Always, [0.5, 0.5]);
						imgui::set_next_window_size([1000.0, 800.0], imgui::Cond::Always);
						imgui::begin("Patch Notes###aetherment_upload", None, imgui::WindowFlags::None);
						imgui::input_text_multiline("##patchnotes", &mut m.uploading.1, [0.0, 700.0], imgui::InputTextFlags::None);
						m.uploading.1.truncate(500);
						imgui::text(&format!("{}/500", m.uploading.1.len()));
						if imgui::button("Confirm", [0.0; 2]) {
							let auth = user.token.clone();
							let path = m.path.clone();
							let modpack_path = m.uploading.2.clone();
							let patchnotes = m.uploading.1.clone();
							let refresh = self.refresh.clone();
							thread::spawn(move || {
								// TODO: fetch remote data and check if we can even upload this, to save on bandwidth and the likes
								match crate::creator::upload::upload_mod(&auth, &path, modpack_path, None, &patchnotes) {
									Ok(id) => {
										let mut remote = File::create(path.join("aeth")).unwrap();
										remote.write_all(&id.to_le_bytes()).unwrap();
										*refresh.lock().unwrap() = true;
									},
									Err(err) => log!(err, "{err:?}"),
								}
							});
							m.uploading.0 = 0;
						}
						imgui::end();
					},
					_ => {},
				}
			}
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
	
	pub fn load_mod(&mut self, path: PathBuf, state: &mut crate::Data) {
		if let Some(m) = &self.curmod && m.save.is_some() {
			save_mod(m, &state);
		}
		
		let mut m = match File::open(path.join("meta.json")) {
			Ok(f) => serde_json::from_reader(f).unwrap(),
			Err(_) => Meta {
				name: path.file_name().unwrap().to_str().unwrap().to_owned(),
				description: String::new(),
				contributors: Vec::new(),
				dependencies: Vec::new(),
				content_rating: 0,
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
			packing: (String::with_capacity(256), None),
			path: path,
			meta: m,
			uploading: (0, String::with_capacity(2000), PathBuf::new()),
			save: None,
		});
	}
}

fn save_mod(m: &CurMod, state: &crate::Data) {
	File::create(m.path.join("meta.json")).unwrap().write_all(crate::serialize_json(json!(m.meta)).as_bytes()).unwrap();
	
	if let Some(online) = &m.online && let Some(user) = &state.user {
		log!("update online mod meta");
		let mut allowed = Vec::new();
		for p in &m.meta.previews {
			if !online.previews.contains(p) {
				allowed.push(p.to_owned());
			}
		}
		
		let auth = user.token.clone();
		let path = m.path.clone();
		thread::spawn(move || {
			crate::creator::upload::update_mod_meta(&auth, &path, allowed).unwrap();
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
									let data = image::io::Reader::new(Cursor::new(crate::get_resource(&avatar_url)))
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
					author: crate::server::structs::IdName,
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
									author: v.author.name,
								};
								let thumbnail = v.thumbnail.clone();
								
								thread::spawn(move || {
									let data = image::io::Reader::new(Cursor::new(crate::get_resource(&thumbnail_url)))
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