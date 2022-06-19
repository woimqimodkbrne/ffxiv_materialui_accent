pub mod external {
	pub mod dds;
	pub mod png;
	pub mod tga;
	
	pub mod fbx;
	
	pub mod json;
}

pub mod game {
	pub type Result<T, E = ironworks::Error> = std::result::Result<T, E>;
	
	pub mod tex;
	pub mod mtrl;
}