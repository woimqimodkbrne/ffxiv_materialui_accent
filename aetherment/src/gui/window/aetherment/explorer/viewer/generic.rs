// TODO: save the changes to layers

use std::{collections::HashMap,  path::PathBuf, io::Write};
use crate::{apply::penumbra::{ConfSetting, Config}, GAME, gui::imgui};
use super::Viewer;

pub struct Generic {
	ext: String,
	gamepath: String,
	#[allow(dead_code)] rootpath: Option<PathBuf>,
}

impl Generic {
	pub fn new(gamepath: String, rootpath: Option<PathBuf>, _realpaths: Option<Vec<Vec<Option<String>>>>, _settings: Option<HashMap<String, ConfSetting>>) -> Self {
		Generic {
			ext: format!(".{}", gamepath.split('.').last().unwrap()),
			gamepath,
			rootpath,
		}
	}
}

impl Viewer for Generic {
	fn valid_imports(&self) -> Vec<String> {
		vec![self.ext.to_owned()]
	}
	
	fn valid_exports(&self) -> Vec<String> {
		vec![self.ext.to_owned()]
	}
	
	fn draw(&mut self, _state: &mut crate::Data, _conf: Option<&mut Config>) {
		imgui::text("generic");
	}
	
	fn save(&self, _ext: &str, writer: &mut Vec<u8>) {
		writer.write_all(&GAME.file::<Vec<u8>>(&self.gamepath).unwrap()).unwrap();
	}
}