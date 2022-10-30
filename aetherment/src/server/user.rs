use std::io::Cursor;
use serde::Deserialize;
use crate::{SERVER, SERVERCDN, CLIENT, gui::aeth::{Texture, TextureOptions}};

pub struct User {
	pub id: i32,
	pub name: String,
	pub token: String,
	pub avatar: Texture,
}

impl User {
	pub fn new(token: String) -> Option<Self> {
		#[derive(Deserialize)]
		#[serde(untagged)]
		enum Resp {
			Success{id: i32, name: String, avatar: Option<String>},
			#[allow(dead_code)] Error{error: String}
		}
		
		match CLIENT.get(format!("{SERVER}/api/user/stats"))
			.header("Authorization", &token)
			.send()
			.ok()?
			.json::<Resp>()
			.ok()? {
			Resp::Success{id, name, avatar} => Some(User {
				id: id,
				name: name,
				token,
				avatar: if let Some(avatar) = &avatar {
					let resp_avatar = CLIENT.get(format!("{SERVERCDN}/u/{}/p/{}", id, avatar))
						.send()
						.ok()?;
					
					Texture::with_data(TextureOptions {
						width: 64,
						height: 64,
						format: 28, // DXGI_FORMAT_R8G8B8A8_UNORM
						usage: 1, // D3D11_USAGE_IMMUTABLE
						cpu_access_flags: 0,
					}, &image::io::Reader::new(Cursor::new(resp_avatar
						.bytes()
						.ok()?
						.to_vec()))
						.with_guessed_format()
						.ok()?
						.decode()
						.ok()?
						.resize_exact(64, 64, image::imageops::FilterType::Triangle)
						.into_rgba8())
				} else {
					// TODO: default avatar
					Texture::empty()
				}
			}),
			Resp::Error{error: _} => None,
		}
	}
	
	pub fn load() -> Option<Self> {
		let token = keyring::Entry::new("Aetherment", "user")
			.get_password().ok()?;
		
		Self::new(token)
	}
	
	pub fn store(&self) -> Result<(), keyring::Error> {
		keyring::Entry::new("Aetherment", "user")
			.set_password(&self.token)?;
		
		Ok(())
	}
	
	pub fn delete(&self) -> Result<(), keyring::Error> {
		keyring::Entry::new("Aetherment", "user").delete_password()
	}
}