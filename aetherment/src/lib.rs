#![allow(improper_ctypes_definitions)]
#![feature(panic_backtrace_config)]
#![feature(backtrace)]
#![feature(seek_stream_len)]
#![feature(let_chains)]

use std::{panic::BacktraceStyle, collections::HashMap};
use ironworks::{Ironworks, sqpack::SqPack, ffxiv};
use serde::Serialize;
use reqwest::blocking as req;

#[macro_use]
extern crate lazy_static;

pub const SERVER: &'static str = "http://localhost:8080";
lazy_static! {
	pub static ref CLIENT: req::Client = req::Client::new();
	
	pub static ref IRONWORKS: ironworks::Ironworks = {
		let mut i = Ironworks::new();
		i.add_resource(SqPack::new(ffxiv::FsResource::search().unwrap()));
		i
	};
}

static mut LOG: fn(u8, String) = |_, _| {};

#[macro_export]
macro_rules! log {
	(ftl, $($e:tt)*) => { unsafe { crate::LOG(255, format!($($e)*)) } };
	(log, $($e:tt)*) => { unsafe { crate::LOG(0,   format!($($e)*)) } };
	(err, $($e:tt)*) => { unsafe { crate::LOG(1,   format!($($e)*)) } };
}

mod gui {
	pub mod imgui;
}

pub fn serialize_json(json: serde_json::Value) -> String {
	let buf = Vec::new();
	let formatter = serde_json::ser::PrettyFormatter::with_indent(b"\t");
	let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
	json.serialize(&mut ser).unwrap();
	String::from_utf8(ser.into_inner()).unwrap()
}

#[no_mangle]
extern fn initialize(log: fn(u8, String)) {
	unsafe { LOG = log }
	
	std::panic::set_backtrace_style(BacktraceStyle::Short);
	std::panic::set_hook(Box::new(|info| {
		// log!(ftl, "{}", info);
		log!(err, "{}", info);
	}));
}

lazy_static! {
	pub static ref BOXES: HashMap<u64, std::any::TypeId> = HashMap::new();
}

#[no_mangle]
extern fn draw() {
	// Somehow this works without setting the allocators and stuff, idk why but i'll take it
	use gui::imgui;
	
	let mut open = true;
	imgui::set_next_window_size([200.0, 200.0], imgui::ImGuiCond::Always);
	imgui::begin("aetherment", &mut open, imgui::ImGuiWindowFlags::None);
	imgui::text("hello there");
	imgui::end();
}

#[repr(packed)]
#[allow(dead_code)]
struct FfiResult<T> {
	pub error: bool,
	pub obj: T,
}

#[macro_export]
macro_rules! ffi {
	(fn $name:ident ($($param_name:ident: $param_type:ty),*) $inner:block) => {
		ffi!(fn $name ($($param_name: $param_type),*) -> () $inner);
	};
	
	(fn $name:ident ($($param_name:ident: $param_type:ty),*) -> $return_type:ty $inner:block) => {
		#[no_mangle]
		extern fn $name($($param_name: $param_type,)*) -> *const () {
			match std::panic::catch_unwind(|| -> anyhow::Result<$return_type> {Ok($inner)}) {
				Ok(v) => match v {
					Ok(v) => Box::into_raw(Box::new(crate::FfiResult{error: false, obj: v})) as *const (),
					// This error sucks and theres no traceback, cant manage to add it somehow
					Err(e) => Box::into_raw(Box::new(crate::FfiResult{error: true, obj: format!("{:?}", e)})) as *const (),
				},
				Err(_) => Box::into_raw(Box::new(crate::FfiResult{error: true, obj: "I give up, look in your console it's there".to_string()})) as *const (),
			}
		}
	};
}

// this doesn't work, im stupid
// TODO: fix it or fuck c# and go fully rust (figure out how to get device here)
#[no_mangle]
fn free_object(s: *mut ()) {
	unsafe { Box::from_raw(s); }
}

mod server;
mod moddev {
	mod import;
	mod index;
	mod upload;
}
mod downloader {
	pub mod download;
	pub mod penumbra;
}
mod explorer {
	mod tools;
	mod viewer;
	mod datas;
}