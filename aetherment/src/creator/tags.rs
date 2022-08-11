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
		
		let tagss: Vec<TagS> = CLIENT.get(format!("{}/mod/tags.json", SERVER)).send().unwrap().json().unwrap();
		
		tagss.into_iter().map(|t| Tag {
			name: t.name,
			category: t.category,
			regex: t.regex.into_iter().map(|r| regex::Regex::new(&r).unwrap()).collect()
		}).collect()
	};
}