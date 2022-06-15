#![allow(improper_ctypes_definitions)]
#![feature(panic_backtrace_config)]
#![feature(seek_stream_len)]
#![feature(let_chains)]

use std::panic::BacktraceStyle;
use ironworks::{Ironworks, sqpack::SqPack, ffxiv};
use serde::Serialize;
use reqwest::blocking as req;

#[macro_use]
extern crate lazy_static;

pub const SERVER: &'static str = "http://localhost:8080";
lazy_static! {
	pub static ref CLIENT: req::Client = req::Client::new();
	
	pub static ref IRONWORKS: ironworks::Ironworks = Ironworks::new()
		.resource(SqPack::new(ffxiv::FsResource::search().unwrap()));
}

static mut LOG: fn(u8, String) = |_, _| {};

#[macro_export]
macro_rules! log {
	(ftl, $($e:tt)*) => { unsafe { crate::LOG(255, format!($($e)*)) } };
	(log, $($e:tt)*) => { unsafe { crate::LOG(0,   format!($($e)*)) } };
}

#[no_mangle]
extern fn initialize(log: fn(u8, String)) {
	unsafe { LOG = log }
	
	std::panic::set_backtrace_style(BacktraceStyle::Full);
	std::panic::set_hook(Box::new(|info| {
		log!(ftl, "{}", info);
	}));
	
	// use noumenon::formats::{game::tex::Tex, external::dds::Dds};
	// let mut fr = std::fs::File::open("C:/ffxiv/aetherment/UI Test/files/overlay.dds").unwrap();
	// let mut fw = std::fs::File::create("C:/ffxiv/aetherment/UI Test/files/overlay.tex").unwrap();
	// <Tex as Dds>::read(&mut fr).write(&mut fw);
}

#[macro_export]
macro_rules! ffi {
	// types that we can just send across as is
	// yes, this dumb but it works
	(fn $name:ident ($($param_name:ident: $param_type:ty),*) $inner:block) => {ffi!($name>$($param_name, $param_type)*>()>$inner);};
	(fn $name:ident ($($param_name:ident: $param_type:ty),*) -> i8 $inner:block) => {ffi!($name>$($param_name, $param_type)*>i8>$inner);};
	(fn $name:ident ($($param_name:ident: $param_type:ty),*) -> u8 $inner:block) => {ffi!($name>$($param_name, $param_type)*>u8>$inner);};
	(fn $name:ident ($($param_name:ident: $param_type:ty),*) -> i16 $inner:block) => {ffi!($name>$($param_name, $param_type)*>i16>$inner);};
	(fn $name:ident ($($param_name:ident: $param_type:ty),*) -> u16 $inner:block) => {ffi!($name>$($param_name, $param_type)*>u16>$inner);};
	(fn $name:ident ($($param_name:ident: $param_type:ty),*) -> i32 $inner:block) => {ffi!($name>$($param_name, $param_type)*>i32>$inner);};
	(fn $name:ident ($($param_name:ident: $param_type:ty),*) -> u32 $inner:block) => {ffi!($name>$($param_name, $param_type)*>u32>$inner);};
	(fn $name:ident ($($param_name:ident: $param_type:ty),*) -> i64 $inner:block) => {ffi!($name>$($param_name, $param_type)*>i64>$inner);};
	(fn $name:ident ($($param_name:ident: $param_type:ty),*) -> u64 $inner:block) => {ffi!($name>$($param_name, $param_type)*>u64>$inner);};
	
	($name:ident>$($param_name:ident, $param_type:ty)*>$return_type:ty>$inner:block) => {
		#[no_mangle]
		extern fn $name($($param_name: $param_type,)*) -> $return_type $inner
	};
	
	// types that we box
	(fn $name:ident ($($param_name:ident: $param_type:ty),*) -> $return_type:ty $inner:block) => {
		#[no_mangle]
		extern fn $name($($param_name: $param_type, )*) -> *mut $return_type {
			Box::into_raw(Box::new($inner))
		}
	};
}

pub fn serialize_json(json: serde_json::Value) -> String {
	let buf = Vec::new();
	let formatter = serde_json::ser::PrettyFormatter::with_indent(b"\t");
	let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
	json.serialize(&mut ser).unwrap();
	String::from_utf8(ser.into_inner()).unwrap()
}

mod server;
mod moddev {
	mod import;
	mod index;
	mod upload;
}
mod downloader {
	mod download;
	mod penumbra;
}

ffi!(fn free_object(s: *mut ()) {
	unsafe { Box::from_raw(s); }
});

ffi!(fn cool_test(s: &str) -> String {
	format!("cool str! {}, this was send from rust", s)
});

ffi!(fn cool_test2(s: &[&str]) -> Vec<String> {
	s.into_iter().map(|e| (e.parse::<i32>().unwrap() * -2).to_string()).collect()
});

ffi!(fn panic(s: &str) {
	panic!("{}", s);
});