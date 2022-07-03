#![allow(improper_ctypes_definitions)]
#![feature(panic_backtrace_config)]
#![feature(backtrace)]
#![feature(seek_stream_len)]
#![feature(let_chains)]

use std::{panic::BacktraceStyle, path::PathBuf};
use ironworks::{Ironworks, sqpack::SqPack, ffxiv};
use serde::Serialize;
use reqwest::blocking as req;

// ---------------------------------------- //

pub fn serialize_json(json: serde_json::Value) -> String {
	let buf = Vec::new();
	let formatter = serde_json::ser::PrettyFormatter::with_indent(b"\t");
	let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
	json.serialize(&mut ser).unwrap();
	String::from_utf8(ser.into_inner()).unwrap()
}

static mut LOG: fn(u8, String) = |_, _| {};
#[macro_export]
macro_rules! log {
	(ftl, $($e:tt)*) => { unsafe { crate::LOG(255, format!($($e)*)) } };
	(log, $($e:tt)*) => { unsafe { crate::LOG(0,   format!($($e)*)) } };
	(err, $($e:tt)*) => { unsafe { crate::LOG(1,   format!($($e)*)) } };
	($($e:tt)*) => { unsafe { crate::LOG(0,   format!($($e)*)) } };
}

// ---------------------------------------- //

#[macro_use]
extern crate lazy_static;

pub const SERVER: &'static str = "http://localhost:8080";
lazy_static! {
	pub static ref CLIENT: req::Client = req::Client::new();
	pub static ref GAME: ironworks::Ironworks = Ironworks::new()
		.with_resource(SqPack::new(ffxiv::FsResource::search().unwrap()));
}

// ---------------------------------------- //

pub mod server;
pub mod config;
pub mod apply;
pub mod gui {
	pub mod imgui;
	pub mod aeth;
	pub mod window {
		pub mod aetherment;
	}
}

// ---------------------------------------- //

struct State {
	data: Data,
	
	win_aetherment: gui::window::aetherment::Window,
}

pub struct Data {
	binary_path: PathBuf,
	config_path: PathBuf,
	
	config: config::Config,
}

#[repr(packed)]
struct Initializers<'a> {
	binary_path: &'a str,
	config_path: &'a str,
	log: fn(u8, String),
	create_texture: fn(gui::aeth::TextureOptions) -> usize,
	create_texture_data: fn(gui::aeth::TextureOptions, Vec<u8>) -> usize,
	drop_texture: fn(usize),
	pin_texture: fn(usize) -> *mut u8,
	unpin_texture: fn(usize),
}

#[no_mangle]
extern fn initialize(init: Initializers) -> *mut State {
	use gui::aeth::texture;
	
	unsafe {
		LOG = init.log;
		texture::CREATE = init.create_texture;
		texture::CREATEDATA = init.create_texture_data;
		texture::DROP = init.drop_texture;
		texture::PIN = init.pin_texture;
		texture::UNPIN = init.unpin_texture;
	}
	
	std::panic::set_backtrace_style(BacktraceStyle::Short);
	std::panic::set_hook(Box::new(|info| {
		// log!(ftl, "{}", info);
		log!(err, "{}", info);
	}));
	
	let config_path: PathBuf = init.config_path.into();
	
	let mut data = Data {
		config: config::Config::load(config_path.join("config.json")),
		
		binary_path: init.binary_path.into(),
		config_path: config_path,
	};
	
	Box::into_raw(Box::new(State {
		win_aetherment: gui::window::aetherment::Window::new(&mut data),
		data: data,
	}))
}

#[no_mangle]
extern fn destroy(state: *mut State) {
	let _state = unsafe{&mut *state};
}

#[no_mangle]
extern fn draw(state: *mut State) {
	let state = unsafe{&mut *state};
	
	use gui::imgui;
	
	if state.win_aetherment.visible {
		imgui::set_next_window_size([1100.0, 600.0], imgui::Cond::FirstUseEver);
		imgui::begin("Aetherment", &mut state.win_aetherment.visible, imgui::WindowFlags::None);
		if let Err(e) = state.win_aetherment.draw(&mut state.data) {log!(err, "{:?}", e);}
		imgui::end();
	}
}

#[no_mangle]
extern fn command(state: *mut State, args: &str) {
	let state = unsafe{&mut *state};
	
	match args {
		_ => state.win_aetherment.visible = !state.win_aetherment.visible,
	}
}