use std::collections::BTreeMap;
use serde::Deserialize;
use crate::{CLIENT, SERVER};

#[derive(Debug)]
pub struct Tag {
	pub name: String,
	pub category: Option<String>,
	pub regex: Vec<regex::Regex>,
}

lazy_static! {
	pub static ref TAGS: Vec<Tag> = {
		#[derive(Deserialize)]
		pub struct TagS {
			name: String,
			category: Option<String>,
			regex: Vec<String>,
		}
		
		let tagss: Vec<TagS> = match CLIENT.get(format!("{}/api/mod/tags", SERVER)).send(){
			Ok(v) => match v.json() {
				Ok(v) => v,
				Err(_) => return Vec::new(),
			},
			Err(_) => return Vec::new(),
		};
		
		tagss.into_iter().map(|t| Tag {
			name: t.name,
			category: t.category,
			regex: t.regex.into_iter().map(|r| regex::Regex::new(&r).unwrap()).collect()
		}).collect()
	};
	
	pub static ref TAGS_SORTED: BTreeMap<Option<String>, Vec<(usize, String)>> = {
		let mut tags = BTreeMap::new();
		for (i, tag) in TAGS.iter().enumerate() {
			tags.entry(tag.category.clone()).or_insert(Vec::new()).push((i, tag.name.clone()));
		}
		
		tags
	};
}