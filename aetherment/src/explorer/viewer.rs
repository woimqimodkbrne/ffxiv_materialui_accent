use std::{fs::File, collections::HashMap, io::{Seek, Read}};
use anyhow::Context;
use noumenon::formats::{game::{tex::Tex, mtrl::Mtrl}, external::{dds::Dds, png::Png}};
use crate::{IRONWORKS, downloader::penumbra::{resolve_layer, ConfSettings}};

ffi!(fn viewer_tex_load(paths: &[&[&str]], settings: &str) -> *mut Tex {
	let load_file = |path: &str| -> Option<Vec<u8>> {
		// TODO: allow reading from mods with lower priority
		match File::open(path) {
			Ok(mut f) => {
				let mut buf = Vec::with_capacity(f.stream_len().unwrap() as usize);
				f.read_to_end(&mut buf).unwrap();
				Some(buf)
			},
			Err(_) => IRONWORKS.file::<Vec<u8>>(path).ok(),
		}
	};
	
	// cba figuring out how to ffi enums, dont @ me
	let settings: HashMap<String, ConfSettings> = serde_json::from_str(&settings)?;
	
	let mut layers = paths.iter().map(|l| l.iter().map(|p| if *p == "" {None} else {Some(p)}).collect::<Vec<Option<&&str>>>());
	let mut result = resolve_layer(&layers.next().context("There were no layers")?, &settings, load_file).context("Failed resolving layer")?;
	while let Some(layer) = layers.next() {
		resolve_layer(&layer, &settings, load_file).context("Failed resolving layer")?.overlay_onto(&mut result);
	}
	
	Box::into_raw(Box::new(result))
});

// Nearest neighbour scaling
ffi!(fn viewer_tex_preview(tex: *mut Tex, target_width: u16, target_height: u16, miplevel: u16, depth: u16) -> Vec<u8> {
	let tex = unsafe { &*tex };
	
	let (w, h, slice) = tex.slice(miplevel, depth);
	let bx = target_width as f32 / w as f32;
	let by = target_height as f32 / h as f32;
	let mut data = Vec::with_capacity(target_width as usize * target_height as usize * 4);
	for y in 0..target_height as usize {
		for x in 0..target_width as usize {
			let i = (y as f32 / by).floor() as usize * 4 * w as usize + (x as f32 / bx).floor() as usize * 4;
			data.push(slice[i    ]);
			data.push(slice[i + 1]);
			data.push(slice[i + 2]);
			data.push(slice[i + 3]);
		}
	}
	
	data
});

ffi!(fn viewer_tex_save(tex: *mut Tex, path: &str, format: &str) {
	let tex = unsafe { &*tex };
	
	let mut file = File::create(path)?;
	match format { // TODO: probably use enum instead
		"tex" => Tex::write(&tex, &mut file),
		"dds" => <Tex as Dds>::write(&tex, &mut file),
		"png" => <Tex as Png>::write(&tex, &mut file),
		_ => {},
	}
});

ffi!(fn viewer_mtrl_load(path: &str) -> *mut Mtrl {
	Box::into_raw(Box::new(match IRONWORKS.file::<Mtrl>(path) {
		Ok(v) => v,
		Err(_) => Mtrl::read(&mut File::open(path)?)
	}))
});