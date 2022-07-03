use serde::Deserialize;

#[derive(Deserialize)]
pub struct IdName {
	pub id: String,
	pub name: String,
}

#[derive(Deserialize)]
pub struct Mod {
	pub id: String,
	pub name: String,
	pub description: String,
	pub author: IdName,
	pub contributors: Vec<IdName>,
	pub main_mod: Option<IdName>,
	pub dependencies: Vec<IdName>,
	pub size_install: i64,
	pub size_download: i64,
	pub tags: Vec<i16>,
	pub previews: Vec<String>,
	pub nsfw: bool,
	pub likes: i32,
	pub downloads: i32,
}