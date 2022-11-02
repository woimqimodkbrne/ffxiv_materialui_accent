// TODO: overwrite prompt

use std::{path::{PathBuf, Path}, collections::BTreeMap, time::SystemTime};
use chrono::{DateTime, Local};
use crate as imgui;
use super::F2;

const WHITE: u32 = 0xFFFFFFFF;
const EXTICONS: [(&str, char, u32); 27] = [
	// file-alt
	(".txt", '', WHITE),
	(".md", '', WHITE),
	
	// file-image
	(".png", '', WHITE),
	(".apng", '', WHITE),
	(".gif", '', WHITE),
	(".jpg", '', WHITE),
	(".jpeg", '', WHITE),
	(".tga", '', WHITE),
	(".tiff", '', WHITE),
	
	// file-archive
	(".zip", '', WHITE),
	(".7z", '', WHITE),
	(".tar.gz", '', WHITE),
	(".pmp", '', WHITE), // penumbra
	(".ttmp", '', WHITE), // textools
	(".ttmp2", '', WHITE), // textools
	
	// file-invoice, we deserve a custom icon c:
	(".amp", '', 0xFF7EB182),
	(".amp.patch", '', 0xFF7EB182),
	
	// file-code
	(".json", '', WHITE),
	(".yaml", '', WHITE),
	(".toml", '', WHITE),
	(".xml", '', WHITE),
	(".ini", '', WHITE),
	(".sh", '', WHITE),
	(".bat", '', WHITE),
	(".rs", '', WHITE),
	(".cs", '', WHITE),
	(".py", '', WHITE),
	// dont care about any others
];

pub enum FileDialogResult {
	Canceled,
	Success(Vec<PathBuf>),
	Busy,
}

pub struct FileDialogBuilder {
	path: String,
	file_name: String,
	limit: usize,
	open: bool,
	allow_dirs: bool,
	allow_files: bool,
	allowed_extensions: BTreeMap<String, Vec<String>>, // name, exts
	pinned: Vec<(PathBuf, String, String)>, // path, icon, name
}

impl FileDialogBuilder {
	pub fn add_extension<S>(mut self, ext: S, name: Option<S>) -> Self where
	S: Into<String> {
		let ext = ext.into();
		self.allowed_extensions.entry(if let Some(name) = name {name.into()} else {ext.clone()}).or_default().push(ext);
		self
	}
	
	pub fn add_pin<P, S>(mut self, path: P, icon: char, name: S) -> Self where
	P: Into<PathBuf>,
	S: Into<String> {
		self.pinned.push((path.into(), icon.to_string(), name.into()));
		self
	}
	
	pub fn limit(mut self, limit: usize) -> Self {
		self.limit = if self.open {if limit == 0 {usize::MAX} else {limit}} else {1};
		self
	}
	
	pub fn save_mode(mut self, state: bool) -> Self {
		self.open = !state;
		if state {self.limit = 1}
		self
	}
	
	pub fn allow_directories(mut self, state: bool) -> Self {
		self.allow_dirs = state;
		self
	}
	
	pub fn allow_files(mut self, state: bool) -> Self {
		self.allow_files = state;
		self
	}
	
	pub fn finish(self) -> FileDialog {
		let mut s = FileDialog {
			prev_path: self.path.clone(),
			path: self.path,
			file_ext: "".to_owned(),
			file_name: self.file_name,
			limit: self.limit,
			open: self.open,
			allow_dirs: self.allow_dirs,
			allow_files: self.allow_files,
			allowed_extensions: self.allowed_extensions,
			pinned: self.pinned,
			search: String::with_capacity(64),
			cur_dirs: Vec::new(),
			cur_files: Vec::new(),
			sort_column: 0,
			sort_dir: imgui::SortDirection::Ascending,
			editing_path: false,
			selected: Vec::new(),
		};
		_ = s.load_dir();
		s.select_ext();
		
		s
	}
}

pub struct FileDialog {
	prev_path: String,
	path: String,
	file_name: String,
	file_ext: String,
	limit: usize,
	open: bool,
	allow_dirs: bool,
	allow_files: bool,
	allowed_extensions: BTreeMap<String, Vec<String>>, // name, exts
	pinned: Vec<(PathBuf, String, String)>, // path, icon, name
	search: String,
	cur_dirs: Vec<(PathBuf, String)>, // path, name
	cur_files: Vec<(PathBuf, (String, u32), String, DateTime<Local>, DateTime<Local>, u64)>, // path, (icon, clr), name, created, modified, size
	sort_column: i16,
	sort_dir: imgui::SortDirection,
	editing_path: bool,
	selected: Vec<PathBuf>,
}

impl FileDialog {
	pub fn new<P, S>(path: P, file_name: S) -> FileDialogBuilder where
	P: Into<String>,
	S: Into<String> {
		let mut pinned = Vec::new();
		#[cfg(windows)]
		{ // lol
			for i in 65..=90 {
				let s = format!("{}:\\", char::from_u32(i).unwrap());
				let path = PathBuf::from(&s);
				if path.exists() {pinned.push((path, ''.to_string(), s))}
			}
		}
		
		#[cfg(unix)]
		{ // TODO: use `mount -l` (`diskutil list` on macos) to get all mount points
			pinned.push(("/".into(), ''.to_string(), "/".to_owned()))
		}
		
		// if any of these are somehow None wtf happened
		pinned.push((dirs::home_dir().unwrap().into(), ''.to_string(), "Home".into()));
		pinned.push((dirs::desktop_dir().unwrap().into(), ''.to_string(), "Desktop".into()));
		pinned.push((dirs::download_dir().unwrap().into(), ''.to_string(), "Downloads".into()));
		pinned.push((dirs::document_dir().unwrap().into(), ''.to_string(), "Documents".into()));
		pinned.push((dirs::picture_dir().unwrap().into(), ''.to_string(), "Pictures".into()));
		pinned.push((dirs::video_dir().unwrap().into(), ''.to_string(), "Videos".into()));
		pinned.push((dirs::audio_dir().unwrap().into(), ''.to_string(), "Music".into()));
		
		let mut path = path.into();
		path.reserve(256);
		
		let mut file_name = file_name.into();
		file_name.reserve(64);
		
		FileDialogBuilder {
			path,
			file_name,
			limit: 1,
			open: true,
			allow_dirs: false,
			allow_files: true,
			allowed_extensions: BTreeMap::new(),
			pinned,
		}
	}
	
	pub fn draw(&mut self) -> FileDialogResult {
		// Up dir
		if super::button_icon("") && let Some(parent) = Path::new(&self.path).parent() {
			self.path = parent.to_string_lossy().to_string();
			self.prev_path = self.path.clone();
			_ = self.load_dir();
		}
		
		// Path bar edit
		imgui::same_line();
		if super::button_icon("") {self.editing_path = !self.editing_path}
		
		if self.editing_path {
			// Path bar display
			imgui::same_line();
			super::next_max_width();
			imgui::input_text("##path", &mut self.path, imgui::InputTextFlags::AutoSelectAll);
			if imgui::is_item_activated() {
				self.prev_path = self.path.clone();
			} else if imgui::is_item_deactivated() {
				self.editing_path = false;
				if self.load_dir().is_err() {
					self.path = self.prev_path.clone();
				}
			}
		} else {
			// Path segments
			imgui::same_line();
			imgui::dummy([0.0; 2]);
			imgui::push_style_var2(imgui::StyleVar::ItemSpacing, [0.0; 2]);
			let path = Path::new(&self.path);
			for (i, seg) in path.iter().enumerate() {
				// if this isn't done there is a '/' after the drive for some reason
				#[cfg(windows)]
				if i == 1 {continue}
				
				if i > 0 {
					imgui::same_line();
					imgui::push_font(super::fa5());
					imgui::text("");
					imgui::pop_font();
				}
				
				imgui::same_line();
				imgui::button(&seg.to_string_lossy(), [0.0; 2]);
			}
			imgui::pop_style_var(1);
			imgui::same_line();
			imgui::dummy([0.0; 2]); // else the style var still affects the search bar
		}
		
		// Search bar
		// probably should make it update a filtered map of all items instead of doing it every frame but oh well
		super::next_max_width();
		imgui::input_text_with_hint("##search", "Search", &mut self.search, imgui::InputTextFlags::None);
		
		// pinned and curdir
		let spacing = imgui::get_style().item_spacing;
		let footer_height = super::frame_height() * 2.0 + spacing.y() * 2.0;
		let h = imgui::get_content_region_avail().y() - footer_height;
		
		imgui::begin_child("div_holder", [0.0, h], false, imgui::WindowFlags::None);
		super::divider("div", false)
			.left(50.0, || {
				// pins
				let mut load_dir = false;
				for (path, icon, name) in &self.pinned {
					if super::selectable_with_icon(&icon, &name, false, imgui::SelectableFlags::None, [0.0; 2]) && path.exists() {
						self.path = path.to_string_lossy().to_string();
						self.prev_path = path.to_string_lossy().to_string();
						load_dir = true;
					}
				}
				
				if load_dir {_ = self.load_dir()}
			}).right(100.0, || {
				// cur dir
				use imgui::TableFlags;
				super::table("curdir", 4, TableFlags::Sortable | TableFlags::SizingFixedFit | TableFlags::ScrollY | TableFlags::NoHostExtendX | TableFlags::Hideable, [0.0; 2], 0.0, || {
					imgui::table_setup_column("Name", imgui::TableColumnFlags::WidthStretch, -1.0, 0);
					imgui::table_setup_column("Date Modified", imgui::TableColumnFlags::WidthFixed, -1.0, 1);
					imgui::table_setup_column("Date Created", imgui::TableColumnFlags::WidthFixed, -1.0, 2);
					imgui::table_setup_column("Size", imgui::TableColumnFlags::WidthFixed, -1.0, 4);
					imgui::table_headers_row();
					
					let specs = imgui::table_get_sort_specs();
					if specs.dirty() {
						let mut sort = false;
						if specs.column_index() != self.sort_column {
							self.sort_column = specs.column_index();
							sort = true;
						}
						
						if specs.column_sort_direction() != self.sort_dir {
							self.sort_dir = specs.column_sort_direction();
							sort = true;
						}
						
						if sort {self.sort()}
					}
					
					let search = self.search.to_lowercase();
					
					use imgui::SelectableFlags;
					for (path, name) in &self.cur_dirs {
						if !search.is_empty() && !name.to_lowercase().contains(&search) {continue}
						
						imgui::table_next_row(imgui::TableRowFlags::None, 0.0);
						imgui::table_next_column();
						let icon = if imgui::get_hovered_id() == imgui::get_id(name) {""} else {""}; // cute little animation c:
						if super::selectable_with_icon(icon, name, self.selected.contains(&path), SelectableFlags::AllowDoubleClick | SelectableFlags::SpanAllColumns, [0.0; 2]) {
							if imgui::is_mouse_double_clicked(imgui::MouseButton::Left) {
								self.path = path.to_string_lossy().to_string();
								if self.load_dir().is_err() {self.path = self.prev_path.clone()}
								break;
							} else if self.allow_dirs {
								if imgui::get_io().key_shift {
									self.selected.insert(0, path.clone());
									self.selected.truncate(self.limit);
								} else {
									self.selected.clear();
									self.selected.push(path.clone());
								}
							}
						}
					}
					
					for (path, (icon, icon_clr), name, created, modified, size) in &self.cur_files {
						if !search.is_empty() && !name.to_lowercase().contains(&search) {continue}
						
						imgui::table_next_row(imgui::TableRowFlags::None, 0.0);
						imgui::table_next_column();
						if super::selectable_with_icon_u32(&icon, *icon_clr, &name, self.selected.contains(&path), SelectableFlags::SpanAllColumns, [0.0; 2]) {
							if imgui::get_io().key_shift {
								self.selected.insert(0, path.clone());
								self.selected.truncate(self.limit);
							} else {
								self.selected.clear();
								self.selected.push(path.clone());
							}
							
							if self.selected.len() == 1 {
								self.file_name.clear();
								self.file_name.push_str(&self.selected[0].file_name().unwrap().to_string_lossy());
								self.select_ext();
								break;
							}
						}
						
						// TODO: display the date in the system locale format
						imgui::table_next_column();
						imgui::text(&modified.format("%Y/%m/%d %H:%M").to_string());
						
						imgui::table_next_column();
						imgui::text(&created.format("%Y/%m/%d %H:%M").to_string());
						
						imgui::table_next_column();
						imgui::text(&super::format_size(*size));
					}
				});
			});
		imgui::end_child();
		
		// footer
		imgui::text("File Name:");
		imgui::same_line();
		if self.allowed_extensions.len() > 0 {imgui::set_next_item_width(-200.0 - spacing.x())} else {super::next_max_width()}
		if imgui::input_text("##filename", &mut self.file_name, imgui::InputTextFlags::None) {self.select_ext()}
		if self.allowed_extensions.len() > 0 {
			imgui::same_line();
			super::next_max_width();
			if imgui::begin_combo("##ext", &self.file_ext, imgui::ComboFlags::None) {
				for (ext_name, exts) in &self.allowed_extensions {
					let name = if exts.len() == 1 && &exts[0] == ext_name {
						ext_name.clone()
					} else {
						format!("{ext_name} ({})", exts.join(", "))
					};
					
					if imgui::selectable(&name, exts.contains(&self.file_ext), imgui::SelectableFlags::None, [0.0; 2]) {
						let ext = &exts[0];
						if !self.file_name.ends_with(ext) {
							if let Some(p) = self.file_name.find('.') {self.file_name.truncate(p)}
							if self.file_name.len() > 0 {self.file_name.push_str(ext)}
							self.file_ext = ext_name.clone();
						}
					}
				}
				imgui::end_combo();
			}
		}
		
		let w = (imgui::get_content_region_avail().x() - spacing.x()) / 2.0;
		if imgui::button(if self.open {"Open"} else {"Save"}, [w, 0.0]) {
			let c = Path::new(&self.path).join(&self.file_name);
			if (!self.open || c.exists()) && !self.selected.contains(&c) {
				self.selected.insert(0, c.to_path_buf());
				self.selected.truncate(self.limit);
			}
			
			if self.selected.len() > 0 {
				return FileDialogResult::Success(self.selected.clone())
			}
		}
		imgui::same_line();
		if imgui::button("Cancel", [w, 0.0]) {return FileDialogResult::Canceled}
		
		FileDialogResult::Busy
	}
	
	fn load_dir(&mut self) -> std::io::Result<()> {
		let readdir = std::fs::read_dir(Path::new(&self.path))?;
		self.cur_dirs.clear();
		self.cur_files.clear();
		for entry in readdir {
			if let Ok(entry) = entry && let Ok(meta) = entry.metadata() {
				if meta.is_dir() {
					self.cur_dirs.push((entry.path(), entry.file_name().to_string_lossy().to_string()));
				} else if meta.is_file() && self.allow_files {
					let name = entry.file_name().to_string_lossy().to_string();
					if self.allowed_extensions.len() == 0 || 'allowed: {
						for (_ext_name, exts) in &self.allowed_extensions {
							for ext in exts {
								if name.ends_with(ext) {
									break 'allowed true;
								}
							}
						}
						
						false
					} {
						self.cur_files.push((
							entry.path(),
							'icon: {
								for (ext, icon, clr) in EXTICONS {
									if name.ends_with(ext) {
										break 'icon (icon.to_string(), clr);
									}
								}
								
								(''.to_string(), WHITE)
							},
							name,
							meta.created().unwrap_or(SystemTime::UNIX_EPOCH).into(),
							meta.modified().unwrap_or(SystemTime::UNIX_EPOCH).into(),
							meta.len()
						));
					}
				}
			}
		}
		
		self.sort();
		self.selected.clear();
		
		Ok(())
	}
	
	fn sort(&mut self) {
		// dirs is always by name
		self.cur_dirs.sort_by(|a, b| a.1.to_lowercase().cmp(&b.1.to_lowercase()));
		
		match self.sort_column {
			0 => self.cur_files.sort_by(|a, b| a.2.to_lowercase().cmp(&b.2.to_lowercase())), // name
			1 => self.cur_files.sort_by(|a, b| a.4.cmp(&b.4)), // modified
			2 => self.cur_files.sort_by(|a, b| a.3.cmp(&b.3)), // created
			3 => self.cur_files.sort_by(|a, b| a.5.cmp(&b.5)), // size
			_ => {},
		}
		
		if self.sort_dir == imgui::SortDirection::Descending {
			self.cur_dirs.reverse();
			self.cur_files.reverse();
		}
	}
	
	fn select_ext(&mut self) {
		for (ext_name, exts) in &self.allowed_extensions {
			for ext in exts {
				if self.file_name.ends_with(ext) {
					self.file_ext = ext_name.clone();
					return;
				}
			}
		}
		
		self.file_ext = "".to_owned();
	}
}

pub fn file_picker<S, F>(title: S, setup: F, path: &mut String) -> bool where
S: AsRef<str>,
F: FnOnce() -> FileDialog {
	let dialog_ptr = imgui::get_state_storage().ptr::<FileDialog>(imgui::get_id(title.as_ref()), 0 as *mut _);
	if super::button_icon(&format!("###{}", title.as_ref())) && dialog_ptr.is_null() {
		*dialog_ptr = Box::into_raw(Box::new(setup()));
	}
	
	if let Some(dialog) = unsafe{(*dialog_ptr).as_mut()} {
		let mut open = true;
		imgui::set_next_window_size([800.0, 600.0], imgui::Cond::FirstUseEver);
		imgui::begin(title.as_ref(), Some(&mut open), imgui::WindowFlags::NoSavedSettings);
		let mut r = match dialog.draw() {
			FileDialogResult::Canceled => {
				unsafe{Box::from_raw(*dialog_ptr)};
				*dialog_ptr = 0 as *mut _;
				false
			},
			FileDialogResult::Success(paths) => {
				unsafe{Box::from_raw(*dialog_ptr)};
				*dialog_ptr = 0 as *mut _;
				path.clear();
				path.push_str(&paths[0].to_string_lossy());
				true
			},
			FileDialogResult::Busy => false,
		};
		imgui::end();
		
		if !open {
			unsafe{Box::from_raw(*dialog_ptr)};
			*dialog_ptr = 0 as *mut _;
			r = false;
		}
		
		r
	} else {
		false
	}
}

pub fn file_dialog<S, F>(title: S, setup: F) -> FileDialogResult where
S: AsRef<str>,
F: FnOnce() -> FileDialog {
	let dialog_ptr = imgui::get_state_storage().ptr::<FileDialog>(imgui::get_id(title.as_ref()), 0 as *mut _);
	if dialog_ptr.is_null() {
		*dialog_ptr = Box::into_raw(Box::new(setup()));
	}
	
	if let Some(dialog) = unsafe{(*dialog_ptr).as_mut()} {
		let mut open = true;
		imgui::set_next_window_size([800.0, 600.0], imgui::Cond::FirstUseEver);
		imgui::begin(title.as_ref(), Some(&mut open), imgui::WindowFlags::NoSavedSettings);
		let mut r = dialog.draw();
		match r {
			FileDialogResult::Canceled | FileDialogResult::Success(_) => {
				unsafe{Box::from_raw(*dialog_ptr)};
				*dialog_ptr = 0 as *mut _;
			},
			FileDialogResult::Busy => {},
		};
		imgui::end();
		
		if !open {
			unsafe{Box::from_raw(*dialog_ptr)};
			*dialog_ptr = 0 as *mut _;
			r = FileDialogResult::Canceled;
		}
		
		r
	} else {
		FileDialogResult::Canceled
	}
}