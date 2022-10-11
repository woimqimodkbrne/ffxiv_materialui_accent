use std::io::Cursor;
use crate::{CLIENT, gui::aeth::{Texture, TextureOptions}, SERVERCDN};

pub struct User {
	pub id: i32,
	pub name: String,
	pub token: String,
	pub avatar: Texture,
}

impl User {
	pub fn new(id: i32, name: String, token: String) -> Self {
		let resp = CLIENT.get(format!("{}/u/{}/avatar.png", SERVERCDN, id))
			.send()
			.unwrap();
		
		User {
			id,
			name,
			token,
			avatar: if resp.status() == reqwest::StatusCode::OK {
				Texture::with_data(TextureOptions {
					width: 64,
					height: 64,
					format: 28, // DXGI_FORMAT_R8G8B8A8_UNORM
					usage: 1, // D3D11_USAGE_IMMUTABLE
					cpu_access_flags: 0,
				}, &image::io::Reader::new(Cursor::new(resp
					.bytes()
					.unwrap()
					.to_vec()))
					.with_guessed_format()
					.unwrap()
					.decode()
					.unwrap()
					.resize_exact(64, 64, image::imageops::FilterType::Triangle)
					.into_rgba8())
			} else {
				// TODO: default avatar
				Texture::empty()
			}
		}
	}
	
	pub fn load() -> Option<Self> {
		let creds = keyring::Entry::new("Aetherment", "user")
			.get_password().ok()?;
		let mut creds = creds.split('\0');
		
		Some(Self::new(creds.next()?.parse::<i32>().ok()?, creds.next()?.to_owned(), creds.next()?.to_owned()))
	}
	
	pub fn store(&self) -> Result<(), keyring::Error> {
		keyring::Entry::new("Aetherment", "user")
			.set_password(&format!("{}\0{}\0{}", self.id, self.name, self.token))?;
		
		Ok(())
	}
	
	pub fn delete(&self) -> Result<(), keyring::Error> {
		keyring::Entry::new("Aetherment", "user").delete_password()
	}
}