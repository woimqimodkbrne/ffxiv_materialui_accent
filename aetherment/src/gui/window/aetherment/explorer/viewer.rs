use std::{path::PathBuf, fs::File, io::{Write, Cursor}, collections::HashMap};
use serde_json::json;
use crate::apply::{penumbra::PenumbraFile, Datas};

mod generic;
mod options;
mod tex;
mod mtrl;

pub use generic::*;
pub use options::*;
pub use tex::*;
pub use mtrl::*;

pub struct Conf<'a> {
	pub path: PathBuf,
	pub datas: &'a mut Datas,
	pub option: &'a mut String,
	pub sub_option: &'a mut String,
}

impl <'a> Conf<'a> {
	pub fn save(&self) {
		File::create(&self.path.join("datas.json")).unwrap().write_all(crate::serialize_json(json!(self.datas)).as_bytes()).unwrap();
	}
	
	pub fn reload_penumbra(&self) {
		use crate::api::penumbra;
		
		// TODO: redraw settings
		// TODO: dont check every file to see if we need to create a temp file
		let load_file = crate::apply::penumbra::get_load_file(Some(self.path.clone()));
		
		let mapfile = |f: &PenumbraFile| -> String {
			if f.0.len() > 1 || f.0[0].paths.len() > 1 {
				use crate::apply::penumbra;
				
				let mut layers = f.0.iter();
				let layer = layers.next().unwrap();
				let mut tex = penumbra::resolve_layer(&penumbra::Layer {
					value: if let Some(id) = &layer.id {self.datas.penumbra.as_ref().unwrap().options.iter().find(|v| v.id() == Some(id)).and_then(|v| Some(v.default()))} else {None},
					files: layer.paths.clone()
				}, &load_file).expect("Failed resolving layer");
				while let Some(layer) = layers.next() {
					let l = penumbra::resolve_layer(&penumbra::Layer {
						value: if let Some(id) = &layer.id {self.datas.penumbra.as_ref().unwrap().options.iter().find(|v| v.id() == Some(id)).and_then(|v| Some(v.default()))} else {None},
						files: layer.paths.clone()
					}, &load_file).expect("Failed resolving layer");
					l.overlay_onto(&mut tex);
				}
				
				let temp = self.path.join("temp");
				std::fs::create_dir_all(&temp).unwrap();
				let mut data = Vec::new();
				tex.write(&mut Cursor::new(&mut data));
				// let hash = blake3::hash(&data).to_hex().to_string();
				let hash = crate::hash_str(blake3::hash(&data).as_bytes());
				let file = temp.join(&hash);
				if !file.exists() {
					File::create(&file).unwrap().write_all(&data).unwrap();
				}
				file.to_str().unwrap().to_owned()
			} else {
				self.path.join(&f.0[0].paths[0]).to_str().unwrap().to_owned()
			}
		};
		
		log!("penumbra dev mod");
		penumbra::remove_mod("aetherment_creator", i32::MAX); // without removing the old it keeps old paths
		penumbra::add_mod(
			"aetherment_creator", 
			self.datas.penumbra.as_ref().unwrap().files_ref(&self.option, &self.sub_option)
				.unwrap()
				.into_iter()
				.map(|(gamepath, file)| (gamepath.to_owned(), mapfile(file)))
				.collect::<HashMap<String, String>>(),
			"",
			i32::MAX
		);
		penumbra::redraw_self();
	}
	
	pub fn file_mut<'b>(&'b mut self, path: &str) -> Option<&'b mut PenumbraFile> {
		self.datas.penumbra.as_mut().unwrap().file_mut(self.option, self.sub_option, path)
	}
	
	pub fn file_ref<'b>(&'b self, path: &str) -> Option<&'b PenumbraFile> {
		self.datas.penumbra.as_ref().unwrap().file_ref(self.option, self.sub_option, path)
	}
}

pub trait Viewer {
	fn valid_imports(&self) -> Vec<&str>;
	fn valid_exports(&self) -> Vec<&str>;
	fn draw(&mut self, state: &mut crate::Data, conf: Option<Conf>);
	fn save(&self, ext: &str, writer: &mut Vec<u8>);
}