use std::io::Write;
use crate::GAME;
use super::Viewer;

pub struct Generic {
	ext: String,
	gamepath: String,
}

impl Generic {
	pub fn new(gamepath: String, _conf: Option<super::Conf>) -> Self {
		Generic {
			ext: format!(".{}", gamepath.split('.').last().unwrap()),
			gamepath,
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
	
	fn draw(&mut self, _state: &mut crate::Data, _conf: Option<super::Conf>) {
		imgui::text("generic");
	}
	
	fn save(&self, _ext: &str, writer: &mut Vec<u8>) {
		writer.write_all(&GAME.file::<Vec<u8>>(&self.gamepath).unwrap()).unwrap();
	}
}