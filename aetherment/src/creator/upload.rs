use std::{path::Path, fs::File, io::{Read, Seek}};
use binrw::BinReaderExt;
use serde::Deserialize;
use serde_json::json;
use crate::{SERVER, CLIENT};
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
		match CLIENT.post(format!("{SERVER}/api/mod/new"))
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
	const BUF_SIZE: usize = CHUNK_SIZE + 100_000_000;
	while let Ok(pos) = modpack.stream_position() && pos < modpack_len {
		// we construct our own multipart to allow custom part headers
		let mut multipart = Vec::with_capacity(BUF_SIZE);
		let boundary_raw = format!("{:032x}-{:032x}", rand::random::<u128>(), rand::random::<u128>());
		let boundary = format!("--{boundary_raw}");
		
		if pos == 0 {
			write_meta(&path, &mut multipart, &boundary, &meta, None);
		}
		
		// modpack chunk
		let chunk_size = CHUNK_SIZE.min(modpack_len as usize);
		let mut buf = vec![0u8; chunk_size];
		modpack.read_exact(&mut buf).unwrap();
		
		multipart.extend_from_slice(boundary.as_bytes());
		multipart.extend_from_slice(&format!("\r\nContent-Disposition: form-data; name=modpack; offset={pos}; total_size={modpack_len}\r\nContent-Length: {}\r\n\r\n", chunk_size).as_bytes());
		multipart.extend(buf);
		multipart.extend_from_slice("\r\n".as_bytes());
		
		// patchnotes
		if modpack.stream_position().unwrap() == modpack_len {
			multipart.extend_from_slice(boundary.as_bytes());
			multipart.extend_from_slice(&format!("\r\nContent-Disposition: form-data; name=patchnotes\r\nContent-Length: {}\r\n\r\n", patchnotes.len()).as_bytes());
			multipart.extend_from_slice(patchnotes.as_bytes());
			multipart.extend_from_slice("\r\n".as_bytes());
		}
		
		// boundary ending
		multipart.extend_from_slice(boundary.as_bytes());
		multipart.extend_from_slice("--".as_bytes());
		
		match CLIENT.post(format!("{SERVER}/api/mod/{mod_id}/manage"))
			.header("Authorization", auth)
			.header("Content-Type", format!("multipart/form-data; boundary={boundary_raw}"))
			.header("Content-Length", multipart.len())
			.body(multipart)
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

pub fn update_mod_meta<P>(auth: &str, path: P, allowed_previews: Vec<String>) -> Result<(), UploadError> where
P: AsRef<Path> {
	let path = path.as_ref();
	if !path.exists() {return Err(UploadError::InvalidMod)}
	
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
	
	let mod_id = File::open(path.join("aeth"))
		.map_err(|_| UploadError::InvalidMod)?
		.read_le::<i32>()
		.map_err(|_| UploadError::InvalidMod)?;
	
	// multipart
	let mut multipart = Vec::new();
	let boundary_raw = format!("{:032x}-{:032x}", rand::random::<u128>(), rand::random::<u128>());
	let boundary = format!("--{boundary_raw}");
	
	write_meta(&path, &mut multipart, &boundary, &meta, Some(allowed_previews));
	
	// boundary ending
	multipart.extend_from_slice(boundary.as_bytes());
	multipart.extend_from_slice("--".as_bytes());
	
	CLIENT.post(format!("{SERVER}/api/mod/{mod_id}/manage"))
		.header("Authorization", auth)
		.header("Content-Type", format!("multipart/form-data; boundary={boundary_raw}"))
		.header("Content-Length", multipart.len())
		.body(multipart)
		.send()
		.map_err(|e| UploadError::ServerResponse(e.to_string()))?;
	
	Ok(())
}

fn write_meta(path: &Path, multipart: &mut Vec<u8>, boundary: &str, meta: &Meta, allowed_previews: Option<Vec<String>>) {
	// meta
	let content = json!({
		"name": meta.name,
		"description": meta.description,
		"contributors": meta.contributors.iter().map(|(id, _, _)| *id).collect::<Vec<i32>>(),
		"dependencies": meta.dependencies.iter().map(|(id, _, _, _)| *id).collect::<Vec<i32>>(),
		"previews": meta.previews,
	}).to_string();
	multipart.extend_from_slice(boundary.as_bytes());
	multipart.extend_from_slice(&format!("\r\nContent-Disposition: form-data; name=meta\r\nContent-Length: {}\r\n\r\n", content.len()).as_bytes());
	multipart.extend_from_slice(content.as_bytes());
	multipart.extend_from_slice("\r\n".as_bytes());
	
	// previews
	// TODO: allow selecting if a preview should be the new thumbnail
	for preview_id in &meta.previews {
		if (allowed_previews.is_none() || allowed_previews.as_ref().unwrap().contains(&preview_id)) && let Ok(mut preview) = File::open(path.join("previews").join(preview_id)) {
			log!("uploading preview {preview_id}");
			let mut preview_buf = Vec::new();
			preview.read_to_end(&mut preview_buf).unwrap();
			
			multipart.extend_from_slice(boundary.as_bytes());
			let preview_id = preview_id.replace("\\", "\\\\").replace("\"", "\\\""); // no sneaky attempts here
			multipart.extend_from_slice(&format!("\r\nContent-Disposition: form-data; name=preview; id=\"{preview_id}\"; thumbnail=0\r\nContent-Length: {}\r\n\r\n", preview_buf.len()).as_bytes());
			multipart.extend(preview_buf);
			multipart.extend_from_slice("\r\n".as_bytes());
		}
	}
}