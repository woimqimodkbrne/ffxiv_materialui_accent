use std::fs::File;
use noumenon::formats::{game::{tex::Tex, mtrl::Mtrl}, external::{dds::Dds, png::Png}};
use crate::IRONWORKS;

ffi!(fn viewer_tex_load(path: &str) -> *mut Tex {
	Box::into_raw(Box::new(IRONWORKS.file::<Tex>(path)?))
});

ffi!(fn viewer_tex_save(tex: *mut Tex, path: &str, format: &str) {
	// let tex = *unsafe { Box::from_raw(tex) };
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
	Box::into_raw(Box::new(IRONWORKS.file::<Mtrl>(path)?))
});