use std::fs::File;
use noumenon::formats::{game::{tex::Tex, mtrl::Mtrl}, external::{dds::Dds, png::Png}};
use crate::IRONWORKS;

ffi!(fn viewer_tex_load(path: &str) -> *mut Tex {
	Box::into_raw(Box::new(match IRONWORKS.file::<Tex>(path) {
		Ok(v) => v,
		Err(_) => Tex::read(&mut File::open(path)?)
	}))
});

// Nearest neighbour scaling
ffi!(fn viewer_tex_preview(tex: *mut Tex, width: u16, height: u16) -> Vec<u8> {
	let tex = unsafe { &*tex };
	
	let bx = width as f32 / tex.header.width as f32;
	let by = height as f32 / tex.header.height as f32;
	let mut data = Vec::with_capacity(width as usize * height as usize * 4);
	for y in 0..height as usize {
		for x in 0..width as usize {
			let i = (y as f32 / by) as usize * 4 * tex.header.width as usize + (x as f32 / bx) as usize * 4;
			data.push(tex.data[i    ]);
			data.push(tex.data[i + 1]);
			data.push(tex.data[i + 2]);
			data.push(tex.data[i + 3]);
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