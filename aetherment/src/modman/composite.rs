pub mod tex;

pub trait Composite {
	fn get_files(&self) -> Vec<&str>;
}