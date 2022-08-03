use std::{path::PathBuf, fs::File};
use serde::{Deserialize, Serialize};
use crate::gui::aeth;

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

pub struct Tab {
	mod_entries: Vec<String>,
	selected_mod: String,
	curmod: Option<Meta>,
}

impl Tab {
	pub fn new(state: &mut crate::Data) -> Self {
		let mut t = Tab {
			mod_entries: Vec::new(),
			selected_mod: "".to_owned(),
			curmod: None,
		};
		
		t.load_mods(state);
		
		t
	}
	
	pub fn draw(&mut self, state: &mut crate::Data) {
		aeth::divider("div", false)
		.left(100.0, || {
			for i in 0..self.mod_entries.len() {
				let e = self.mod_entries.get(i).unwrap();
				if imgui::selectable(e, e == &self.selected_mod, imgui::SelectableFlags::None, [0.0, 0.0]) {
					self.selected_mod = e.to_owned();
					self.load_mod(PathBuf::from(&state.config.local_path).join(e));
				}
			}
		}).right(400.0, || {
			if self.curmod.is_none() {return}
			let m = self.curmod.as_mut().unwrap();
			
			imgui::input_text("Name", &mut m.name, imgui::InputTextFlags::None);
			let limit = m.name.len() >= NAMEMAX;
			if limit {imgui::push_style_color(imgui::Col::Text, 0xFF3030B0)}
			imgui::text(&format!("{}/{}", m.name.len(), NAMEMAX));
			if limit {imgui::pop_style_color(1)}
			
			imgui::input_text_multiline("Description", &mut m.description, [0.0, 400.0], imgui::InputTextFlags::None);
			let limit = m.name.len() >= DESCMAX;
			if limit {imgui::push_style_color(imgui::Col::Text, 0xFF3030B0)}
			imgui::text(&format!("{}/{}", m.description.len(), DESCMAX));
			if limit {imgui::pop_style_color(1)}
			
			imgui::text("Contributors: TODO");
			imgui::text("Dependencies: TODO");
			
			imgui::checkbox("NSFW", &mut m.nsfw);
			
			imgui::text("Previews: TODO");
			
			if imgui::button("create modpack", [0.0, 0.0]) {
				let path = PathBuf::from(&state.config.local_path).join(&self.selected_mod);
				std::thread::spawn(move || {
					crate::creator::modpack::pack(path, 1 << 24, true);
					// crate::creator::modpack::pack(path, (1 << 24) + (1 << 16), true);
				});
			}
		});
	}
	
	pub fn load_mods(&mut self, state: &mut crate::Data) {
		self.mod_entries = std::fs::read_dir(&state.config.local_path)
			.unwrap()
			.into_iter()
			.filter_map(|e| {
				let e = e.unwrap();
				if e.metadata().unwrap().is_dir() {Some(e.file_name().to_str().unwrap().to_owned())} else {None}
			})
			.collect()
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
		
		self.curmod = Some(m);
	}
}