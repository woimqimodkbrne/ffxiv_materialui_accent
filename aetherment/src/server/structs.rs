use std::{sync::{Arc, Mutex}, thread};
use imgui::aeth::{Texture, TextureOptions};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct IdName {
	pub id: i32,
	pub name: String,
}

pub struct IdNameImg {
	pub id: i32,
	pub name: String,
	pub img: Arc<Mutex<Texture>>,
}

impl<'de> Deserialize<'de> for IdNameImg {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
	D: serde::Deserializer<'de> {
		#[derive(Deserialize)]
		struct IdNameImgResp {
			id: i32,
			name: String,
			img: String,
		}
		
		let v: IdNameImgResp = Deserialize::deserialize(deserializer)?;
		let img = Arc::new(Mutex::new(Texture::empty()));
		
		{
			let texture = img.clone();
			thread::spawn(move || {
				let img = image::io::Reader::new(std::io::Cursor::new(crate::get_resource(&v.img)))
					.with_guessed_format()
					.unwrap()
					.decode()
					.unwrap();
				
				*texture.lock().unwrap() = Texture::with_data(TextureOptions {
					width: img.width() as i32,
					height: img.height() as i32,
					format: 28, // DXGI_FORMAT_R8G8B8A8_UNORM
					usage: 1, // D3D11_USAGE_IMMUTABLE
					cpu_access_flags: 0,
				}, &img.into_rgba8());
			});
		}
		
		Ok(Self {
			id: v.id,
			name: v.name,
			img,
		})
	}
}