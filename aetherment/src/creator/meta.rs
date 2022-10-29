use imgui::aeth::{TextureOptions, Texture};
use serde::{Deserialize, Serialize, Serializer};

pub const MAX_NAME_LEN: usize = 64;
pub const MAX_DESC_LEN: usize = 5000;
pub const MAX_CONTRIBUTORS: usize = 8;
pub const MAX_DEPENDENCIES: usize = 8;
pub const PREVIEW_RESOLUTION: [u32; 2] = [1620, 1080];

pub const CONTRIBUTOR_IMG: TextureOptions = TextureOptions {
	width: 32,
	height: 32,
	format: 28, // DXGI_FORMAT_R8G8B8A8_UNORM
	usage: 1, // D3D11_USAGE_IMMUTABLE
	cpu_access_flags: 0,
};

pub const DEPENDENCY_IMG: TextureOptions = TextureOptions {
	width: 45,
	height: 30,
	format: 28, // DXGI_FORMAT_R8G8B8A8_UNORM
	usage: 1, // D3D11_USAGE_IMMUTABLE
	cpu_access_flags: 0,
};

#[derive(Debug)]
pub struct ContributorTexture(pub Texture, pub Vec<u8>);

impl std::ops::Deref for ContributorTexture {
	type Target = Texture;
	
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<'de> Deserialize<'de> for ContributorTexture {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
	D: serde::Deserializer<'de> {
		let v: String = Deserialize::deserialize(deserializer)?;
		let data = base64::decode(v).unwrap();
		Ok(if data.len() == 0 {
			ContributorTexture(Texture::empty(), data)
		} else {
			ContributorTexture(Texture::with_data(CONTRIBUTOR_IMG, &data), data)
		})
	}
}

impl Serialize for ContributorTexture {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
	S: Serializer {
		serializer.serialize_str(&base64::encode(&self.1))
	}
}

#[derive(Debug)]
pub struct DependencyTexture(pub Texture, pub Vec<u8>);

impl std::ops::Deref for DependencyTexture {
	type Target = Texture;
	
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<'de> Deserialize<'de> for DependencyTexture {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
	D: serde::Deserializer<'de> {
		let v: String = Deserialize::deserialize(deserializer)?;
		let data = base64::decode(v).unwrap();
		Ok(if data.len() == 0 {
			DependencyTexture(Texture::empty(), data)
		} else {
			DependencyTexture(Texture::with_data(DEPENDENCY_IMG, &data), data)
		})
	}
}

impl Serialize for DependencyTexture {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
	S: Serializer {
		serializer.serialize_str(&base64::encode(&self.1))
	}
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Meta {
	pub name: String,
	pub description: String,
	pub contributors: Vec<(i32, String, ContributorTexture)>,
	pub dependencies: Vec<(i32, String, String, DependencyTexture)>,
	pub nsfw: bool,
	pub previews: Vec<String>,
}