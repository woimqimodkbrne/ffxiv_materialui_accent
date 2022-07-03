use std::fs::File;

mod tex;

pub use tex::*;

pub trait Viewer {
	fn valid_imports(&self) -> Vec<String>;
	fn valid_exports(&self) -> Vec<String>;
	fn draw(&mut self, state: &mut crate::Data);
	fn save(&self, writer: &mut File); // can't use Write + Seek sadly
}