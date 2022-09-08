use std::{collections::{HashMap, HashSet}, path::PathBuf};
use serde::{Deserialize, Serialize};
use crate::creator::tags::TAGS;
use self::penumbra::ConfOption;

pub mod penumbra;

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct Datas {
	pub dalamud: Option<i32>, // TODO
	pub penumbra: Option<penumbra::Config>,
}

impl Datas {
	pub fn tags(&self) -> HashMap<String, Vec<String>> {
		let mut tags = HashMap::<String, Vec<String>>::new();
		
		if self.dalamud.is_some() {tags.insert(TAGS[1].name.clone(), Vec::new());} // Dalamud Style
		
		if let Some(penumbra) = &self.penumbra {
			tags.insert(TAGS[2].name.clone(), Vec::new()); // Penumbra
			
			penumbra.files.iter().for_each(|(path, _)| TAGS.iter().for_each(|tag| {
				tag.regex.iter().for_each(|r| if r.is_match(&path) {
					tags.entry(tag.name.clone()).or_insert_with(|| Vec::new()).push(path.to_owned());
				})
			}));
			
			penumbra.options.iter().for_each(|o| match o {
				ConfOption::Single(opt) | ConfOption::Multi(opt) => opt.options.iter().for_each(|o| o.files.iter().for_each(|(path, _)| TAGS.iter().for_each(|tag| {
					tag.regex.iter().for_each(|r| if r.is_match(&path) {
						tags.entry(tag.name.clone()).or_insert_with(|| Vec::new()).push(path.to_owned());
					})
				}))),
				_ => {tags.insert(TAGS[0].name.clone(), Vec::new());} // Customizable
			})
		}
		
		tags
	}
	
	pub fn cleanup(&self, mod_path: &PathBuf) {
		let mut used_files = HashSet::new();
		
		if let Some(penumbra) = &self.penumbra {
			penumbra.files.iter().for_each(|(_, f)| f.0.iter().for_each(|l| l.paths.iter().for_each(|p| {used_files.insert(p.replace("\\", "/"));})));
			penumbra.options.iter().for_each(|o| match o {
				ConfOption::Single(o) | ConfOption::Multi(o) => o.options.iter().for_each(|o| o.files.iter().for_each(|(_, f)| f.0.iter().for_each(|l| l.paths.iter().for_each(|p| {used_files.insert(p.replace("\\", "/"));})))),
				_ => {},
			})
		}
		
		fn check_dir(root: &PathBuf, d: &PathBuf, files: &HashSet<String>) {
			if !d.exists() {return}
			
			std::fs::read_dir(d)
				.unwrap()
				.for_each(|e| {
					let e = e.unwrap();
					if e.path().is_dir() {
						check_dir(root, &e.path(), files);
					}
				});
			
			let mut c = 0;
			std::fs::read_dir(d)
				.unwrap()
				.for_each(|e| {
					c += 1;
					let e = e.unwrap();
					if e.path().is_file() && !files.contains(&e.path().strip_prefix(root).unwrap().to_str().unwrap().replace("\\", "/")) {
						std::fs::remove_file(e.path()).unwrap();
						c -= 1;
					}
				});
			
			if c == 0 && &root.join("files") != d {
				std::fs::remove_dir(d).unwrap();
			}
		}
		
		let fp = mod_path.join("files");
		check_dir(&mod_path, &fp, &used_files);
	}
}