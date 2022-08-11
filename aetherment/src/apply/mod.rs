use std::collections::HashMap;
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
}