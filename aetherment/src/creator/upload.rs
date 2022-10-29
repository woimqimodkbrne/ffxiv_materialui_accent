use std::{path::Path, fs::File, io::{Read, Seek}};
use serde::Deserialize;
use serde_json::json;
use reqwest::blocking::multipart::{Form, Part};
use crate::SERVER;
use super::meta::Meta;

pub const MAX_PATCH_DESC_LEN: usize = 500;

#[derive(Debug)]
pub enum UploadError {
	InvalidMod,
	InvalidModPack,
	ServerResponse(String),
	ServerResponsePartial(i32, String),
}

pub fn upload_mod<P, P2>(auth: &str, path: P, modpack: P2, mod_id: Option<i32>, patchnotes: &str) -> Result<i32, UploadError> where
P: AsRef<Path>,
P2: AsRef<Path> {
	if patchnotes.len() > MAX_PATCH_DESC_LEN {return Err(UploadError::InvalidMod)}
	
	let path = path.as_ref();
	let modpack_path = modpack.as_ref();
	
	if !path.exists() {return Err(UploadError::InvalidMod)}
	if !modpack_path.exists() {return Err(UploadError::InvalidMod)}
	let meta: Meta = match File::open(path.join("meta.json")) {
		Ok(mut file) => {
			let mut buf = Vec::new();
			file.read_to_end(&mut buf).map_err(|_| UploadError::InvalidMod)?;
			match serde_json::from_slice(&buf) {
				Ok(v) => v,
				Err(_) => return Err(UploadError::InvalidMod),
			}
		},
		Err(_) => return Err(UploadError::InvalidMod),
	};
	
	let mut modpack = File::open(modpack_path).map_err(|_| UploadError::InvalidMod)?;
	let modpack_len = modpack.stream_len().map_err(|_| UploadError::InvalidMod)?;
	
	let client = reqwest::blocking::Client::new();
	
	let mod_id = if let Some(mod_id) = mod_id {
		mod_id
	} else {
		// Create a new mod draft
		#[derive(Deserialize)]
		#[serde(untagged)]
		enum Resp {
			Error{error: String},
			Success{id: i32},
		}
		match client.post(format!("{SERVER}/api/mod/new"))
			.header("Authorization", auth)
			.send()
			.map_err(|e| UploadError::ServerResponse(e.to_string()))?
			.json::<Resp>()
			.map_err(|e| UploadError::ServerResponse(e.to_string()))? {
			Resp::Error{error} => return Err(UploadError::ServerResponse(error)),
			Resp::Success{id} => id,
		}
	};
	
	// Upload the mod
	#[derive(Deserialize)]
	#[serde(untagged)]
	enum Resp2 {
		Error{error: String},
		#[allow(dead_code)] Success{success: bool},
	}
	const CHUNK_SIZE: usize = 80_000_000;
	while let Ok(p) = modpack.stream_position() && p < modpack_len {
		let pos = p;
		let mut buf = vec![0u8; CHUNK_SIZE];
		modpack.read_exact(&mut buf[..(CHUNK_SIZE.min(modpack_len as usize))]).unwrap();
		
		let mut parts = Form::new();
		if pos == 0 {
			parts = parts.part(json!({"type": "Meta"}).to_string(), Part::text(json!({
				"name": meta.name,
				"description": meta.description,
				"contributors": meta.contributors.iter().map(|(id, _, _)| *id).collect::<Vec<i32>>(),
				"dependencies": meta.dependencies.iter().map(|(id, _, _, _)| *id).collect::<Vec<i32>>(),
				"previews": meta.previews,
			}).to_string()));
			
			for preview_id in &meta.previews {
				if let Ok(mut preview) = File::open(path.join("previews").join(preview_id)) {
					let mut preview_buf = Vec::new();
					preview.read_to_end(&mut preview_buf).unwrap();
					parts = parts.part(json!({"type": "Preview", "id": preview_id, "thumbnail": true}).to_string(), Part::bytes(preview_buf));
				}
			}
		}
		
		parts = parts.part(json!({"type": "ModPack", "offset": pos, "total_size": modpack_len, "patchnotes": patchnotes}).to_string(), Part::bytes(buf));
		
		match client.post(format!("{SERVER}/api/mod/{mod_id}/manage"))
			.header("Authorization", auth)
			.multipart(parts)
			.send()
			.map_err(|e| UploadError::ServerResponsePartial(mod_id, e.to_string()))?
			.json::<Resp2>()
			.map_err(|e| UploadError::ServerResponsePartial(mod_id, e.to_string()))? {
			Resp2::Error{error} => return Err(UploadError::ServerResponsePartial(mod_id, error)),
			_ => log!("succesfully uploaded part"),
		}
	}
	
	Ok(mod_id)
}