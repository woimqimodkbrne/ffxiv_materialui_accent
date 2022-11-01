#![allow(improper_ctypes_definitions)]
// #![feature(panic_backtrace_config)]
#![feature(seek_stream_len)]
#![feature(let_chains)]
#![feature(generic_associated_types)]

use std::{path::{PathBuf, Path}, net::TcpListener};
use ironworks::{Ironworks, sqpack::SqPack, ffxiv};
use serde::Serialize;
use reqwest::blocking as req;

extern crate imgui;

// ---------------------------------------- //

pub fn serialize_json(json: serde_json::Value) -> String {
	let buf = Vec::new();
	let formatter = serde_json::ser::PrettyFormatter::with_indent(b"\t");
	let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
	json.serialize(&mut ser).unwrap();
	String::from_utf8(ser.into_inner()).unwrap()
}

// pub fn hash_str(hash: blake3::Hash) -> String {
// 	base64::encode_config(hash.as_bytes(), base64::URL_SAFE_NO_PAD)
// }

pub fn hash_str(hash: &[u8; 32]) -> String {
	base64::encode_config(hash, base64::URL_SAFE_NO_PAD)
}

pub fn file_picker<S, F>(title: S, setup: F, path: &mut String, config: &mut config::Config) -> bool where
S: AsRef<str>,
F: FnOnce() -> imgui::aeth::FileDialog {
	let r = imgui::aeth::file_picker(title, setup, path);
	if r && let Some(parent) = Path::new(path).parent() {
		config.explorer_path = parent.to_string_lossy().to_string();
		_ = config.save_forced();
	}
	
	r
}

// TODO: move to seperate file and check for outdated cache files to delete (x days not accessed or smth)
pub fn get_resource(path: &str) -> Vec<u8> {
	use std::{fs::File, io::{Read, Write}};
	
	let hash = hash_str(blake3::hash(path.as_bytes()).as_bytes());
	let cache_dir = dirs::cache_dir().unwrap().join("Aetherment").join("cache");
	let cache_path = cache_dir.join(&hash);
	if let Ok(mut f) = File::open(&cache_path) {
		let mut r = Vec::new();
		f.read_to_end(&mut r).unwrap();
		return r;
	}
	
	let r = CLIENT.get(format!("{SERVERCDN}{path}"))
		.send()
		.unwrap()
		.bytes()
		.unwrap()
		.to_vec();
	
	std::fs::create_dir_all(cache_dir).unwrap();
	let mut f = File::create(cache_path).unwrap();
	f.write_all(&r).unwrap();
	r
}

static mut LOG: fn(u8, String) = |_, _| {};
#[macro_export]
macro_rules! log {
	(ftl, $($e:tt)*) => {unsafe{crate::LOG(255, format!($($e)*))}};
	(log, $($e:tt)*) => {unsafe{crate::LOG(0, format!($($e)*))}};
	(err, $($e:tt)*) => {unsafe{crate::LOG(1,format!($($e)*))}};
	($($e:tt)*) => {unsafe{crate::LOG(0, format!($($e)*))}};
}

// ---------------------------------------- //

#[macro_use]
extern crate lazy_static;

pub const SERVER: &'static str = "http://localhost:80";
pub const SERVERCDN: &'static str = "https://cdn.aetherment.com";
lazy_static! {
	pub static ref CLIENT: req::Client = req::Client::new();
	pub static ref GAME: ironworks::Ironworks = Ironworks::new()
		// .with_resource(SqPack::new(ffxiv::FsResource::search().unwrap()));
		.with_resource(SqPack::new(ffxiv::FsResource::at(std::env::current_exe().unwrap().parent().unwrap().parent().unwrap())));
}

// ---------------------------------------- //

pub mod api {
	pub mod penumbra;
}
pub mod server {
	pub mod user;
}
pub mod creator {
	pub mod meta;
	pub mod modpack;
	pub mod upload;
	pub mod tags;
	pub mod import {
		pub mod penumbra;
		pub mod v1;
	}
}
pub mod config;
pub mod apply;
pub mod gui {
	pub use imgui::aeth;
	pub mod window {
		pub mod aetherment;
	}
}

// ---------------------------------------- //

pub struct State {
	data: Data,
	
	win_aetherment: gui::window::aetherment::Window,
	
	server: TcpListener,
}

pub struct Data {
	binary_path: PathBuf,
	#[allow(dead_code)] config_path: PathBuf,
	config: config::Config,
	user: Option<server::user::User>,
}

#[repr(packed)]
pub struct Initializers<'a> {
	binary_path: &'a str,
	config_path: &'a str,
	log: fn(u8, String),
	create_texture: fn(gui::aeth::TextureOptions) -> usize,
	create_texture_data: fn(gui::aeth::TextureOptions, Vec<u8>) -> usize,
	drop_texture: fn(usize),
	pin_texture: fn(usize) -> *mut u8,
	unpin_texture: fn(usize),
	fa5: *mut imgui::sys::ImFont,
	penumbra_redraw: fn(),
	penumbra_redraw_self: fn(),
	penumbra_add_mod: fn(String, String, String, i32) -> u8,
	penumbra_remove_mod: fn(String, i32) -> u8,
}

#[no_mangle]
pub extern fn initialize(init: Initializers) -> *mut State {
	use gui::aeth::texture;
	
	unsafe {
		LOG = init.log;
		texture::CREATE = init.create_texture;
		texture::CREATEDATA = init.create_texture_data;
		texture::DROP = init.drop_texture;
		texture::PIN = init.pin_texture;
		texture::UNPIN = init.unpin_texture;
		gui::aeth::FA5 = &mut *init.fa5;
		
		api::penumbra::REDRAW = init.penumbra_redraw;
		api::penumbra::REDRAWSELF = init.penumbra_redraw_self;
		api::penumbra::ADDMOD = init.penumbra_add_mod;
		api::penumbra::REMOVEMOD = init.penumbra_remove_mod;
	}
	
	// std::panic::set_backtrace_style(BacktraceStyle::Short);
	std::panic::set_hook(Box::new(|info| {
		// log!(ftl, "{}", info);
		log!(err, "{}", info);
	}));
	
	let config_path: PathBuf = init.config_path.into();
	let mut data = Data {
		config: config::Config::load(config_path.join("config.json")),
		binary_path: init.binary_path.into(),
		config_path: config_path,
		user: server::user::User::load(),
	};
	
	let server = TcpListener::bind("127.0.0.1:6577").expect("Can't' bind to port 6577");
	server.set_nonblocking(true).expect("Can't set server to non-blocking");
	
	Box::into_raw(Box::new(State {
		win_aetherment: gui::window::aetherment::Window::new(&mut data),
		data,
		server,
	}))
}

#[no_mangle]
pub extern fn destroy(state: *mut State) {
	log!("destroy");
	let _state = unsafe{Box::from_raw(state)}.server;
}

#[no_mangle]
pub extern fn update_resources(_state: *mut State, fa5: *mut imgui::sys::ImFont) {
	// let state = unsafe{&mut *state};
	unsafe{gui::aeth::FA5 = &mut *fa5}
}

#[no_mangle]
pub extern fn draw(state: *mut State) {
	let state = state as usize;
	
	std::panic::catch_unwind(|| {
		let state = unsafe{&mut *(state as *mut State)};
		if state.win_aetherment.visible {
			imgui::set_next_window_size([1100.0, 600.0], imgui::Cond::FirstUseEver);
			imgui::begin("Aetherment", Some(&mut state.win_aetherment.visible), imgui::WindowFlags::None);
			if let Err(e) = state.win_aetherment.draw(&mut state.data) {log!(err, "{:?}", e)}
			imgui::end();
		}
		
		gui::aeth::draw_error();
	}).ok();
	
	// run server, most definitely shouldnt do this in draw but oh well
	let mut state = unsafe{&mut *(state as *mut State)};
	handle_server(&mut state);
}

#[no_mangle]
pub extern fn command(state: *mut State, args: &str) {
	let state = unsafe{&mut *state};
	
	match args {
		_ => state.win_aetherment.visible = !state.win_aetherment.visible,
	}
}

fn handle_server(state: &mut State) {
	use std::io::{Read, Write};
	#[derive(serde::Deserialize)]
	enum Msg {
		Auth(String),
	}
	
	if let Ok((mut stream, _addr)) = state.server.accept() {
		let mut buf = [0u8; 4096];
		if let Ok(read_count) = stream.read(&mut buf) && let Ok(req) = std::str::from_utf8(&buf[..read_count]) {
			// log!("msg received: ({req})");
			
			// im sure this is fine, we dont care about anything else
			if let Some(p) = req.find("\r\n\r\n") && let Ok(msg) = serde_json::from_slice(req[p + 4..].as_bytes()) {
				match msg {
					Msg::Auth(token) => {
						log!("auth attempt");
						state.data.user = server::user::User::new(token);
						if let Some(user) = &state.data.user {
							_ = user.store();
						}
					}
				}
			}
			
			_ = stream.write(&[]);
		}
	}
}