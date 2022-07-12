use crate::apply::penumbra::Config;

mod generic;
mod tex;

pub use generic::*;
pub use tex::*;

pub trait Viewer {
	fn valid_imports(&self) -> Vec<String>;
	fn valid_exports(&self) -> Vec<String>;
	fn draw(&mut self, state: &mut crate::Data, conf: Option<&mut Config>);
	fn save(&self, ext: &str, writer: &mut Vec<u8>);
}